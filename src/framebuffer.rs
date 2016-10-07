// Prelude: RenderPass and Framebuffer

use super::internals::*;
use {std, vk};
use vk::ffi::*;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct AttachmentDesc
{
	pub format: VkFormat, pub samples: VkSampleCountFlagBits,
	pub clear_on_load: Option<bool>, pub preserve_stored_value: bool,
	pub stencil_clear_on_load: Option<bool>, pub preserve_stored_stencil_value: bool,
	pub initial_layout: VkImageLayout, pub final_layout: VkImageLayout
}
impl std::default::Default for AttachmentDesc
{
	fn default() -> Self
	{
		AttachmentDesc
		{
			format: VkFormat::UNDEFINED, samples: VK_SAMPLE_COUNT_1_BIT,
			clear_on_load: None, preserve_stored_value: false,
			stencil_clear_on_load: None, preserve_stored_stencil_value: false,
			initial_layout: VkImageLayout::Undefined, final_layout: VkImageLayout::Undefined
		}
	}
}
impl <'a> std::convert::Into<VkAttachmentDescription> for &'a AttachmentDesc
{
	fn into(self) -> VkAttachmentDescription
	{
		VkAttachmentDescription
		{
			flags: 0, format: self.format, samples: self.samples,
			loadOp: self.clear_on_load.map(|b| if b { VkAttachmentLoadOp::Clear } else { VkAttachmentLoadOp::Load })
				.unwrap_or(VkAttachmentLoadOp::DontCare),
			stencilLoadOp: self.stencil_clear_on_load.map(|b| if b { VkAttachmentLoadOp::Clear } else { VkAttachmentLoadOp::Load })
				.unwrap_or(VkAttachmentLoadOp::DontCare),
			storeOp: if self.preserve_stored_value { VkAttachmentStoreOp::Store } else { VkAttachmentStoreOp::DontCare },
			stencilStoreOp: if self.preserve_stored_stencil_value { VkAttachmentStoreOp::Store } else { VkAttachmentStoreOp::DontCare },
			initialLayout: self.initial_layout, finalLayout: self.final_layout
		}
	}
}
impl AttachmentDesc
{
	pub fn swapchain_buffer(format: VkFormat) -> Self
	{
		AttachmentDesc
		{
			format: format, clear_on_load: Some(true), preserve_stored_value: true,
			initial_layout: VkImageLayout::ColorAttachmentOptimal, final_layout: VkImageLayout::PresentSrcKHR,
			.. Default::default()
		}
	}
}
pub type AttachmentRef = VkAttachmentReference;
impl AttachmentRef
{
	pub fn color(index: u32) -> Self { VkAttachmentReference(index, VkImageLayout::ColorAttachmentOptimal) }
	pub fn input(index: u32) -> Self { VkAttachmentReference(index, VkImageLayout::ShaderReadOnlyOptimal) }
}
#[derive(Clone)]
pub struct PassDesc
{
	pub input_attachment_indices: Vec<AttachmentRef>,
	pub color_attachment_indices: Vec<AttachmentRef>,
	pub resolved_attachment_indices: Option<Vec<AttachmentRef>>,
	pub depth_stencil_attachment_index: Option<AttachmentRef>,
	pub preserved_attachment_indices: Vec<u32>
}
impl std::default::Default for PassDesc
{
	fn default() -> Self
	{
		PassDesc
		{
			input_attachment_indices: Vec::new(),
			color_attachment_indices: Vec::new(),
			resolved_attachment_indices: None,
			depth_stencil_attachment_index: None,
			preserved_attachment_indices: Vec::new()
		}
	}
}
impl PassDesc
{
	pub fn single_fragment_output(index: u32) -> PassDesc
	{
		PassDesc { color_attachment_indices: vec![AttachmentRef::color(index)], .. Default::default() }
	}
}
impl <'a> std::convert::Into<VkSubpassDescription> for &'a PassDesc
{
	fn into(self) -> VkSubpassDescription
	{
		VkSubpassDescription
		{
			flags: 0, pipelineBindPoint: VkPipelineBindPoint::Graphics,
			inputAttachmentCount: self.input_attachment_indices.len() as u32,
			pInputAttachments: self.input_attachment_indices.as_ptr(),
			colorAttachmentCount: self.color_attachment_indices.len() as u32,
			pColorAttachments: self.color_attachment_indices.as_ptr(),
			pResolveAttachments: self.resolved_attachment_indices.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
			pDepthStencilAttachment: self.depth_stencil_attachment_index.as_ref().map(|x| x as *const AttachmentRef).unwrap_or(std::ptr::null_mut()),
			preserveAttachmentCount: self.preserved_attachment_indices.len() as u32,
			pPreserveAttachments: self.preserved_attachment_indices.as_ptr()
		}
	}
}
#[derive(Clone, Copy)]
pub struct PassDependency
{
	pub src: u32, pub dst: u32,
	pub src_stage_mask: VkPipelineStageFlags, pub dst_stage_mask: VkPipelineStageFlags,
	pub src_access_mask: VkAccessFlags, pub dst_access_mask: VkAccessFlags,
	pub depend_by_region: bool
}
impl std::default::Default for PassDependency
{
	fn default() -> Self
	{
		PassDependency
		{
			src: 0, dst: 0,
			src_stage_mask: VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
			dst_stage_mask: VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
			src_access_mask: VK_ACCESS_MEMORY_READ_BIT,
			dst_access_mask: VK_ACCESS_MEMORY_READ_BIT,
			depend_by_region: false
		}
	}
}
impl <'a> std::convert::Into<VkSubpassDependency> for &'a PassDependency
{
	fn into(self) -> VkSubpassDependency
	{
		VkSubpassDependency
		{
			srcSubpass: self.src, dstSubpass: self.dst,
			srcStageMask: self.src_stage_mask, dstStageMask: self.dst_stage_mask,
			srcAccessMask: self.src_access_mask, dstAccessMask: self.dst_access_mask,
			dependencyFlags: if self.depend_by_region { VK_DEPENDENCY_BY_REGION_BIT } else { 0 }
		}
	}
}
impl PassDependency
{
	pub fn fragment_referer(src_pass: u32, dst_pass: u32, dep_by_region: bool) -> Self
	{
		PassDependency
		{
			src: src_pass, dst: dst_pass,
			src_stage_mask: VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, dst_stage_mask: VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
			src_access_mask: VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT, dst_access_mask: VK_ACCESS_SHADER_READ_BIT,
			depend_by_region: dep_by_region
		}
	}
}
#[derive(Clone, Copy)]
pub enum AttachmentClearValue
{
	Color(f32, f32, f32, f32), DepthStencil(f32, u32)
}
impl <'a> std::convert::Into<VkClearValue> for &'a AttachmentClearValue
{
	fn into(self) -> VkClearValue
	{
		VkClearValue(match self
		{
			&AttachmentClearValue::Color(r, g, b, a) => VkClearColorValue(r, g, b, a),
			&AttachmentClearValue::DepthStencil(d, s) => unsafe
			{
				*std::mem::transmute::<_, &VkClearColorValue>(&VkClearDepthStencilValue(d, s))
			}
		})
	}
}

