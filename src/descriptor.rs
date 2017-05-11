// Prelude: Descriptor and its layout in shader

#![allow(dead_code)]

use rawexports::InternalExports;
use Resource;
use {std, vk};
use vk::defs::*;
use {EngineResult, GraphicsInterface, Sampler, ImageView};
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] #[repr(u8)]
pub enum ShaderStage
{
	Vertex = VK_SHADER_STAGE_VERTEX_BIT as u8,
	TessControl = VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT as u8,
	TessEvaluation = VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT as u8,
	Geometry = VK_SHADER_STAGE_GEOMETRY_BIT as u8,
	Fragment = VK_SHADER_STAGE_FRAGMENT_BIT as u8
}
impl std::ops::BitOr for ShaderStage
{
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self { unsafe { std::mem::transmute(self as u8 | rhs as u8) } }
}
impl std::ops::BitOrAssign for ShaderStage
{
	fn bitor_assign(&mut self, rhs: Self) { *self = *self | rhs; }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Descriptor
{
	Uniform(u32, ShaderStage), Storage(u32, ShaderStage),
	CombinedSampler(u32, ShaderStage),
	InputAttachment(u32, ShaderStage)
}
pub trait DescriptorInternals
{
	fn count(&self) -> u32;
	fn into_binding(&self, index: u32) -> VkDescriptorSetLayoutBinding;
	fn into_pool_size(&self) -> VkDescriptorPoolSize;
}
impl Descriptor
{
	fn native_type(&self) -> VkDescriptorType
	{
		match self
		{
			&Descriptor::Uniform(_, _) => VkDescriptorType::UniformBuffer,
			&Descriptor::Storage(_, _) => VkDescriptorType::StorageBuffer,
			&Descriptor::CombinedSampler(_, _) => VkDescriptorType::CombinedImageSampler,
			&Descriptor::InputAttachment(_, _) => VkDescriptorType::InputAttachment
		}
	}
	fn stage_mask(&self) -> VkShaderStageFlags
	{
		match self
		{
			&Descriptor::Uniform(_, s) => s as VkShaderStageFlags,
			&Descriptor::Storage(_, s) => s as VkShaderStageFlags,
			&Descriptor::CombinedSampler(_, s) => s as VkShaderStageFlags,
			&Descriptor::InputAttachment(_, s) => s as VkShaderStageFlags
		}
	}
}
impl DescriptorInternals for Descriptor
{
	fn count(&self) -> u32
	{
		match self
		{
			&Descriptor::Uniform(n, _) => n, &Descriptor::Storage(n, _) => n,
			&Descriptor::CombinedSampler(n, _) => n,
			&Descriptor::InputAttachment(n, _) => n
		}
	}
	fn into_binding(&self, index: u32) -> VkDescriptorSetLayoutBinding
	{
		VkDescriptorSetLayoutBinding
		{
			binding: index, descriptorType: self.native_type(), descriptorCount: self.count(),
			stageFlags: self.stage_mask(), pImmutableSamplers: std::ptr::null()
		}
	}
	fn into_pool_size(&self) -> VkDescriptorPoolSize
	{
		VkDescriptorPoolSize(self.native_type(), self.count())
	}
}

pub struct DescriptorSetLayout(vk::DescriptorSetLayout, Vec<Descriptor>);
impl DescriptorSetLayout
{
	pub fn new<'a>(engine: &GraphicsInterface, bindings: Cow<'a, [Descriptor]>) -> EngineResult<Self>
	{
		let native = bindings.iter().enumerate().map(|(i, x)| x.into_binding(i as u32)).collect::<Vec<_>>();
		vk::DescriptorSetLayout::new(engine.device(), &native).map(|d| DescriptorSetLayout(d, bindings.into_owned())).map_err(From::from)
	}
	pub fn descriptors(&self) -> &[Descriptor] { &self.1 }
}
pub struct DescriptorSets(vk::DescriptorPool, Vec<VkDescriptorSet>);
impl DescriptorSets
{
	pub fn preallocate(engine: &GraphicsInterface, layouts: &[&DescriptorSetLayout]) -> EngineResult<Self>
	{
		let set_count = layouts.len();
		let (uniform_total, storage_total, combined_sampler_total, ia_total) = layouts.iter().map(|&&DescriptorSetLayout(_, ref ds)|
			ds.iter().fold((0, 0, 0, 0), |(u, s, cs, ia), desc| match desc
			{
				&Descriptor::Uniform(n, _) => (u + n, s, cs, ia),
				&Descriptor::Storage(n, _) => (u, s + n, cs, ia),
				&Descriptor::CombinedSampler(n, _) => (u, s, cs + n, ia),
				&Descriptor::InputAttachment(n, _) => (u, s, cs, ia + n)
			})
		).fold((0, 0, 0, 0), |(u, s, cs, ia), (u2, s2, cs2, ia2)| (u + u2, s + s2, cs + cs2, ia + ia2));
		let mut pool_sizes = Vec::new();
		if uniform_total > 0 { pool_sizes.push(VkDescriptorPoolSize(VkDescriptorType::UniformBuffer, uniform_total)); }
		if storage_total > 0 { pool_sizes.push(VkDescriptorPoolSize(VkDescriptorType::StorageBuffer, storage_total)); }
		if combined_sampler_total > 0 { pool_sizes.push(VkDescriptorPoolSize(VkDescriptorType::CombinedImageSampler, combined_sampler_total)); }
		if ia_total > 0 { pool_sizes.push(VkDescriptorPoolSize(VkDescriptorType::InputAttachment, ia_total)); }

		vk::DescriptorPool::new(engine.device(), set_count, &pool_sizes).and_then(|pool|
			pool.allocate(&layouts.iter().map(|&&DescriptorSetLayout(ref l, _)| **l).collect::<Vec<_>>())
				.map(|sets| DescriptorSets(pool, sets))).map_err(From::from)
	}
}
impl InternalExports for DescriptorSetLayout { type InternalT = vk::DescriptorSetLayout; fn get_internal(&self) -> &Self::InternalT { &self.0 } }

