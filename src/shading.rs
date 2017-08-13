///! Interlude: Primitive Shading(Shaders and Pipelines)

use interlude_vk_defs::*;
use interlude_vk_funport::*;
use {EngineResult, GraphicsInterface, PreciseRenderPass, AssetProvider, AssetPath, RenderPass, DescriptorSetLayout};
use device::Device;
use std::ffi::CString;
use std::ops::{Deref, DerefMut, Range, BitOr, BitOrAssign};
use std::io::prelude::*;
use std::fs::File;
use std::rc::Rc;
use std::path::Path;
use std::mem::{size_of, transmute, zeroed};
use std::ptr::{null, null_mut};
use subsystem_layer::{NativeHandleProvider, NativeResultValueHandler};
use data::*;
use libc::c_char;

/// Shader Stage bitflags
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] #[repr(u8)]
pub enum ShaderStage
{
	Vertex = VK_SHADER_STAGE_VERTEX_BIT as u8,
	TessControl = VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT as u8,
	TessEvaluation = VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT as u8,
	Geometry = VK_SHADER_STAGE_GEOMETRY_BIT as u8,
	Fragment = VK_SHADER_STAGE_FRAGMENT_BIT as u8
}
/// Set of Shader Stage bitflags
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ShaderStageSet(VkFlags);
impl ShaderStageSet
{
	pub fn has_vertex_bit(&self) -> bool { (self.0 & ShaderStage::Vertex as VkFlags) != 0 }
	pub fn has_tessellation_control_bit(&self) -> bool { (self.0 & ShaderStage::TessControl as VkFlags) != 0 }
	pub fn has_tessellation_evaluation_bit(&self) -> bool { (self.0 & ShaderStage::TessEvaluation as VkFlags) != 0 }
	pub fn has_geometry_bit(&self) -> bool { (self.0 & ShaderStage::Geometry as VkFlags) != 0 }
	pub fn has_fragment_bit(&self) -> bool { (self.0 & ShaderStage::Fragment as VkFlags) != 0 }
}
pub fn shader_stage_flags(set: ShaderStageSet) -> VkShaderStageFlags { set.0 as _ }
pub fn retrieve_shader_stage_flags(sss: ShaderStageSet) -> VkShaderStageFlags { sss.0 as _ }
impl BitOr for ShaderStage
{
	type Output = ShaderStageSet;
	fn bitor(self, rhs: Self) -> ShaderStageSet { ShaderStageSet(self as VkFlags | rhs as VkFlags) }
}
impl BitOr for ShaderStageSet
{
	type Output = ShaderStageSet;
	fn bitor(self, rhs: Self) -> ShaderStageSet { ShaderStageSet(self.0 | rhs.0) }
}
impl BitOr<ShaderStage> for ShaderStageSet
{
	type Output = ShaderStageSet;
	fn bitor(self, rhs: ShaderStage) -> ShaderStageSet { ShaderStageSet(self.0 | rhs as VkFlags) }
}
impl BitOr<ShaderStageSet> for ShaderStage
{
	type Output = ShaderStageSet;
	fn bitor(self, rhs: ShaderStageSet) -> ShaderStageSet { ShaderStageSet(self as VkFlags | rhs.0) }
}
impl BitOrAssign for ShaderStageSet
{
	fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}
impl BitOrAssign<ShaderStage> for ShaderStageSet
{
	fn bitor_assign(&mut self, rhs: ShaderStage) { self.0 |= rhs as _; }
}
impl Into<ShaderStageSet> for ShaderStage
{
	fn into(self) -> ShaderStageSet { ShaderStageSet(self as _) }
}
impl Into<VkShaderStageFlags> for ShaderStageSet { fn into(self) -> VkShaderStageFlags { self.0 as _ } }

#[derive(Clone, Debug, PartialEq)]
pub enum VertexBinding { PerVertex(u32), PerInstance(u32) }
#[derive(Clone, Debug, PartialEq)]
pub struct VertexAttribute(pub u32, pub VkFormat, pub u32);
pub struct IntoNativeVertexInputState
{
	bindings: Vec<VkVertexInputBindingDescription>,
	attributes: Vec<VkVertexInputAttributeDescription>
}

