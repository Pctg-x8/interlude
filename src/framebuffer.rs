///! Interlude: RenderPass and Framebuffer

use interlude_vk_defs::*;
use interlude_vk_funport::*;
use {ImageView, GraphicsInterface, Size2, AssetProvider, EngineResult};
use device::Device;
use subsystem_layer::{NativeHandleProvider, NativeResultValueHandler};
use std::rc::Rc;
use std::ops::{Deref, BitOr, BitOrAssign};
use std::mem::{transmute, zeroed, uninitialized as reserved};
use std::ptr::{null, null_mut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachmentDesc
{
	pub format: VkFormat,
	/// connect OR for support multiple sample counts(e.g. 1 | 2 | 8)
	pub samples: u32,
	pub clear_on_load: Option<bool>, pub preserve_stored_value: bool,
	pub stencil_clear_on_load: Option<bool>, pub preserve_stored_stencil_value: bool,
	pub initial_layout: VkImageLayout, pub final_layout: VkImageLayout
}
impl Default for AttachmentDesc
{
	fn default() -> Self
	{
		AttachmentDesc
		{
			samples: 1, clear_on_load: None, stencil_clear_on_load: None, .. unsafe { zeroed() }
		}
	}
}
impl<'a> Into<VkAttachmentDescription> for &'a AttachmentDesc
{
	fn into(self) -> VkAttachmentDescription
	{
		VkAttachmentDescription
		{
			format: self.format, samples: self.samples, initialLayout: self.initial_layout, finalLayout: self.final_layout, flags: 0,
			loadOp: self.clear_on_load.map(|b| if b { VK_ATTACHMENT_LOAD_OP_CLEAR } else { VK_ATTACHMENT_LOAD_OP_LOAD })
				.unwrap_or(VK_ATTACHMENT_LOAD_OP_DONT_CARE),
			stencilLoadOp: self.stencil_clear_on_load.map(|b| if b { VK_ATTACHMENT_LOAD_OP_CLEAR } else { VK_ATTACHMENT_LOAD_OP_LOAD })
				.unwrap_or(VK_ATTACHMENT_LOAD_OP_DONT_CARE),
			storeOp: if self.preserve_stored_value { VK_ATTACHMENT_STORE_OP_STORE } else { VK_ATTACHMENT_STORE_OP_DONT_CARE },
			stencilStoreOp: if self.preserve_stored_stencil_value { VK_ATTACHMENT_STORE_OP_STORE } else { VK_ATTACHMENT_STORE_OP_DONT_CARE }
		}
	}
}
impl AttachmentDesc
{
	/// For Swapchain Buffer: Layout Transition = ColorAttachmentOptimal -> PresentSrc, ClearOnLoad, PreserveStoredValue
	pub fn swapchain_buffer(format: VkFormat) -> Self
	{
		AttachmentDesc
		{
			format: format, clear_on_load: Some(true), preserve_stored_value: true,
			initial_layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, final_layout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
			.. Default::default()
		}
	}
}
#[repr(C)] #[derive(Clone, PartialEq, Eq, Debug)]
pub struct AttachmentRef(VkAttachmentReference);
impl AttachmentRef
{
	/// Color Attachment
	pub fn color(index: u32) -> Self { AttachmentRef(VkAttachmentReference { attachment: index, layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL }) }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PassDesc
{
	pub input_attachment_indices: Vec<AttachmentRef>,
	pub color_attachment_indices: Vec<AttachmentRef>,
	pub resolved_attachment_indices: Vec<AttachmentRef>,
	pub depth_stencil_attachment_index: Option<AttachmentRef>,
	pub preserved_attachment_indices: Vec<u32>
}
impl Default for PassDesc
{
	fn default() -> Self
	{
		PassDesc
		{
			input_attachment_indices: Vec::new(),
			color_attachment_indices: Vec::new(),
			resolved_attachment_indices: Vec::new(),
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
impl<'a> Into<VkSubpassDescription> for &'a PassDesc
{
	fn into(self) -> VkSubpassDescription
	{
		fn vec_ptr_or_null<T>(v: &[T]) -> *const T { if v.is_empty() { null() } else { v.as_ptr() } }
		VkSubpassDescription
		{
			flags: 0, pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
			inputAttachmentCount: self.input_attachment_indices.len() as _,
			pInputAttachments: unsafe { transmute(self.input_attachment_indices.as_ptr()) },
			colorAttachmentCount: self.color_attachment_indices.len() as _,
			pColorAttachments: unsafe { transmute(self.color_attachment_indices.as_ptr()) },
			pResolveAttachments: unsafe { transmute(vec_ptr_or_null(&self.resolved_attachment_indices)) },
			pDepthStencilAttachment: self.depth_stencil_attachment_index.as_ref().map(|x| unsafe { transmute(x) }).unwrap_or_else(null),
			preserveAttachmentCount: self.preserved_attachment_indices.len() as _,
			pPreserveAttachments: self.preserved_attachment_indices.as_ptr()
		}
	}
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PassDependency
{
	pub src: u32, pub dst: u32,
	pub src_stage_mask: VkPipelineStageFlags, pub dst_stage_mask: VkPipelineStageFlags,
	pub src_access_mask: AccessFlags, pub dst_access_mask: AccessFlags,
	pub depend_by_region: bool
}
impl Default for PassDependency
{
	fn default() -> Self
	{
		PassDependency
		{
			src: 0, dst: 0,
			src_stage_mask: VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
			dst_stage_mask: VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT,
			src_access_mask: AccessFlags::MemoryRead,
			dst_access_mask: AccessFlags::MemoryRead,
			depend_by_region: false
		}
	}
}
impl<'a> Into<VkSubpassDependency> for &'a PassDependency
{
	fn into(self) -> VkSubpassDependency
	{
		VkSubpassDependency
		{
			srcSubpass: self.src, dstSubpass: self.dst,
			srcStageMask: self.src_stage_mask, dstStageMask: self.dst_stage_mask,
			srcAccessMask: self.src_access_mask as VkAccessFlags, dstAccessMask: self.dst_access_mask as VkAccessFlags,
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
			src_access_mask: AccessFlags::ColorAttachmentWrite, dst_access_mask: AccessFlags::ShaderRead,
			depend_by_region: dep_by_region
		}
	}
}
#[derive(Clone, Debug, PartialEq)]
pub enum AttachmentClearValue
{
	Color(f32, f32, f32, f32), DepthStencil(f32, u32)
}
impl<'a> Into<VkClearValue> for &'a AttachmentClearValue
{
	fn into(self) -> VkClearValue
	{
		match self
		{
			&AttachmentClearValue::Color(r, g, b, a) => VkClearValue { color: VkClearColorValue { float32: [r, g, b, a] } },
			&AttachmentClearValue::DepthStencil(depth, stencil) => VkClearValue { depthStencil: VkClearDepthStencilValue { depth, stencil } }
		}
	}
}

use subsystem_layer::{NativeRenderPass, NativeFramebuffer};
/// RenderPass: Determines how pixels in attachments are used.
#[derive(Clone)] pub struct RenderPass(Rc<NativeRenderPass>);
/// Framebuffer: A set of Attachments(ImageViews) and RenderPass
#[derive(Clone)] pub struct Framebuffer { internal: Rc<NativeFramebuffer>, mold_ref: RenderPass, area: VkExtent2D }
/// A pair of RenderPass and subpass index
#[derive(Clone)]
pub struct PreciseRenderPass<'a>(pub &'a RenderPass, pub u32);
impl RenderPass
{
	/// Creates a Render Pass
	pub fn new(engine: &GraphicsInterface, attachments: &[AttachmentDesc], passes: &[PassDesc], deps: &[PassDependency]) -> EngineResult<Self>
	{
		let attachments = attachments.into_iter().map(Into::into).collect::<Vec<_>>();
		let passes = passes.into_iter().map(Into::into).collect::<Vec<_>>();
		let deps = deps.into_iter().map(Into::into).collect::<Vec<_>>();
		let mut rp = unsafe { reserved() };
		unsafe { vkCreateRenderPass(engine.device().native(), &VkRenderPassCreateInfo
		{
			attachmentCount: attachments.len() as _, subpassCount: passes.len() as _, dependencyCount: deps.len() as _,
			pAttachments: attachments.as_ptr(), pSubpasses: passes.as_ptr(), pDependencies: deps.as_ptr(), .. Default::default()
		}, null(), &mut rp) }.make_result_with(|| RenderPass(Rc::new(NativeRenderPass(rp, engine.device().clone()))))
	}
}
impl Framebuffer
{
	/// Creates a Framebuffer
	pub fn new(engine: &GraphicsInterface, mold: &RenderPass, attachments: &[&ImageView], form: &Size2, layers: u32) -> EngineResult<Self>
	{
		let attachments = attachments.into_iter().map(|x| unsafe { transmute(x.internal()) }).collect::<Vec<_>>();
		let &Size2(width, height) = form;
		let mut fb = unsafe { reserved() };
		unsafe { vkCreateFramebuffer(engine.device().native(), &VkFramebufferCreateInfo
		{
			renderPass: mold.native(), attachmentCount: attachments.len() as _, pAttachments: attachments.as_ptr(),
			width, height, layers, .. Default::default()
		}, null(), &mut fb) }.make_result_with(|| Framebuffer
		{
			internal: Rc::new(NativeFramebuffer(fb, engine.device().clone())), mold_ref: mold.clone(), area: VkExtent2D { width, height }
		})
	}
	pub fn new_for_presented<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(engine: &Engine, attachment: &ImageView, clear_mode: Option<bool>, form: &Size2)
		-> EngineResult<Self>
	{
		engine.presenting_renderpass(attachment.format(), clear_mode).and_then(|m| Self::new(engine, m, &[attachment], form, 1))
	}
	pub fn new_with_default<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(engine: &Engine, attachment: &ImageView, clear_mode: Option<bool>, form: &Size2)
		-> EngineResult<Self>
	{
		engine.default_renderpass(attachment.format(), clear_mode).and_then(|m| Self::new(engine, m, &[attachment], form, 1))
	}
	pub fn renderpass(&self) -> &RenderPass { &self.mold_ref }
	pub fn area(&self) -> &VkExtent2D { &self.area }
}
impl NativeHandleProvider for RenderPass { type NativeT = VkRenderPass; fn native(&self) -> VkRenderPass { self.0.native() } }
impl NativeHandleProvider for Framebuffer { type NativeT = VkFramebuffer; fn native(&self) -> VkFramebuffer { self.internal.native() } }

/// Access Flags Mask
#[repr(u32)] #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AccessFlags
{
	IndirectCommandRead = VK_ACCESS_INDIRECT_COMMAND_READ_BIT,
	IndexRead = VK_ACCESS_INDEX_READ_BIT,
	VertexAttributeRead = VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT,
	UniformRead = VK_ACCESS_UNIFORM_READ_BIT,
	InputAttachmentRead = VK_ACCESS_INPUT_ATTACHMENT_READ_BIT,
	ShaderRead = VK_ACCESS_SHADER_READ_BIT,
	ShaderWrite = VK_ACCESS_SHADER_WRITE_BIT,
	ColorAttachmentRead = VK_ACCESS_COLOR_ATTACHMENT_READ_BIT,
	ColorAttachmentWrite = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
	DepthStencilAttachmentRead = VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
	DepthStencilAttachmentWrite = VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
	TransferRead = VK_ACCESS_TRANSFER_READ_BIT,
	TransferWrite = VK_ACCESS_TRANSFER_WRITE_BIT,
	HostRead = VK_ACCESS_HOST_READ_BIT,
	HostWrite = VK_ACCESS_HOST_WRITE_BIT,
	MemoryRead = VK_ACCESS_MEMORY_READ_BIT,
	MemoryWrite = VK_ACCESS_MEMORY_WRITE_BIT
}

impl BitOr for AccessFlags
{
	type Output = AccessFlags;
	fn bitor(self, rhs: Self) -> Self { unsafe { transmute(self as u32 | rhs as u32) } }
}
impl BitOrAssign for AccessFlags
{
	fn bitor_assign(&mut self, rhs: Self) { *self = unsafe { transmute(*self as u32 | rhs as u32) }; }
}