pub type DescriptorSetArrayView = [VkDescriptorSet];
impl std::ops::Deref for DescriptorSets
{
	type Target = DescriptorSetArrayView;
	fn deref(&self) -> &Self::Target { &self.1 }
}

#[derive(Clone)]
pub struct BufferInfo<'a>(pub &'a Resource<Type = VkBuffer>, pub std::ops::Range<usize>);
impl<'a> std::convert::Into<VkDescriptorBufferInfo> for &'a BufferInfo<'a>
{
	fn into(self) -> VkDescriptorBufferInfo
	{
		let &BufferInfo(res, ref range) = self;
		VkDescriptorBufferInfo(res.resource(), range.start as VkDeviceSize, (range.end - range.start) as VkDeviceSize)
	}
}
#[derive(Clone)]
pub struct ImageInfo<'a>(pub &'a Sampler, pub &'a ImageView, pub VkImageLayout);
impl<'a> std::convert::Into<VkDescriptorImageInfo> for &'a ImageInfo<'a>
{
	fn into(self) -> VkDescriptorImageInfo
	{
		let &ImageInfo(smp, view, layout) = self;
		VkDescriptorImageInfo(**smp.get_internal(), view.get_native(), layout)
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
impl<'a> std::convert::Into<IntoWriteDescriptorSetNativeStruct> for &'a DescriptorSetWriteInfo<'a>
{
	fn into(self) -> IntoWriteDescriptorSetNativeStruct
	{
		match self
		{
			&DescriptorSetWriteInfo::UniformBuffer(target, binding, ref bufs) => IntoWriteDescriptorSetNativeStruct::Buffers
			{
				target: target, binding: binding, buffers: bufs.iter().map(|x| x.into()).collect(),
				dtype: VkDescriptorType::UniformBuffer
			},
			&DescriptorSetWriteInfo::StorageBuffer(target, binding, ref bufs) => IntoWriteDescriptorSetNativeStruct::Buffers
			{
				target: target, binding: binding, buffers: bufs.iter().map(|x| x.into()).collect(),
				dtype: VkDescriptorType::StorageBuffer
			},
			&DescriptorSetWriteInfo::CombinedImageSampler(target, binding, ref imgs) => IntoWriteDescriptorSetNativeStruct::Images
			{
				target: target, binding: binding, images: imgs.iter().map(|x| x.into()).collect(),
				dtype: VkDescriptorType::CombinedImageSampler
			},
			&DescriptorSetWriteInfo::InputAttachment(target, binding, ref imgs) => IntoWriteDescriptorSetNativeStruct::Images
			{
				target: target, binding: binding, images: imgs.iter().map(|x| x.into()).collect(),
				dtype: VkDescriptorType::InputAttachment
			}
		}
	}
}
impl <'a> std::convert::Into<VkWriteDescriptorSet> for &'a IntoWriteDescriptorSetNativeStruct
{
	fn into(self) -> VkWriteDescriptorSet
	{
		match self
		{
			&IntoWriteDescriptorSetNativeStruct::Buffers { target, binding, dtype, ref buffers } => VkWriteDescriptorSet
			{
				sType: VkStructureType::WriteDescriptorSet, pNext: std::ptr::null(),
				dstSet: target, dstBinding: binding, dstArrayElement: 0,
				descriptorType: dtype, descriptorCount: buffers.len() as u32,
				pBufferInfo: buffers.as_ptr(), pImageInfo: std::ptr::null(), pTexelBufferView: std::ptr::null()
			},
			&IntoWriteDescriptorSetNativeStruct::Images { target, binding, dtype, ref images } => VkWriteDescriptorSet
			{
				sType: VkStructureType::WriteDescriptorSet, pNext: std::ptr::null(),
				dstSet: target, dstBinding: binding, dstArrayElement: 0,
				descriptorType: dtype, descriptorCount: images.len() as u32,
				pBufferInfo: std::ptr::null(), pImageInfo: images.as_ptr(), pTexelBufferView: std::ptr::null()
			}
		}
	}
}