/// Discrete Shader Module that holds shader program
pub struct ShaderModule(VkShaderModule, Rc<Device>);
struct VertexProcessing { module: ShaderModule, entry_point: CString, input_state: IntoNativeVertexInputState }
struct ShaderModuleWithEntryPoint { module: ShaderModule, entry_point: CString }
impl ShaderModule
{
	fn load_module(engine: &GraphicsInterface, path: &Path) -> EngineResult<VkShaderModule>
	{
		let content = File::open(path).and_then(|mut fp| { let mut vb = Vec::new(); fp.read_to_end(&mut vb).map(|_| vb) })?;
		let mut smod = unsafe { zeroed() };
		unsafe { vkCreateShaderModule(engine.device().native(), &VkShaderModuleCreateInfo
		{
			codeSize: content.len() as _, pCode: content.as_ptr() as _, .. Default::default()
		}, null(), &mut smod) }.make_result(smod)
	}
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P) -> EngineResult<Self>
	{
		let path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading Shader from {:?}...", path);
		Self::load_module(engine, &path).map(|m| ShaderModule(m, engine.device().clone()))
	}
	fn from_asset_msg<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, shader_msg: &str) -> EngineResult<Self>
	{
		let path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading {} from {:?}...", shader_msg, path);
		Self::load_module(engine, &path).map(|m| ShaderModule(m, engine.device().clone()))
	}

	pub fn into_vertex_shader(self, entry_point: &str, bindings: &[VertexBinding], attributes: &[VertexAttribute]) -> EngineResult<VertexShader>
	{
		let input_state = IntoNativeVertexInputState
		{
			bindings: bindings.iter().enumerate().map(|(i, x)| match x
			{
				&VertexBinding::PerVertex(stride) => VkVertexInputBindingDescription { binding: i as _, stride, inputRate: VK_VERTEX_INPUT_RATE_VERTEX },
				&VertexBinding::PerInstance(stride) => VkVertexInputBindingDescription { binding: i as _, stride, inputRate: VK_VERTEX_INPUT_RATE_INSTANCE }
			}).collect(),
			attributes: attributes.iter().enumerate()
				.map(|(i, &VertexAttribute(binding, format, offset))| VkVertexInputAttributeDescription { location: i as _, binding, format, offset })
				.collect()
		};
		CString::new(entry_point).map(|entry_point| VertexShader(Rc::new(VertexProcessing { module: self, entry_point, input_state }))).map_err(From::from)
	}
	pub fn into_tessellation_control_shader(self, entry_point: &str) -> EngineResult<TessellationControlShader>
	{
		CString::new(entry_point).map(|entry_point| TessellationControlShader(Rc::new(ShaderModuleWithEntryPoint { module: self, entry_point }))).map_err(From::from)
	}
	pub fn into_tessellation_evaluation_shader(self, entry_point: &str) -> EngineResult<TessellationEvaluationShader>
	{
		CString::new(entry_point).map(|entry_point| TessellationEvaluationShader(Rc::new(ShaderModuleWithEntryPoint { module: self, entry_point }))).map_err(From::from)
	}
	pub fn into_geometry_shader(self, entry_point: &str) -> EngineResult<GeometryShader>
	{
		CString::new(entry_point).map(|entry_point| GeometryShader(Rc::new(ShaderModuleWithEntryPoint { module: self, entry_point }))).map_err(From::from)
	}
	pub fn into_fragment_shader(self, entry_point: &str) -> EngineResult<FragmentShader>
	{
		CString::new(entry_point).map(|entry_point| FragmentShader(Rc::new(ShaderModuleWithEntryPoint { module: self, entry_point }))).map_err(From::from)
	}
	pub fn into_compute_shader(self, entry_point: &str) -> EngineResult<ComputeShader>
	{
		CString::new(entry_point).map(|entry_point| ComputeShader(Rc::new(ShaderModuleWithEntryPoint { module: self, entry_point }))).map_err(From::from)
	}
}
impl Drop for ShaderModule { fn drop(&mut self) { unsafe { vkDestroyShaderModule(self.1.native(), self.0, null()) }; } }

