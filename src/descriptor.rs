// Interlude: Descriptor and its layout in shader

use std::borrow::Cow;
use interlude_vk_defs::*;
use interlude_vk_funport::*;
use std::ops::{Range, Deref};
use device::Device;
use shading::ShaderStageSet;
use std::rc::Rc;
use std::ptr::null;
use std::mem::{transmute, uninitialized as reserved};
use {ImageView, Sampler, BufferResource, EngineResult, GraphicsInterface};
use subsystem_layer::{NativeHandleProvider, NativeResultValueHandler};
use shading::shader_stage_flags;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Descriptor
{
	Uniform(u32, ShaderStageSet), Storage(u32, ShaderStageSet),
	CombinedSampler(u32, ShaderStageSet), InputAttachment(u32, ShaderStageSet)
}
impl Descriptor
{
	fn deconstruct(&self) -> (VkDescriptorType, u32, VkShaderStageFlags)
	{
		use Descriptor::*;
		match self
		{
			&Uniform(c, s) => (VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, c, shader_stage_flags(s)),
			&Storage(c, s) => (VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, c, shader_stage_flags(s)),
			&CombinedSampler(c, s) => (VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, c, shader_stage_flags(s)),
			&InputAttachment(c, s) => (VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT, c, shader_stage_flags(s))
		}
	}
	#[allow(non_snake_case)]
	fn into_binding(&self, index: u32) -> VkDescriptorSetLayoutBinding
	{
		let (descriptorType, descriptorCount, stageFlags) = self.deconstruct();
		VkDescriptorSetLayoutBinding
		{
			binding: index, descriptorType, descriptorCount, stageFlags, pImmutableSamplers: null()
		}
	}
}

use subsystem_layer::NativeDescriptorSetLayout;
pub struct DescriptorSetLayout(Rc<NativeDescriptorSetLayout>, Vec<Descriptor>);
impl DescriptorSetLayout
{
	pub fn new<'a>(engine: &GraphicsInterface, bindings: Cow<'a, [Descriptor]>) -> EngineResult<Self>
	{
		let native_bindings = bindings.iter().enumerate().map(|(i, x)| x.into_binding(i as u32)).collect::<Vec<_>>();
		let mut dsl = unsafe { reserved() };
		unsafe { vkCreateDescriptorSetLayout(engine.device().native(), &VkDescriptorSetLayoutCreateInfo
		{
			bindingCount: native_bindings.len() as _, pBindings: native_bindings.as_ptr(), .. Default::default()
		}, null(), &mut dsl) }.make_result_with(|| DescriptorSetLayout(Rc::new(NativeDescriptorSetLayout(dsl, engine.device().clone())), bindings.into_owned()))
	}
	pub fn descriptors(&self) -> &[Descriptor] { &self.1 }
}
pub fn addref_dsl(dsl: &DescriptorSetLayout) -> Rc<NativeDescriptorSetLayout> { dsl.0.clone() }
impl NativeHandleProvider for DescriptorSetLayout { type NativeT = VkDescriptorSetLayout; fn native(&self) -> Self::NativeT { self.0.native() } }

pub struct DescriptorSets(VkDescriptorPool, Rc<Device>, Vec<VkDescriptorSet>);
pub type DescriptorSetArrayView = [VkDescriptorSet];
impl DescriptorSets
{
	pub fn new(engine: &GraphicsInterface, layouts: &[&DescriptorSetLayout]) -> EngineResult<Self>
	{
		let (mut uniform_total, mut storage_total, mut combined_sampler_total, mut input_attachment_total) = (0, 0, 0, 0);
		let mut dsls = Vec::with_capacity(layouts.len());
		for &&DescriptorSetLayout(ref dsl, ref ds) in layouts
		{
			dsls.push(dsl.native());
			for desc in ds
			{
				match desc
				{
					&Descriptor::Uniform(n, _) => uniform_total += n,
					&Descriptor::Storage(n, _) => storage_total += n,
					&Descriptor::CombinedSampler(n, _) => combined_sampler_total += n,
					&Descriptor::InputAttachment(n, _) => input_attachment_total += n
				}
			}
		}
		let mut pool_sizes = Vec::new();
		if uniform_total > 0 { pool_sizes.push(VkDescriptorPoolSize { _type: VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, descriptorCount: uniform_total }); }
		if storage_total > 0 { pool_sizes.push(VkDescriptorPoolSize { _type: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, descriptorCount: storage_total }); }
		if combined_sampler_total > 0
		{
			pool_sizes.push(VkDescriptorPoolSize { _type: VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, descriptorCount: combined_sampler_total });
		}
		if input_attachment_total > 0
		{
			pool_sizes.push(VkDescriptorPoolSize { _type: VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT, descriptorCount: input_attachment_total });
		}
		
		let mut dp = unsafe { reserved() };
		let dp = unsafe { vkCreateDescriptorPool(engine.device().native(), &VkDescriptorPoolCreateInfo
		{
			poolSizeCount: pool_sizes.len() as _, pPoolSizes: pool_sizes.as_ptr() as _, maxSets: dsls.len() as _, .. Default::default()
		}, null(), &mut dp) }.make_result(dp)?;
		let mut descriptors = vec![unsafe { reserved() }; dsls.len()];
		unsafe { vkAllocateDescriptorSets(engine.device().native(), &VkDescriptorSetAllocateInfo
		{
			descriptorPool: dp, descriptorSetCount: dsls.len() as _, pSetLayouts: dsls.as_ptr(), .. Default::default()
		}, descriptors.as_mut_ptr()) }.make_result_with(|| DescriptorSets(dp, engine.device().clone(), descriptors))
	}
}
impl Deref for DescriptorSets
{
	type Target = DescriptorSetArrayView;
	fn deref(&self) -> &Self::Target { &self.2 }
}

