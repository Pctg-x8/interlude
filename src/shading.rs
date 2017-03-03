///! Interlude: Primitive Shading(Shaders and Pipelines)

use {std, vk};
use vkdefs::*;
use std::ffi::CString;
use std::os::raw::c_char;
use super::*;
use rawexports::InternalExports;
use std::ops::Deref;
use std::io::prelude::*;
use std::rc::Rc;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum VertexBinding { PerVertex(u32), PerInstance(u32) }
#[derive(Clone, Debug, PartialEq)]
pub struct VertexAttribute(pub u32, pub VkFormat, pub u32);
pub struct IntoNativeVertexInputState
{
	bindings: Vec<VkVertexInputBindingDescription>,
	attributes: Vec<VkVertexInputAttributeDescription>
}

/// The structure that is part of Shader Program.
pub trait Shader : InternalExports<InternalT = vk::ShaderModule>
{
	fn as_stage_bits() -> VkShaderStageFlags;
	fn entry_point_ptr(&self) -> *const c_char;
}
pub struct VertexShader { internal: vk::ShaderModule, entry_point: CString, vertex_input: IntoNativeVertexInputState }
pub struct TessellationControlShader { internal: vk::ShaderModule, entry_point: CString }
pub struct TessellationEvaluationShader { internal: vk::ShaderModule, entry_point: CString }
pub struct GeometryShader { internal: vk::ShaderModule, entry_point: CString }
pub struct FragmentShader { internal: vk::ShaderModule, entry_point: CString }

impl Shader for VertexShader
{
	fn as_stage_bits() -> VkShaderStageFlags { VK_SHADER_STAGE_VERTEX_BIT }
	fn entry_point_ptr(&self) -> *const c_char { self.entry_point.as_ptr() }
}
impl Shader for TessellationControlShader
{
	fn as_stage_bits() -> VkShaderStageFlags { VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT }
	fn entry_point_ptr(&self) -> *const c_char { self.entry_point.as_ptr() }
}
impl Shader for TessellationEvaluationShader
{
	fn as_stage_bits() -> VkShaderStageFlags { VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT }
	fn entry_point_ptr(&self) -> *const c_char { self.entry_point.as_ptr() }
}
impl Shader for GeometryShader
{
	fn as_stage_bits() -> VkShaderStageFlags { VK_SHADER_STAGE_GEOMETRY_BIT }
	fn entry_point_ptr(&self) -> *const c_char { self.entry_point.as_ptr() }
}
impl Shader for FragmentShader
{
	fn as_stage_bits() -> VkShaderStageFlags { VK_SHADER_STAGE_FRAGMENT_BIT }
	fn entry_point_ptr(&self) -> *const c_char { self.entry_point.as_ptr() }
}
impl InternalExports for VertexShader { type InternalT = vk::ShaderModule; fn get_internal(&self) -> &vk::ShaderModule { &self.internal } }
impl InternalExports for TessellationControlShader { type InternalT = vk::ShaderModule; fn get_internal(&self) -> &vk::ShaderModule { &self.internal } }
impl InternalExports for TessellationEvaluationShader { type InternalT = vk::ShaderModule; fn get_internal(&self) -> &vk::ShaderModule { &self.internal } }
impl InternalExports for GeometryShader { type InternalT = vk::ShaderModule; fn get_internal(&self) -> &vk::ShaderModule { &self.internal } }
impl InternalExports for FragmentShader { type InternalT = vk::ShaderModule; fn get_internal(&self) -> &vk::ShaderModule { &self.internal } }
fn build_shader_module_from_file(engine: &GraphicsInterface, path: &Path) -> EngineResult<vk::ShaderModule>
{
	std::fs::File::open(path).and_then(|mut fp| { let mut vb = Vec::new(); fp.read_to_end(&mut vb).map(|_| vb) }).map_err(From::from)
		.and_then(|b| vk::ShaderModule::new(engine.device(), &b).map_err(From::from))
}
impl VertexShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str,
		bindings: &[VertexBinding], attributes: &[VertexAttribute]) -> EngineResult<Rc<Self>>
	{
		let fs_path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading Vertex Shader from {:?}...", fs_path);
		build_shader_module_from_file(engine, &fs_path).map(|m| Rc::new(VertexShader
		{
			internal: m, entry_point: CString::new(entry_point).unwrap(),
			vertex_input: IntoNativeVertexInputState
			{
				bindings: bindings.iter().enumerate().map(|(i, x)| match x
				{
					&VertexBinding::PerVertex(stride) => VkVertexInputBindingDescription(i as u32, stride, VkVertexInputRate::Vertex),
					&VertexBinding::PerInstance(stride) => VkVertexInputBindingDescription(i as u32, stride, VkVertexInputRate::Instance)
				}).collect(),
				attributes: attributes.iter().enumerate()
					.map(|(i, &VertexAttribute(binding, format, offset))| VkVertexInputAttributeDescription(i as u32, binding, format, offset))
					.collect()
			}
		}))
	}
}
impl TessellationControlShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Rc<Self>>
	{
		let fs_path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading Tessellation Control Shader from {:?}...", fs_path);
		build_shader_module_from_file(engine, &fs_path).map(|m| Rc::new(TessellationControlShader { internal: m, entry_point: CString::new(entry_point).unwrap() }))
	}
}
impl TessellationEvaluationShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Rc<Self>>
	{
		let fs_path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading Tessellation Evaluation Shader from {:?}...", fs_path);
		build_shader_module_from_file(engine, &fs_path).map(|m| Rc::new(TessellationEvaluationShader { internal: m, entry_point: CString::new(entry_point).unwrap() }))
	}
}
impl GeometryShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Rc<Self>>
	{
		let fs_path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading Geometry Shader from {:?}...", fs_path);
		build_shader_module_from_file(engine, &fs_path).map(|m| Rc::new(GeometryShader { internal: m, entry_point: CString::new(entry_point).unwrap() }))
	}
}
impl FragmentShader
{
	pub fn from_asset<Engine: AssetProvider + Deref<Target = GraphicsInterface>, P: AssetPath>(engine: &Engine, path: P, entry_point: &str) -> EngineResult<Rc<Self>>
	{
		let fs_path = engine.parse_asset(path, "spv");
		info!(target: "Interlude::ShaderProgram", "Loading Fragment Shader from {:?}...", fs_path);
		build_shader_module_from_file(engine, &fs_path).map(|m| Rc::new(FragmentShader { internal: m, entry_point: CString::new(entry_point).unwrap() }))
	}
}