pub struct RenderPass { internal: Rc<vk::RenderPass> }
pub struct Framebuffer { mold: Rc<vk::RenderPass>, internal: vk::Framebuffer, area: VkExtent2D }
impl InternalExports<Rc<vk::RenderPass>> for RenderPass { fn get_internal(&self) -> &Rc<vk::RenderPass> { &self.internal } }
impl InternalExports<vk::Framebuffer> for Framebuffer { fn get_internal(&self) -> &vk::Framebuffer { &self.internal } }
pub trait RenderPassInternals
{
	fn new(rp: vk::RenderPass) -> Self;
}
impl RenderPassInternals for RenderPass
{
	fn new(rp: vk::RenderPass) -> Self { RenderPass { internal: Rc::new(rp) } }
}
pub trait FramebufferInternals
{
	fn new(fb: vk::Framebuffer, mold: &Rc<vk::RenderPass>, area: VkExtent2D) -> Self;
	fn get_mold(&self) -> &Rc<vk::RenderPass>;
	fn get_area(&self) -> VkExtent2D;
}
impl FramebufferInternals for Framebuffer
{
	fn new(fb: vk::Framebuffer, mold: &Rc<vk::RenderPass>, area: VkExtent2D) -> Self
	{
		Framebuffer { internal: fb, mold: mold.clone(), area: area }
	}
	fn get_mold(&self) -> &Rc<vk::RenderPass> { &self.mold }
	fn get_area(&self) -> VkExtent2D { self.area }
}
