// Prelude: Descriptor and its layout in shader

#![allow(dead_code)]

use super::internals::*;
use {std, vk};
use vk::ffi::*;

#[derive(Clone, Copy)]
pub enum ShaderStage { Vertex, TessControl, TessEvaluate, Geometry, Fragment }
impl std::convert::Into<VkShaderStageFlags> for ShaderStage
{
	fn into(self) -> VkShaderStageFlags
	{
		match self
		{
			ShaderStage::Vertex => VK_SHADER_STAGE_VERTEX_BIT,
			ShaderStage::TessControl => VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
			ShaderStage::TessEvaluate => VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
			ShaderStage::Geometry => VK_SHADER_STAGE_GEOMETRY_BIT,
			ShaderStage::Fragment => VK_SHADER_STAGE_FRAGMENT_BIT
		}
	}
}

#[derive(Clone)]
pub enum Descriptor
{
	Uniform(u32, Vec<ShaderStage>),
	CombinedSampler(u32, Vec<ShaderStage>)
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
			&Descriptor::CombinedSampler(_, _) => VkDescriptorType::CombinedImageSampler
		}
	}
	fn stage_mask(&self) -> VkShaderStageFlags
	{
		match self
		{
			&Descriptor::Uniform(_, ref s) => s,
			&Descriptor::CombinedSampler(_, ref s) => s
		}.iter().fold(0, |flag, f| flag | Into::<VkShaderStageFlags>::into(*f))
	}
}
impl DescriptorInternals for Descriptor
{
	fn count(&self) -> u32
	{
		match self
		{
			&Descriptor::Uniform(n, _) => n,
			&Descriptor::CombinedSampler(n, _) => n
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

pub struct DescriptorSetLayout
{
	internal: vk::DescriptorSetLayout,
	structure: Vec<Descriptor>
}
pub trait DescriptorSetLayoutInternals
{
	fn new(dsl: vk::DescriptorSetLayout, structure: &[Descriptor]) -> Self;
	fn descriptors(&self) -> &[Descriptor];
}
impl DescriptorSetLayoutInternals for DescriptorSetLayout
{
	fn new(dsl: vk::DescriptorSetLayout, structure: &[Descriptor]) -> Self
	{
		DescriptorSetLayout { internal: dsl, structure: Vec::from(structure) }
	}
	fn descriptors(&self) -> &[Descriptor] { &self.structure }
}
impl InternalExports<vk::DescriptorSetLayout> for DescriptorSetLayout
{
	fn get_internal(&self) -> &vk::DescriptorSetLayout { &self.internal }
}

pub struct DescriptorSets
{
	#[allow(dead_code)] pool: vk::DescriptorPool, sets: Vec<VkDescriptorSet>
}
pub trait DescriptorSetsInternals
{
	fn new(pool: vk::DescriptorPool, sets: Vec<VkDescriptorSet>) -> Self;
}
impl DescriptorSetsInternals for DescriptorSets
{
	fn new(pool: vk::DescriptorPool, sets: Vec<VkDescriptorSet>) -> Self
	{
		DescriptorSets { pool: pool, sets: sets }
	}
}
impl std::ops::Deref for DescriptorSets
{
	type Target = DescriptorSetArrayView;
	fn deref(&self) -> &Self::Target { &self.sets }
}
pub type DescriptorSetArrayView = [VkDescriptorSet];

#[derive(Clone)]
pub struct BufferInfo<'a>(pub &'a BufferResource, pub std::ops::Range<usize>);
impl <'a> std::convert::Into<VkDescriptorBufferInfo> for &'a BufferInfo<'a>
{
	fn into(self) -> VkDescriptorBufferInfo
	{
		let &BufferInfo(res, ref range) = self;
		VkDescriptorBufferInfo(res.get_resource(), range.start as VkDeviceSize, (range.end - range.start) as VkDeviceSize)
	}
}
#[derive(Clone)]
pub struct ImageInfo<'a>(pub &'a Sampler, pub &'a ImageView, pub VkImageLayout);
impl <'a> std::convert::Into<VkDescriptorImageInfo> for &'a ImageInfo<'a>
{
	fn into(self) -> VkDescriptorImageInfo
	{
		let &ImageInfo(sampler, view, layout) = self;
		VkDescriptorImageInfo(sampler.get_native(), view.get_native(), layout)
	}
}

pub enum DescriptorSetWriteInfo<'a>
{
	UniformBuffer(VkDescriptorSet, u32, Vec<BufferInfo<'a>>),
	CombinedImageSampler(VkDescriptorSet, u32, Vec<ImageInfo<'a>>)
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
impl <'a> std::convert::Into<IntoWriteDescriptorSetNativeStruct> for &'a DescriptorSetWriteInfo<'a>
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
			&DescriptorSetWriteInfo::CombinedImageSampler(target, binding, ref imgs) => IntoWriteDescriptorSetNativeStruct::Images
			{
				target: target, binding: binding, images: imgs.iter().map(|x| x.into()).collect(),
				dtype: VkDescriptorType::CombinedImageSampler
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