#[derive(Clone)] pub struct PushConstantDesc(pub VkShaderStageFlags, pub std::ops::Range<u32>);
impl <'a> std::convert::Into<VkPushConstantRange> for &'a PushConstantDesc
{
	fn into(self) -> VkPushConstantRange
	{
		let PushConstantDesc(stage, ref range) = *self;
		VkPushConstantRange(stage, range.start, range.len() as u32)
	}
}

pub struct PipelineLayout(vk::PipelineLayout);
impl PipelineLayout
{
	pub fn new(engine: &GraphicsInterface, descriptor_set_layouts: &[&DescriptorSetLayout], push_constants: &[&PushConstantDesc]) -> EngineResult<Self>
	{
		let dsl = descriptor_set_layouts.into_iter().map(|&dsl| **dsl.get_internal()).collect::<Vec<_>>();
		let pc = push_constants.into_iter().map(|&pcd| pcd.into()).collect::<Vec<_>>();
		
		vk::PipelineLayout::new(engine.device(), &dsl, &pc).map(PipelineLayout).map_err(From::from)
	}
}
impl InternalExports for PipelineLayout { type InternalT = vk::PipelineLayout; fn get_internal(&self) -> &vk::PipelineLayout { &self.0 } }

// Primitive Topology + With-Adjacency flag
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveTopology
{
	Point, LineList(bool), LineStrip(bool), TriangleList(bool), TriangleStrip(bool)
}
impl std::convert::Into<VkPrimitiveTopology> for PrimitiveTopology
{
	fn into(self) -> VkPrimitiveTopology
	{
		match self
		{
			PrimitiveTopology::Point                => VkPrimitiveTopology::PointList,
			PrimitiveTopology::LineList(false)		=> VkPrimitiveTopology::LineList,
			PrimitiveTopology::LineList(true)		=> VkPrimitiveTopology::LineListWithAdjacency,
			PrimitiveTopology::LineStrip(false)		=> VkPrimitiveTopology::LineStrip,
			PrimitiveTopology::LineStrip(true)		=> VkPrimitiveTopology::LineStripWithAdjacency,
			PrimitiveTopology::TriangleList(false)	=> VkPrimitiveTopology::TriangleList,
			PrimitiveTopology::TriangleList(true)	=> VkPrimitiveTopology::TriangleListWithAdjacency,
			PrimitiveTopology::TriangleStrip(false)	=> VkPrimitiveTopology::TriangleStrip,
			PrimitiveTopology::TriangleStrip(true)	=> VkPrimitiveTopology::TriangleStripWithAdjacency
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CullingSide { Front, Back }
impl std::convert::Into<VkCullModeFlags> for CullingSide
{
	fn into(self) -> VkCullModeFlags
	{
		match self
		{
			CullingSide::Front => VK_CULL_MODE_FRONT_BIT,
			CullingSide::Back => VK_CULL_MODE_BACK_BIT
		}
	}
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
			&ConstantEntry::Float(v) => Vec::from(&unsafe { std::mem::transmute::<_, [u8; 4]>(v) }[..]),
			&ConstantEntry::Uint(v) => Vec::from(&unsafe { std::mem::transmute::<_, [u8; 4]>(v) }[..])
		}
	}
}
pub struct PipelineShaderProgram<Module: Shader>(pub Rc<Module>, pub Vec<(usize, ConstantEntry)>);
impl<Module: Shader> PipelineShaderProgram<Module>
{
	pub fn unspecialized(shref: &Rc<Module>) -> Self { PipelineShaderProgram(shref.clone(), Vec::new()) }
}
impl<Module: Shader> Clone for PipelineShaderProgram<Module>
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
		render_pass: PreciseRenderPass<'a>, fragment_shader: PipelineShaderProgram<FragmentShader>, processing_viewport: &Viewport) -> Result<Self, EngineError>
	{
		Ok(GraphicsPipelineBuilder
		{
			layout: layout, render_pass: render_pass.0, subpass_index: render_pass.1,
			vertex_shader: Some(PipelineShaderProgram::unspecialized(try!{engine.postprocess_vsh(true)})),
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
fn make_shaderstage_data<Module: Shader>(s: &PipelineShaderProgram<Module>) -> IntoNativeShaderStageCreateInfoStruct
{
	let (map_entries, const_values) = if s.1.is_empty() { (Vec::new(), Vec::new()) } else
	{
		let map_entries = s.1.iter().scan(0usize, |o, &(id, ref v)|
		{
			let size = v._sizeof();
			let rval = VkSpecializationMapEntry(id as u32, *o as u32, size);
			*o += size;
			Some(rval)
		}).collect::<Vec<_>>();
		let const_values = s.1.iter().flat_map(|&(_, ref v)| v.as_bytes().into_iter()).collect::<Vec<_>>();
		(map_entries, const_values)
	};

	IntoNativeShaderStageCreateInfoStruct
	{
		stage_bits: Module::as_stage_bits(),
		module: **s.0.get_internal(), entry_point: s.0.entry_point_ptr(),
		specialization_structure: if map_entries.is_empty() { None } else
		{
			Some(VkSpecializationInfo
			{
				mapEntryCount: map_entries.len() as u32, pMapEntries: map_entries.as_ptr(),
				dataSize: const_values.len() as usize, pData: const_values.as_ptr() as *const std::os::raw::c_void
			})
		}, specialization_entry: map_entries, specialization_values: const_values
	}
}
fn make_native_vistate_create_info(s: &IntoNativeVertexInputState) -> VkPipelineVertexInputStateCreateInfo
{
	VkPipelineVertexInputStateCreateInfo
	{
		sType: VkStructureType::Pipeline_VertexInputStateCreateInfo, pNext: std::ptr::null(), flags: 0,
		vertexBindingDescriptionCount: s.bindings.len() as u32, pVertexBindingDescriptions: s.bindings.as_ptr(),
		vertexAttributeDescriptionCount: s.attributes.len() as u32, pVertexAttributeDescriptions: s.attributes.as_ptr()
	}
}
fn make_native_shaderstage(s: &IntoNativeShaderStageCreateInfoStruct) -> VkPipelineShaderStageCreateInfo
{
	VkPipelineShaderStageCreateInfo
	{
		sType: VkStructureType::Pipeline_ShaderStageCreateInfo, pNext: std::ptr::null(), flags: 0,
		stage: s.stage_bits, module: s.module, pName: s.entry_point, pSpecializationInfo: s.specialization_structure.as_ref().map(|n| n as *const VkSpecializationInfo).unwrap_or_else(std::ptr::null)
	}
}
fn make_attachment_blend_state(s: AttachmentBlendState) -> VkPipelineColorBlendAttachmentState
{
	const COLOR_COMPONENT_ALL: VkColorComponentFlags = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;

	match s
	{
		AttachmentBlendState::Disabled => VkPipelineColorBlendAttachmentState
		{
			blendEnable: false as VkBool32, colorWriteMask: COLOR_COMPONENT_ALL, .. unsafe { std::mem::zeroed() }
		},
		AttachmentBlendState::AlphaBlend => VkPipelineColorBlendAttachmentState
		{
			blendEnable: true as VkBool32,
			srcColorBlendFactor: VkBlendFactor::SrcAlpha, dstColorBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
			srcAlphaBlendFactor: VkBlendFactor::One, dstAlphaBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
			colorBlendOp: VkBlendOp::Add, alphaBlendOp: VkBlendOp::Add, colorWriteMask: COLOR_COMPONENT_ALL
		},
		AttachmentBlendState::PremultipliedAlphaBlend => VkPipelineColorBlendAttachmentState
		{
			blendEnable: true as VkBool32,
			srcColorBlendFactor: VkBlendFactor::One, dstColorBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
			srcAlphaBlendFactor: VkBlendFactor::One, dstAlphaBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
			colorBlendOp: VkBlendOp::Add, alphaBlendOp: VkBlendOp::Add, colorWriteMask: COLOR_COMPONENT_ALL
		},
	}
}
impl<'a> std::convert::Into<IntoNativeGraphicsPipelineCreateInfoStruct<'a>> for &'a GraphicsPipelineBuilder<'a>
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
			unsafe { (std::mem::transmute::<_, VkViewport>(vp.clone()), std::mem::transmute::<_, VkRect2D>(sc.clone())) }).unzip();
		let attachment_blend_states = self.attachment_blend_states.iter().map(|&b| make_attachment_blend_state(b)).collect::<Vec<_>>();
		IntoNativeGraphicsPipelineCreateInfoStruct
		{
			into_shader_stage: shader_stage_vec,
			shader_stage: shader_stage,
			vertex_input_state: make_native_vistate_create_info(&vshader.0.vertex_input),
			input_assembly_state: VkPipelineInputAssemblyStateCreateInfo
			{
				sType: VkStructureType::Pipeline_InputAssemblyStateCreateInfo, pNext: std::ptr::null(), flags: 0,
				topology: self.primitive_topology.into(), primitiveRestartEnable: false as VkBool32
			},
			viewport_state: VkPipelineViewportStateCreateInfo
			{
				sType: VkStructureType::Pipeline_ViewportStateCreateInfo, pNext: std::ptr::null(), flags: 0,
				viewportCount: vports.len() as u32, pViewports: vports.as_ptr(),
				scissorCount: scissors.len() as u32, pScissors: scissors.as_ptr()
			},
			rasterization_state: VkPipelineRasterizationStateCreateInfo
			{
				sType: VkStructureType::Pipeline_RasterizationStateCreateInfo, pNext: std::ptr::null(), flags: 0,
				depthClampEnable: false as VkBool32, depthBiasEnable: false as VkBool32, rasterizerDiscardEnable: self.fragment_shader.is_none() as VkBool32,
				polygonMode: if self.rasterizer_state.wired_render { VkPolygonMode::Line } else { VkPolygonMode::Fill },
				cullMode: if let Some(side) = self.rasterizer_state.cull_side { side.into() } else { VK_CULL_MODE_NONE },
				frontFace: VkFrontFace::CounterClockwise,
				depthBiasConstantFactor: 0.0f32, depthBiasClamp: 0.0f32, depthBiasSlopeFactor: 0.0f32,
				lineWidth: 1.0f32
			},
			multisample_state: VkPipelineMultisampleStateCreateInfo
			{
				sType: VkStructureType::Pipeline_MultisampleStateCreateInfo, pNext: std::ptr::null(), flags: 0,
				rasterizationSamples: VK_SAMPLE_COUNT_1_BIT, sampleShadingEnable: false as VkBool32,
				minSampleShading: 0.0f32, pSampleMask: std::ptr::null(),
				alphaToCoverageEnable: self.use_alpha_to_coverage as VkBool32, alphaToOneEnable: false as VkBool32
			},
			color_blend_state: VkPipelineColorBlendStateCreateInfo
			{
				sType: VkStructureType::Pipeline_ColorBlendStateCreateInfo, pNext: std::ptr::null(), flags: 0,
				logicOpEnable: false as VkBool32, logicOp: VkLogicOp::NOP,
				attachmentCount: attachment_blend_states.len() as u32, pAttachments: attachment_blend_states.as_ptr(),
				blendConstants: [0.0f32; 4]
			},
			attachment_blend_states: attachment_blend_states,
			viewports: vports, scissors: scissors, base: self
		}
	}
}
impl<'a> std::convert::Into<VkGraphicsPipelineCreateInfo> for &'a IntoNativeGraphicsPipelineCreateInfoStruct<'a>
{
	fn into(self) -> VkGraphicsPipelineCreateInfo
	{
		VkGraphicsPipelineCreateInfo
		{
			sType: VkStructureType::GraphicsPipelineCreateInfo, pNext: std::ptr::null(), flags: 0,
			stageCount: self.shader_stage.len() as u32, pStages: self.shader_stage.as_ptr(),
			pVertexInputState: &self.vertex_input_state, pInputAssemblyState: &self.input_assembly_state,
			pTessellationState: std::ptr::null(), pViewportState: &self.viewport_state,
			pRasterizationState: &self.rasterization_state, pMultisampleState: &self.multisample_state,
			pDepthStencilState: std::ptr::null(), pColorBlendState: &self.color_blend_state,
			pDynamicState: std::ptr::null(),
			layout: *self.base.layout.0, renderPass: **self.base.render_pass.get_internal(), subpass: self.base.subpass_index,
			basePipelineHandle: std::ptr::null_mut(), basePipelineIndex: 0
		}
	}
}

pub struct GraphicsPipeline(vk::Pipeline);
pub struct GraphicsPipelines(Vec<GraphicsPipeline>);
impl GraphicsPipelines
{
	pub fn new(engine: &GraphicsInterface, builders: &[&GraphicsPipelineBuilder]) -> EngineResult<Self>
	{
		let builders_n = builders.into_iter().map(|&x| x.into()).collect::<Vec<_>>();
		vk::Pipeline::new_graphics(engine.device(), None, &builders_n.iter().map(Into::into).collect::<Vec<_>>())
			.map(|v| GraphicsPipelines(v.into_iter().map(GraphicsPipeline).collect())).map_err(From::from)
	}
}
impl Deref for GraphicsPipelines
{
	type Target = Vec<GraphicsPipeline>;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for GraphicsPipelines
{
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl InternalExports for GraphicsPipeline { type InternalT = vk::Pipeline; fn get_internal(&self) -> &vk::Pipeline { &self.0 } }
