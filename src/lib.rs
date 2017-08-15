// Interlude crate: In-home Multimedia Framework powered by Vulkan

// External Crates
extern crate libc;
#[macro_use] extern crate log;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate user32;
#[cfg(windows)] extern crate widestring;
#[cfg(windows)] extern crate ole32;
extern crate nalgebra;
extern crate freetype_sys;
extern crate unicode_normalization;
extern crate ansi_term;
extern crate mio;
#[cfg(feature = "target_xlib")] extern crate x11;
#[macro_use] extern crate interlude_vk_defs;
extern crate interlude_vk_funport;

// Interlude
mod subsystem_layer;
mod error;
mod engine;
mod ginterface;
mod device;
mod command;
mod resource;
mod framebuffer;
mod synchronize;
mod shading;
mod render_surface;
mod descriptor;
mod input;
mod data;
mod internal_traits;
mod concurrent;
mod wsi;

// platform dependents
#[cfg(unix)] mod linux;
#[cfg(windows)] mod win32;
#[cfg(unix)] pub use linux::NativeInput as Input;
#[cfg(windows)] pub use win32::NativeInput as Input;

/// Application State(has exited?)
#[derive(PartialEq, Debug)]
pub enum ApplicationState { Continue, EventArrived(u32), Exited }

// Extra Objects
// mod debug_info;

// Enum Flags //
pub use framebuffer::{AccessFlag, AccessFlags};
pub use shading::{ShaderStage, ShaderStageSet};
pub use shading::{PipelineStageFlag, PipelineStage, PipelineStages};
pub use resource::{ImageAspect, ImageAspectSet};

// --- Exported APIs --- //
pub use error::*;
pub use engine::{EngineBuilder, EmptyInput};
pub use ginterface::GraphicsInterface;
pub use framebuffer::{AttachmentDesc, AttachmentRef, PassDesc, PassDependency, AttachmentClearValue, PreciseRenderPass};
pub use command::{
	MemoryBarrier, BufferMemoryBarrier, ImageMemoryBarrier, IndirectCallParameter, BufferCopyRegion, ImageCopyRegion, ImageBlitRegion,
	GraphicsCommandBuffersView, TransferCommandBuffersView
};
pub use resource::{
	ImageSubresourceRange, ImageSubresourceLayers, BufferContent, BufferOffsets,
	ImageDescriptor1, ImageDescriptor2, ImageDescriptor3,
	SamplerState, ComponentSwizzle, ComponentMapping, Filter, ImageLayout
};
pub use shading::{
	ConstantEntry, VertexBinding, VertexAttribute, PushConstantDesc,
	PrimitiveTopology, ViewportWithScissorRect, RasterizerState, AttachmentBlendState,
	GraphicsPipelineBuilder
};
pub use descriptor::{Descriptor, BufferInfo, ImageInfo, DescriptorSetWriteInfo, DescriptorSetArrayView};
// pub use debug_info::DebugLine;
pub use input::*;
pub use data::*;
pub use concurrent::*;
pub use render_surface::*;
// Transient or Stateful APIs //
pub use command::{GraphicsCommandRecorder, TransferCommandRecorder, BundleCommandRecorder};
pub use command::{ImmediateGraphicsCommandSubmission, ImmediateTransferCommandSubmission};

// traits
pub use engine::{AssetProvider, AssetPath, CommandSubmitter};
pub use command::{PrimaryCommandBuffers, SecondaryCommandBuffers, DrawingCommandRecorder, QueueSyncOperationCommandRecorder};
pub use command::{PrimaryGraphicsCommandRecorder, PrimaryTransferCommandRecorder, ClosableCommandRecorder, CommandInjection};
pub use resource::{ImageView, BufferResource, ImageResource, StagingResource};
pub use shading::Shader;
// exported objects
pub use engine::Engine;
pub use synchronize::{QueueFence, Fence};
pub use framebuffer::{RenderPass, Framebuffer};
pub use command::{GraphicsCommandBuffers, BundledCommandBuffers, TransferCommandBuffers, TransientTransferCommandBuffers, TransientGraphicsCommandBuffers};
pub use resource::{Image1D, Image2D, Image3D, LinearImage, DeviceBuffer, StagingBuffer, DeviceImages, StagingImages};
pub use resource::{ImageView1D, ImageView2D, ImageView3D, Sampler, BufferPreallocator, ImagePreallocator, MappedRange};
pub use shading::{VertexShader, TessellationControlShader, TessellationEvaluationShader, GeometryShader, FragmentShader, ShaderModule};
pub use shading::{PipelineShaderProgram, PipelineLayout, GraphicsPipelines, GraphicsPipeline};
pub use descriptor::{DescriptorSetLayout, DescriptorSets};
// pub use debug_info::DebugInfo;

pub type EngineResult<T> = Result<T, EngineError>;
/// Result<_, EngineError> as Unrecoverable(Crashes immediately)
pub trait UnrecoverableExt<T> { fn or_crash(self) -> T; }
impl<T> UnrecoverableExt<T> for EngineResult<T>
{
	fn or_crash(self) -> T
	{
		match self { Err(e) => self::crash(e), Ok(o) => o }
	}
}