/// One of Shader Program
pub trait Shader
{
	// TODO: to be associated constant
	fn stage(&self) -> ShaderStage;
	fn entry_point_ptr(&self) -> *const c_char;
}
/// Shader Program for Vertex Processing Stage
#[derive(Clone)] pub struct VertexShader(Rc<VertexProcessing>);
/// Shader Program for Fragment Output Stage
#[derive(Clone)] pub struct FragmentShader(Rc<ShaderModuleWithEntryPoint>);
/// Shader Program for Primitive Processing Stage
#[derive(Clone)] pub struct GeometryShader(Rc<ShaderModuleWithEntryPoint>);
/// Shader Program for Tessellation Control Parameter Generation Stage(stub)
#[derive(Clone)] pub struct TessellationControlShader(Rc<ShaderModuleWithEntryPoint>);
/// Shader Program for Evaluating Tessellated Primitive Stage(stub)
#[derive(Clone)] pub struct TessellationEvaluationShader(Rc<ShaderModuleWithEntryPoint>);
/// Shader Program for general purpose computation
#[derive(Clone)] pub struct ComputeShader(Rc<ShaderModuleWithEntryPoint>);

macro_rules! ImplShaderModule
{
	(for $($t: ty [$stg: ident]),*) =>
	{ $(
		impl NativeHandleProvider for $t { type NativeT = VkShaderModule; fn native(&self) -> VkShaderModule { self.0.module.0 } }
		impl Shader for $t
		{
			fn stage(&self) -> ShaderStage { ShaderStage::$stg }
			fn entry_point_ptr(&self) -> *const c_char { self.0.entry_point.as_ptr() }
		}
	)* }
}
ImplShaderModule!(for VertexShader[Vertex], FragmentShader[Fragment], GeometryShader[Geometry], TessellationControlShader[TessControl], TessellationEvaluationShader[TessEvaluation]);
impl VertexShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str,
		bindings: &[VertexBinding], attributes: &[VertexAttribute]) -> EngineResult<Self>
	{
		ShaderModule::from_asset_msg(engine, path, "Vertex Shader")?.into_vertex_shader(entry_point, bindings, attributes)
	}
	/// Build VertexShader from Asset, vertices are passed each R32G32B32A32_SFLOAT format in single stream.
	pub fn from_asset_for_postprocessing<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str)
		-> EngineResult<Self>
	{
		Self::from_asset(engine, path, entry_point, &[VertexBinding::PerVertex(size_of::<PosUV>() as u32)],
			&[VertexAttribute(0, VK_FORMAT_R32G32B32A32_SFLOAT, 0)])
	}
}
impl TessellationControlShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Self>
	{
		ShaderModule::from_asset_msg(engine, path, "Tessellation Control Shader")?.into_tessellation_control_shader(entry_point)
	}
}
impl TessellationEvaluationShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Self>
	{
		ShaderModule::from_asset_msg(engine, path, "Tessellation Evaluation Shader")?.into_tessellation_evaluation_shader(entry_point)
	}
}
impl GeometryShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Self>
	{
		ShaderModule::from_asset_msg(engine, path, "Geometry Shader")?.into_geometry_shader(entry_point)
	}
}
impl FragmentShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Self>
	{
		ShaderModule::from_asset_msg(engine, path, "Fragment Shader")?.into_fragment_shader(entry_point)
	}
}

/// Pair of visible shader stages and range of push constant indices
#[derive(Clone)] pub struct PushConstantDesc(pub ShaderStageSet, pub Range<u32>);
impl<'a> Into<VkPushConstantRange> for &'a PushConstantDesc
{
	fn into(self) -> VkPushConstantRange
	{
		VkPushConstantRange { stageFlags: (self.0).0 as _, offset: self.1.start, size: self.1.len() as _ }
	}
}