#[derive(Clone)]
pub struct BufferInfo<'a>(pub &'a BufferResource, pub Range<usize>);
impl<'a> Into<VkDescriptorBufferInfo> for &'a BufferInfo<'a>
{
	fn into(self) -> VkDescriptorBufferInfo
	{
		VkDescriptorBufferInfo { buffer: unsafe { transmute(self.0.internal()) } , offset: self.1.start as _, range: self.1.len() as _ }
	}
}
#[derive(Clone)]
pub struct ImageInfo<'a>(pub &'a Sampler, pub &'a ImageView, pub VkImageLayout);
impl<'a> Into<VkDescriptorImageInfo> for &'a ImageInfo<'a>
{
	fn into(self) -> VkDescriptorImageInfo
	{
		VkDescriptorImageInfo { sampler: self.0.native(), imageView: unsafe { transmute(self.1.internal()) }, imageLayout: self.2 }
	}
}

/// target, binding, infos
pub enum DescriptorSetWriteInfo<'a>
{
	UniformBuffer(VkDescriptorSet, u32, Vec<BufferInfo<'a>>),
	StorageBuffer(VkDescriptorSet, u32, Vec<BufferInfo<'a>>),
	CombinedImageSampler(VkDescriptorSet, u32, Vec<ImageInfo<'a>>),
	InputAttachment(VkDescriptorSet, u32, Vec<ImageInfo<'a>>)
}

pub enum IntoWriteDescriptorSetNativeStruct
{
	Buffers
	{
		target: VkDescriptorSet, binding: u32,
		dtype: VkDescriptorType, buffers: Vec<VkDescriptorBufferInfo>
	},
	Images
	{
		target: VkDescriptorSet, binding: u32,
		dtype: VkDescriptorType, images: Vec<VkDescriptorImageInfo>
	}
}

impl<'a> Into<IntoWriteDescriptorSetNativeStruct> for &'a DescriptorSetWriteInfo<'a>
{
	fn into(self) -> IntoWriteDescriptorSetNativeStruct
	{
		match self
		{
			&DescriptorSetWriteInfo::UniformBuffer(target, binding, ref bufs) => IntoWriteDescriptorSetNativeStruct::Buffers
			{
				target: target, binding: binding, buffers: bufs.iter().map(Into::into).collect(),
				dtype: VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER
			},
			&DescriptorSetWriteInfo::StorageBuffer(target, binding, ref bufs) => IntoWriteDescriptorSetNativeStruct::Buffers
			{
				target: target, binding: binding, buffers: bufs.iter().map(Into::into).collect(),
				dtype: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER
			},
			&DescriptorSetWriteInfo::CombinedImageSampler(target, binding, ref imgs) => IntoWriteDescriptorSetNativeStruct::Images
			{
				target: target, binding: binding, images: imgs.iter().map(Into::into).collect(),
				dtype: VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER
			},
			&DescriptorSetWriteInfo::InputAttachment(target, binding, ref imgs) => IntoWriteDescriptorSetNativeStruct::Images
			{
				target: target, binding: binding, images: imgs.iter().map(Into::into).collect(),
				dtype: VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT
			}
		}
	}
}
impl<'a> Into<VkWriteDescriptorSet> for &'a IntoWriteDescriptorSetNativeStruct
{
	fn into(self) -> VkWriteDescriptorSet
	{
		match self
		{
			&IntoWriteDescriptorSetNativeStruct::Buffers { target, binding, dtype, ref buffers } => VkWriteDescriptorSet
			{
				dstSet: target, dstBinding: binding,
				descriptorType: dtype, descriptorCount: buffers.len() as u32,
				pBufferInfo: buffers.as_ptr(), pImageInfo: null(), pTexelBufferView: null(), .. Default::default()
			},
			&IntoWriteDescriptorSetNativeStruct::Images { target, binding, dtype, ref images } => VkWriteDescriptorSet
			{
				dstSet: target, dstBinding: binding,
				descriptorType: dtype, descriptorCount: images.len() as u32,
				pBufferInfo: null(), pImageInfo: images.as_ptr(), pTexelBufferView: null(), .. Default::default()
			}
		}
	}
}
