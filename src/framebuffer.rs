///! Interlude: RenderPass and Framebuffer

use super::internals::*;
use {std, vk};
use vk::ffi::*;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone)]
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
impl<'a> std::convert::Into<VkAttachmentDescription> for &'a AttachmentDesc
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
impl<'a> std::convert::Into<VkSubpassDescription> for &'a PassDesc
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
#[derive(Clone)]
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
impl<'a> std::convert::Into<VkSubpassDependency> for &'a PassDependency
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
#[derive(Clone)]
pub enum AttachmentClearValue
{
	Color(f32, f32, f32, f32), DepthStencil(f32, u32)
}
impl<'a> std::convert::Into<VkClearValue> for &'a AttachmentClearValue
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

fn into_ref_of_ref<'a, T, U>(from: &[&'a T]) -> Vec<U> where &'a T: std::convert::Into<U>
{
	from.into_iter().map(|&x| x.into()).collect()
}

use ginterface::GraphicsInterface;
use EngineResult;

/// RenderPass: Determines how pixels in attachments are used.
pub struct RenderPass(Rc<vk::RenderPass>);
/// Framebuffer: A set of Attachments(ImageViews) and RenderPass
pub struct Framebuffer { mold: RenderPass, internal: vk::Framebuffer, area: VkExtent2D }
/// A pair of RenderPass and subpass index
#[derive(Clone)]
pub struct PreciseRenderPass<'a>(pub &'a RenderPass, pub u32);
impl RenderPass
{
	pub fn new(engine: &GraphicsInterface, attachments: &[&AttachmentDesc], passes: &[&PassDesc], deps: &[&PassDependency]) -> EngineResult<Self>
	{
		let attachments = into_ref_of_ref(attachments);
		let passes = into_ref_of_ref(passes);
		let deps = into_ref_of_ref(deps);

		vk::RenderPass::new(engine.device(), &VkRenderPassCreateInfo
		{
			sType: VkStructureType::RenderPassCreateInfo, pNext: std::ptr::null(), flags: 0,
			attachmentCount: attachments.len() as u32, subpassCount: passes.len() as u32, dependencyCount: deps.len() as u32,
			pAttachments: attachments.as_ptr(), pSubpasses: passes.as_ptr(), pDependencies: deps.as_ptr()
		}).map(|x| RenderPass(Rc::new(x))).map_err(From::from)
	}
}
impl Framebuffer
{
	pub fn new(engine: &GraphicsInterface, mold: &RenderPass, attachments: &[&ImageView], form: &Size3) -> EngineResult<Self>
	{
		let attachments = attachments.into_iter().map(|&x| x.get_native()).collect::<Vec<_>>();
		let &Size3(w, h, l) = form;

		vk::Framebuffer::new(engine.device(), &VkFramebufferCreateInfo
		{
			sType: VkStructureType::FramebufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			renderPass: **mold.0, attachmentCount: attachments.len() as u32, pAttachments: attachments.as_ptr(),
			width: w, height: h, layers: l
		}).map(|f| Framebuffer { internal: f, mold: RenderPass(mold.0.clone()), area: VkExtent2D(w, h) }).map_err(From::from)
	}
	pub fn new_for_presented<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(engine: &Engine, attachment: &ImageView, clear_mode: Option<bool>, form: &Size3)
		-> EngineResult<Self>
	{
		engine.presenting_renderpass(attachment.format(), clear_mode).and_then(|m| Self::new(engine, m, &[attachment], form))
	}
	pub fn new_with_default<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(engine: &Engine, attachment: &ImageView, clear_mode: Option<bool>, form: &Size3)
		-> EngineResult<Self>
	{
		engine.default_renderpass(attachment.format(), clear_mode).and_then(|m| Self::new(engine, m, &[attachment], form))
	}
	pub fn renderpass(&self) -> &RenderPass { &self.mold }
	pub fn area(&self) -> &VkExtent2D { &self.area }
}

impl InternalExports for RenderPass { type InternalT = vk::RenderPass; fn get_internal(&self) -> &vk::RenderPass { &self.0 } }
impl InternalExports for Framebuffer { type InternalT = vk::Framebuffer; fn get_internal(&self) -> &vk::Framebuffer { &self.internal } }