use descriptor::addref_dsl;
use subsystem_layer::{NativePipelineLayout, NativeDescriptorSetLayout};
/// Data Layout which required while rocessing in pipeline
pub struct PipelineLayout { obj: Rc<NativePipelineLayout>, subrefs: Vec<Rc<NativeDescriptorSetLayout>> }
impl PipelineLayout
{
	pub fn new(engine: &GraphicsInterface, descriptor_set_layouts: &[&DescriptorSetLayout], push_constants: &[&PushConstantDesc]) -> EngineResult<Self>
	{
		let (ndsl, subrefs): (Vec<_>, _) = descriptor_set_layouts.into_iter().map(|dsl| (dsl.native(), addref_dsl(&dsl))).unzip();
		let pc = push_constants.into_iter().map(|&pcd| pcd.into()).collect::<Vec<_>>();
		let mut pl = unsafe { zeroed() };
		unsafe { vkCreatePipelineLayout(engine.device().native(), &VkPipelineLayoutCreateInfo
		{
			setLayoutCount: ndsl.len() as _, pSetLayouts: ndsl.as_ptr(), pushConstantRangeCount: pc.len() as _, pPushConstantRanges: pc.as_ptr(),
			.. Default::default()
		}, null(), &mut pl) }.make_result_with(|| PipelineLayout { obj: Rc::new(NativePipelineLayout(pl, engine.device().clone())), subrefs })
	}
}
impl NativeHandleProvider for PipelineLayout { type NativeT = VkPipelineLayout; fn native(&self) -> Self::NativeT { self.obj.native() } }

// Primitive Topology + With-Adjacency flag
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveTopology
{
	Point, LineList(bool), LineStrip(bool), TriangleList(bool), TriangleStrip(bool)
}
impl Into<VkPrimitiveTopology> for PrimitiveTopology
{
	fn into(self) -> VkPrimitiveTopology
	{
		match self
		{
			PrimitiveTopology::Point                => VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
			PrimitiveTopology::LineList(false)		=> VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
			PrimitiveTopology::LineList(true)		=> VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY,
			PrimitiveTopology::LineStrip(false)		=> VK_PRIMITIVE_TOPOLOGY_LINE_STRIP,
			PrimitiveTopology::LineStrip(true)		=> VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY,
			PrimitiveTopology::TriangleList(false)	=> VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
			PrimitiveTopology::TriangleList(true)	=> VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY,
			PrimitiveTopology::TriangleStrip(false)	=> VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
			PrimitiveTopology::TriangleStrip(true)	=> VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY
		}
	}
}
#[derive(Clone, Debug, PartialEq)]
pub struct ViewportWithScissorRect(Viewport, Rect2);
impl ViewportWithScissorRect
{
	pub fn default_scissor(vp: &Viewport) -> Self
	{
		let &Viewport(vx, vy, vw, vh, _, _) = vp;
		ViewportWithScissorRect(vp.clone(), Rect2(Offset2(vx as i32, vy as i32), Size2(vw as u32, vh as u32)))
	}
}
#[derive(Clone, Copy, Debug, PartialEq)] #[repr(u8)]
pub enum CullingSide
{
	Front = VK_CULL_MODE_FRONT_BIT as u8, Back = VK_CULL_MODE_BACK_BIT as u8
}
#[derive(Clone, Debug, PartialEq)]
pub struct RasterizerState
{
	pub wired_render: bool, pub cull_side: Option<CullingSide>
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachmentBlendState
{
	Disabled, AlphaBlend, PremultipliedAlphaBlend
}

#[derive(Clone)]
pub enum ConstantEntry
{
	Float(f32), Uint(u32)
}
impl ConstantEntry
{
	fn _sizeof(&self) -> usize { match self { &ConstantEntry::Float(_) | &ConstantEntry::Uint(_) => 4 } }
	fn as_bytes(&self) -> Vec<u8>
	{
		match self
		{
			&ConstantEntry::Float(v) => Vec::from(&unsafe { transmute::<_, [u8; 4]>(v) }[..]),
			&ConstantEntry::Uint(v) => Vec::from(&unsafe { transmute::<_, [u8; 4]>(v) }[..])
		}
	}
}
pub struct PipelineShaderProgram<Module: Shader + Clone>(pub Module, pub Vec<(usize, ConstantEntry)>);
impl<Module: Shader + Clone> PipelineShaderProgram<Module>
{
	pub fn unspecialized(shref: &Module) -> Self { PipelineShaderProgram(shref.clone(), Vec::new()) }
}
impl<Module: Shader + Clone> Clone for PipelineShaderProgram<Module>
{
	fn clone(&self) -> Self { PipelineShaderProgram(self.0.clone(), self.1.clone()) }
}
pub struct IntoNativeShaderStageCreateInfoStruct
{
	stage_bits: VkShaderStageFlags, module: VkShaderModule, entry_point: *const i8,
	#[allow(dead_code)] specialization_entry: Vec<VkSpecializationMapEntry>,
	#[allow(dead_code)] specialization_values: Vec<u8>,
	specialization_structure: Option<VkSpecializationInfo>
}
pub struct GraphicsPipelineBuilder<'a>
{
	layout: &'a PipelineLayout, render_pass: &'a RenderPass, subpass_index: u32,
	vertex_shader: Option<PipelineShaderProgram<VertexShader>>,
	tesscontrol_shader: Option<PipelineShaderProgram<TessellationControlShader>>,
	tessevaluation_shader: Option<PipelineShaderProgram<TessellationEvaluationShader>>,
	geometry_shader: Option<PipelineShaderProgram<GeometryShader>>,
	fragment_shader: Option<PipelineShaderProgram<FragmentShader>>,
	primitive_topology: PrimitiveTopology, vp_sc: Vec<ViewportWithScissorRect>,
	rasterizer_state: RasterizerState, use_alpha_to_coverage: bool, attachment_blend_states: Vec<AttachmentBlendState>
}
impl<'a> GraphicsPipelineBuilder<'a>
{
	pub fn new(layout: &'a PipelineLayout, render_pass: PreciseRenderPass<'a>) -> Self
	{
		GraphicsPipelineBuilder
		{
			layout: layout, render_pass: render_pass.0, subpass_index: render_pass.1,
			vertex_shader: None, tesscontrol_shader: None, tessevaluation_shader: None, geometry_shader: None, fragment_shader: None,
			primitive_topology: PrimitiveTopology::TriangleList(false),
			vp_sc: Vec::new(), rasterizer_state: RasterizerState { wired_render: false, cull_side: None },
			use_alpha_to_coverage: false, attachment_blend_states: Vec::new()
		}
	}
	pub fn inherit(base: &GraphicsPipelineBuilder<'a>) -> Self
	{
		GraphicsPipelineBuilder
		{
			layout: base.layout, render_pass: base.render_pass, subpass_index: base.subpass_index,
			vertex_shader: base.vertex_shader.clone(),
			tesscontrol_shader: base.tesscontrol_shader.clone(),
			tessevaluation_shader: base.tessevaluation_shader.clone(),
			geometry_shader: base.geometry_shader.clone(),
			fragment_shader: base.fragment_shader.clone(),
			primitive_topology: base.primitive_topology, vp_sc: base.vp_sc.clone(), rasterizer_state: base.rasterizer_state.clone(),
			use_alpha_to_coverage: base.use_alpha_to_coverage, attachment_blend_states: base.attachment_blend_states.clone()
		}
	}
	pub fn for_postprocess<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(engine: &'a Engine, layout: &'a PipelineLayout,
		render_pass: PreciseRenderPass<'a>, fragment_shader: PipelineShaderProgram<FragmentShader>, processing_viewport: &Viewport) -> EngineResult<Self>
	{
		Ok(GraphicsPipelineBuilder
		{
			layout: layout, render_pass: render_pass.0, subpass_index: render_pass.1,
			vertex_shader: Some(PipelineShaderProgram::unspecialized(engine.postprocess_vsh(true)?)),
			tesscontrol_shader: None, tessevaluation_shader: None,
			geometry_shader: None, fragment_shader: Some(fragment_shader),
			primitive_topology: PrimitiveTopology::TriangleStrip(false),
			vp_sc: vec![ViewportWithScissorRect::default_scissor(processing_viewport)],
			rasterizer_state: RasterizerState { wired_render: false, cull_side: None },
			use_alpha_to_coverage: false, attachment_blend_states: vec![AttachmentBlendState::Disabled]
		})
	}

	pub fn vertex_shader(mut self, vshader: PipelineShaderProgram<VertexShader>) -> Self
	{
		self.vertex_shader = Some(vshader); self
	}
	pub fn tesscontrol_shader(mut self, tcshader: PipelineShaderProgram<TessellationControlShader>) -> Self
	{
		self.tesscontrol_shader = Some(tcshader); self
	}
	pub fn tessevaluation_shader(mut self, teshader: PipelineShaderProgram<TessellationEvaluationShader>) -> Self
	{
		self.tessevaluation_shader = Some(teshader); self
	}
	pub fn geometry_shader(mut self, gshader: PipelineShaderProgram<GeometryShader>) -> Self
	{
		self.geometry_shader = Some(gshader); self
	}
	pub fn fragment_shader(mut self, fshader: PipelineShaderProgram<FragmentShader>) -> Self
	{
		self.fragment_shader = Some(fshader); self
	}
	pub fn primitive_topology(mut self, pt: PrimitiveTopology) -> Self
	{
		self.primitive_topology = pt;
		self
	}
	pub fn viewport_scissors(mut self, vpsc: &[ViewportWithScissorRect]) -> Self
	{
		self.vp_sc = Vec::from(vpsc);
		self
	}
	pub fn rasterizer_enable_wired_mode(mut self) -> Self
	{
		self.rasterizer_state.wired_render = true;
		self
	}
	pub fn rasterizer_enable_culling(mut self, side: CullingSide) -> Self
	{
		self.rasterizer_state.cull_side = Some(side);
		self
	}
	pub fn enable_alpha_to_coverage(mut self) -> Self
	{
		self.use_alpha_to_coverage = true;
		self
	}
	pub fn blend_state(mut self, state: &[AttachmentBlendState]) -> Self
	{
		self.attachment_blend_states = Vec::from(state);
		self
	}
}
pub struct IntoNativeGraphicsPipelineCreateInfoStruct<'a>
{
	base: &'a GraphicsPipelineBuilder<'a>,
	#[allow(dead_code)] viewports: Vec<VkViewport>, #[allow(dead_code)] scissors: Vec<VkRect2D>,
	#[allow(dead_code)] attachment_blend_states: Vec<VkPipelineColorBlendAttachmentState>,
	#[allow(dead_code)] into_shader_stage: Vec<IntoNativeShaderStageCreateInfoStruct>,
	shader_stage: Vec<VkPipelineShaderStageCreateInfo>,
	vertex_input_state: VkPipelineVertexInputStateCreateInfo,
	input_assembly_state: VkPipelineInputAssemblyStateCreateInfo,
	viewport_state: VkPipelineViewportStateCreateInfo,
	rasterization_state: VkPipelineRasterizationStateCreateInfo,
	multisample_state: VkPipelineMultisampleStateCreateInfo,
	color_blend_state: VkPipelineColorBlendStateCreateInfo
}
fn make_shaderstage_data<Module: Shader + NativeHandleProvider<NativeT = VkShaderModule> + Clone>(s: &PipelineShaderProgram<Module>)
	-> IntoNativeShaderStageCreateInfoStruct
{
	let (map_entries, const_values) = if s.1.is_empty() { (Vec::new(), Vec::new()) } else
	{
		let map_entries = s.1.iter().scan(0usize, |o, &(id, ref v)|
		{
			let size = v._sizeof();
			let rval = VkSpecializationMapEntry { constantID: id as _, offset: *o as _, size };
			*o += size;
			Some(rval)
		}).collect::<Vec<_>>();
		let const_values = s.1.iter().flat_map(|&(_, ref v)| v.as_bytes().into_iter()).collect::<Vec<_>>();
		(map_entries, const_values)
	};

	IntoNativeShaderStageCreateInfoStruct
	{
		stage_bits: s.0.stage() as _, module: s.0.native(), entry_point: s.0.entry_point_ptr(),
		specialization_structure: if map_entries.is_empty() { None } else
		{
			Some(VkSpecializationInfo
			{
				mapEntryCount: map_entries.len() as u32, pMapEntries: map_entries.as_ptr(),
				dataSize: const_values.len() as usize, pData: const_values.as_ptr() as *const _
			})
		}, specialization_entry: map_entries, specialization_values: const_values
	}
}
fn make_native_vistate_create_info(s: &IntoNativeVertexInputState) -> VkPipelineVertexInputStateCreateInfo
{
	VkPipelineVertexInputStateCreateInfo
	{
		vertexBindingDescriptionCount: s.bindings.len() as u32, pVertexBindingDescriptions: s.bindings.as_ptr(),
		vertexAttributeDescriptionCount: s.attributes.len() as u32, pVertexAttributeDescriptions: s.attributes.as_ptr(),
		.. Default::default()
	}
}
fn make_native_shaderstage(s: &IntoNativeShaderStageCreateInfoStruct) -> VkPipelineShaderStageCreateInfo
{
	VkPipelineShaderStageCreateInfo
	{
		stage: s.stage_bits, module: s.module, pName: s.entry_point,
		pSpecializationInfo: s.specialization_structure.as_ref().map(|n| n as *const VkSpecializationInfo).unwrap_or_else(null),
		.. Default::default()
	}
}
fn make_attachment_blend_state(s: AttachmentBlendState) -> VkPipelineColorBlendAttachmentState
{
	const COLOR_COMPONENT_ALL: VkColorComponentFlags = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;

	match s
	{
		AttachmentBlendState::Disabled => VkPipelineColorBlendAttachmentState
		{
			blendEnable: false as VkBool32, colorWriteMask: COLOR_COMPONENT_ALL, .. Default::default()
		},
		AttachmentBlendState::AlphaBlend => VkPipelineColorBlendAttachmentState
		{
			blendEnable: true as VkBool32,
			srcColorBlendFactor: VK_BLEND_FACTOR_SRC_ALPHA, dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
			srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE, dstAlphaBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
			colorBlendOp: VK_BLEND_OP_ADD, alphaBlendOp: VK_BLEND_OP_ADD, colorWriteMask: COLOR_COMPONENT_ALL
		},
		AttachmentBlendState::PremultipliedAlphaBlend => VkPipelineColorBlendAttachmentState
		{
			blendEnable: true as VkBool32,
			srcColorBlendFactor: VK_BLEND_FACTOR_ONE, dstColorBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
			srcAlphaBlendFactor: VK_BLEND_FACTOR_ONE, dstAlphaBlendFactor: VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
			colorBlendOp: VK_BLEND_OP_ADD, alphaBlendOp: VK_BLEND_OP_ADD, colorWriteMask: COLOR_COMPONENT_ALL
		},
	}
}
impl<'a> Into<IntoNativeGraphicsPipelineCreateInfoStruct<'a>> for &'a GraphicsPipelineBuilder<'a>
{
	fn into(self) -> IntoNativeGraphicsPipelineCreateInfoStruct<'a>
	{
		let vshader = self.vertex_shader.as_ref().expect("VertexShader is required");
		let shader_stage_vec = vec![
			Some(make_shaderstage_data(vshader)), self.geometry_shader.as_ref().map(make_shaderstage_data),
			self.fragment_shader.as_ref().map(make_shaderstage_data)
		].into_iter().filter_map(|x| x).collect::<Vec<_>>();
		let shader_stage = shader_stage_vec.iter().map(make_native_shaderstage).collect();
		let (vports, scissors): (Vec<_>, Vec<_>) = self.vp_sc.iter().map(|&ViewportWithScissorRect(ref vp, ref sc)|
			unsafe { (transmute::<_, VkViewport>(vp.clone()), transmute::<_, VkRect2D>(sc.clone())) }).unzip();
		let attachment_blend_states = self.attachment_blend_states.iter().map(|&b| make_attachment_blend_state(b)).collect::<Vec<_>>();
		IntoNativeGraphicsPipelineCreateInfoStruct
		{
			into_shader_stage: shader_stage_vec,
			shader_stage: shader_stage,
			vertex_input_state: make_native_vistate_create_info(&(vshader.0).0.input_state),
			input_assembly_state: VkPipelineInputAssemblyStateCreateInfo
			{
				topology: self.primitive_topology.into(), .. Default::default()
			},
			viewport_state: VkPipelineViewportStateCreateInfo
			{
				viewportCount: vports.len() as u32, pViewports: vports.as_ptr(),
				scissorCount: scissors.len() as u32, pScissors: scissors.as_ptr(),
				.. Default::default()
			},
			rasterization_state: VkPipelineRasterizationStateCreateInfo
			{
				rasterizerDiscardEnable: self.fragment_shader.is_none() as VkBool32,
				polygonMode: if self.rasterizer_state.wired_render { VK_POLYGON_MODE_LINE } else { VK_POLYGON_MODE_FILL },
				cullMode: if let Some(side) = self.rasterizer_state.cull_side { side as _ } else { VK_CULL_MODE_NONE },
				.. Default::default()
			},
			multisample_state: VkPipelineMultisampleStateCreateInfo
			{
				rasterizationSamples: VK_SAMPLE_COUNT_1_BIT, alphaToCoverageEnable: self.use_alpha_to_coverage as VkBool32,
				.. Default::default()
			},
			color_blend_state: VkPipelineColorBlendStateCreateInfo
			{
				attachmentCount: attachment_blend_states.len() as u32, pAttachments: attachment_blend_states.as_ptr(),
				.. Default::default()
			},
			attachment_blend_states: attachment_blend_states,
			viewports: vports, scissors: scissors, base: self
		}
	}
}
impl<'a> Into<VkGraphicsPipelineCreateInfo> for &'a IntoNativeGraphicsPipelineCreateInfoStruct<'a>
{
	fn into(self) -> VkGraphicsPipelineCreateInfo
	{
		VkGraphicsPipelineCreateInfo
		{
			stageCount: self.shader_stage.len() as u32, pStages: self.shader_stage.as_ptr(),
			pVertexInputState: &self.vertex_input_state, pInputAssemblyState: &self.input_assembly_state,
			pViewportState: &self.viewport_state, pRasterizationState: &self.rasterization_state,
			pMultisampleState: &self.multisample_state, pColorBlendState: &self.color_blend_state,
			layout: self.base.layout.native(), renderPass: self.base.render_pass.native(), subpass: self.base.subpass_index,
			.. Default::default()
		}
	}
}

pub struct GraphicsPipeline(VkPipeline, Rc<Device>);
pub struct GraphicsPipelines(Vec<GraphicsPipeline>);
impl GraphicsPipelines
{
	pub fn new(engine: &GraphicsInterface, builders: &[&GraphicsPipelineBuilder]) -> EngineResult<Self>
	{
		let builders_n1 = builders.into_iter().map(|&x| x.into()).collect::<Vec<IntoNativeGraphicsPipelineCreateInfoStruct>>();
		let builders_n = builders_n1.iter().map(|x| x.into()).collect::<Vec<_>>();
		let mut pipelines = vec![unsafe { zeroed() }; builders.len()];
		unsafe { vkCreateGraphicsPipelines(engine.device().native(), unsafe { zeroed() }, builders_n.len() as _, builders_n.as_ptr(), null(), pipelines.as_mut_ptr()) }
			.make_result_with(|| GraphicsPipelines(pipelines.into_iter().map(|p| GraphicsPipeline(p, engine.device().clone())).collect()))
	}
}
impl Deref for GraphicsPipelines
{
	type Target = Vec<GraphicsPipeline>;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for GraphicsPipelines { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl AsRef<VkPipeline> for GraphicsPipeline { fn as_ref(&self) -> &VkPipeline { &self.0 } }
impl NativeHandleProvider for GraphicsPipeline
{
	type NativeT = VkPipeline;
	fn native(&self) -> VkPipeline { self.0 }
}
impl Drop for GraphicsPipeline
{
	fn drop(&mut self) { unsafe { vkDestroyPipeline(self.1.native(), self.0, null()) }; }
}
