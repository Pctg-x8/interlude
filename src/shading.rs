// Prelude: Primitive Shading(Shaders and Pipelines)

use {std, vk};
use vk::ffi::*;
use vk::traits::*;
use std::ffi::CString;
use super::internals::*;

pub struct VertexInputState(Vec<VertexBinding>, Vec<VertexAttribute>);
#[derive(Clone)]
pub enum VertexBinding
{
	PerVertex(u32), PerInstance(u32)
}
#[derive(Clone)]
pub struct VertexAttribute(pub u32, pub VkFormat, pub u32);
pub struct IntoNativeVertexInputState
{
	bindings: Vec<VkVertexInputBindingDescription>,
	attributes: Vec<VkVertexInputAttributeDescription>
}
impl <'a> std::convert::Into<VkPipelineVertexInputStateCreateInfo> for &'a IntoNativeVertexInputState
{
	fn into(self) -> VkPipelineVertexInputStateCreateInfo
	{
		VkPipelineVertexInputStateCreateInfo
		{
			sType: VkStructureType::Pipeline_VertexInputStateCreateInfo, pNext: std::ptr::null(), flags: 0,
			vertexBindingDescriptionCount: self.bindings.len() as u32, pVertexBindingDescriptions: self.bindings.as_ptr(),
			vertexAttributeDescriptionCount: self.attributes.len() as u32, pVertexAttributeDescriptions: self.attributes.as_ptr()
		}
	}
}

pub enum ShaderProgram
{
	Vertex { internal: vk::ShaderModule, entry_point: CString, vertex_input: VertexInputState },
	#[allow(dead_code)] TessControl { internal: vk::ShaderModule, entry_point: CString },
	#[allow(dead_code)] TessEvaluate { internal: vk::ShaderModule, entry_point: CString },
	Geometry { internal: vk::ShaderModule, entry_point: CString },
	Fragment { internal: vk::ShaderModule, entry_point: CString }
}
pub trait ShaderProgramInternals
{
	fn new_vertex(module: vk::ShaderModule, entry_point: &str, vbindings: &[VertexBinding], vattributes: &[VertexAttribute]) -> Self;
	fn new_geometry(module: vk::ShaderModule, entry_point: &str) -> Self;
	fn new_fragment(module: vk::ShaderModule, entry_point: &str) -> Self;
	fn get_entry_point(&self) -> &CString;
	fn into_native_vertex_input_state(&self) -> IntoNativeVertexInputState;
}
impl InternalExports<vk::ShaderModule> for ShaderProgram
{
	fn get_internal(&self) -> &vk::ShaderModule
	{
		match self
		{
			&ShaderProgram::Vertex { internal: ref e, entry_point: _, vertex_input: _ } => e,
			&ShaderProgram::Geometry { internal: ref e, entry_point: _ } => e,
			&ShaderProgram::Fragment { internal: ref e, entry_point: _ } => e,
			&ShaderProgram::TessControl { internal: ref e, entry_point: _ } => e,
			&ShaderProgram::TessEvaluate { internal: ref e, entry_point: _ } => e
		}
	}
}
impl ShaderProgramInternals for ShaderProgram
{
	fn new_vertex(module: vk::ShaderModule, entry_point: &str, vbindings: &[VertexBinding], vattributes: &[VertexAttribute]) -> Self
	{
		ShaderProgram::Vertex
		{
			internal: module, entry_point: CString::new(entry_point).unwrap(),
			vertex_input: VertexInputState(Vec::from(vbindings), Vec::from(vattributes))
		}
	}
	fn new_geometry(module: vk::ShaderModule, entry_point: &str) -> Self
	{
		ShaderProgram::Geometry { internal: module, entry_point: CString::new(entry_point).unwrap() }
	}
	fn new_fragment(module: vk::ShaderModule, entry_point: &str) -> Self
	{
		ShaderProgram::Fragment { internal: module, entry_point: CString::new(entry_point).unwrap() }
	}
	fn get_entry_point(&self) -> &CString
	{
		match self
		{
			&ShaderProgram::Vertex { internal: _, entry_point: ref e, vertex_input: _ } => e,
			&ShaderProgram::Geometry { internal: _, entry_point: ref e } => e,
			&ShaderProgram::Fragment { internal: _, entry_point: ref e } => e,
			&ShaderProgram::TessControl { internal: _, entry_point: ref e } => e,
			&ShaderProgram::TessEvaluate { internal: _, entry_point: ref e } => e
		}
	}
	fn into_native_vertex_input_state(&self) -> IntoNativeVertexInputState
	{
		if let &ShaderProgram::Vertex { internal: _, entry_point: _, vertex_input: VertexInputState(ref vb, ref va) } = self
		{
			IntoNativeVertexInputState
			{
				bindings: vb.iter().enumerate().map(|(i, x)| match x
				{
					&VertexBinding::PerVertex(stride) => VkVertexInputBindingDescription(i as u32, stride, VkVertexInputRate::Vertex),
					&VertexBinding::PerInstance(stride) => VkVertexInputBindingDescription(i as u32, stride, VkVertexInputRate::Instance)
				}).collect(),
				attributes: va.iter().enumerate()
					.map(|(i, &VertexAttribute(binding, format, offset))| VkVertexInputAttributeDescription(i as u32, binding, format, offset))
					.collect()
			}
		}
		else { panic!("Unable to create vertex input state from the exception of vertex shader") }
	}
}

#[derive(Clone)]
pub struct PushConstantDesc(pub VkShaderStageFlags, pub std::ops::Range<u32>);
impl <'a> std::convert::Into<VkPushConstantRange> for &'a PushConstantDesc
{
	fn into(self) -> VkPushConstantRange
	{
		let PushConstantDesc(stage, ref range) = *self;
		VkPushConstantRange(stage, range.start, range.len() as u32)
	}
}

pub struct PipelineLayout { internal: vk::PipelineLayout }
pub trait PipelineLayoutInternals { fn new(pl: vk::PipelineLayout) -> Self; }
impl PipelineLayoutInternals for PipelineLayout
{
	fn new(pl: vk::PipelineLayout) -> Self { PipelineLayout { internal: pl } }
}
impl InternalExports<vk::PipelineLayout> for PipelineLayout
{
	fn get_internal(&self) -> &vk::PipelineLayout { &self.internal }
}

// Primitive Topology + With-Adjacency flag
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct ViewportWithScissorRect(VkViewport, VkRect2D);
impl ViewportWithScissorRect
{
	pub fn default_scissor(vp: VkViewport) -> Self
	{
		let VkViewport(vx, vy, vw, vh, _, _) = vp;
		ViewportWithScissorRect(vp, VkRect2D(VkOffset2D(vx as i32, vy as i32), VkExtent2D(vw as u32, vh as u32)))
	}
}
#[derive(Clone, Copy)]
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
#[derive(Clone)]
pub struct RasterizerState
{
	pub wired_render: bool, pub cull_side: Option<CullingSide>
}
#[derive(Clone, Copy)]
pub enum AttachmentBlendState
{
	Disabled, AlphaBlend, PremultipliedAlphaBlend
}
impl std::convert::Into<VkPipelineColorBlendAttachmentState> for AttachmentBlendState
{
	fn into(self) -> VkPipelineColorBlendAttachmentState
	{
		match self
		{
			AttachmentBlendState::Disabled => VkPipelineColorBlendAttachmentState
			{
				blendEnable: false as VkBool32,
				srcColorBlendFactor: VkBlendFactor::One, dstColorBlendFactor: VkBlendFactor::One,
				srcAlphaBlendFactor: VkBlendFactor::One, dstAlphaBlendFactor: VkBlendFactor::One,
				colorBlendOp: VkBlendOp::Add, alphaBlendOp: VkBlendOp::Add,
				colorWriteMask: VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT
			},
			AttachmentBlendState::AlphaBlend => VkPipelineColorBlendAttachmentState
			{
				blendEnable: true as VkBool32,
				srcColorBlendFactor: VkBlendFactor::SrcAlpha, dstColorBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
				srcAlphaBlendFactor: VkBlendFactor::One, dstAlphaBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
				colorBlendOp: VkBlendOp::Add, alphaBlendOp: VkBlendOp::Add,
				colorWriteMask: VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT
			},
			AttachmentBlendState::PremultipliedAlphaBlend => VkPipelineColorBlendAttachmentState
			{
				blendEnable: true as VkBool32,
				srcColorBlendFactor: VkBlendFactor::One, dstColorBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
				srcAlphaBlendFactor: VkBlendFactor::One, dstAlphaBlendFactor: VkBlendFactor::OneMinusSrcAlpha,
				colorBlendOp: VkBlendOp::Add, alphaBlendOp: VkBlendOp::Add,
				colorWriteMask: VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT
			}
		}
	}
}

#[derive(Clone)]
pub enum ConstantEntry
{
	Float(f32), Uint(u32)
}
#[derive(Clone)]
pub struct PipelineShaderProgram<'a>(pub &'a ShaderProgram, pub Vec<(usize, ConstantEntry)>);
impl<'a> PipelineShaderProgram<'a>
{
	pub fn unspecialized(shref: &'a ShaderProgram) -> Self { PipelineShaderProgram(shref, Vec::new()) }
}
pub struct IntoNativeShaderStageCreateInfoStruct
{
	stage_bits: VkShaderStageFlags, module: VkShaderModule, entry_point: *const i8,
	#[allow(dead_code)] specialization_entry: Vec<VkSpecializationMapEntry>,
	#[allow(dead_code)] specialization_values: Vec<u8>,
	specialization_structure: Option<VkSpecializationInfo>
}
impl<'a> std::convert::Into<IntoNativeShaderStageCreateInfoStruct> for &'a PipelineShaderProgram<'a>
{
	fn into(self) -> IntoNativeShaderStageCreateInfoStruct
	{
		let (map_entries, const_values) = if self.1.is_empty() { (Vec::new(), Vec::new()) } else
		{
			let map_entries = self.1.iter().scan(0usize, |acc, &(ref id, ref v)|
			{
				let size = match v
				{
					&ConstantEntry::Float(_) | &ConstantEntry::Uint(_) => 4
				};
				let rval = VkSpecializationMapEntry(*id as u32, *acc as u32, size);
				*acc += size;
				Some(rval)
			}).collect::<Vec<_>>();
			let const_size = map_entries.last().map(|&VkSpecializationMapEntry(_, o, s)| o + s as u32).unwrap();
			let mut const_values = Vec::with_capacity(const_size as usize);
			for &(_, ref v) in &self.1
			{
				const_values.append(&mut match v
				{
					&ConstantEntry::Float(v) => Vec::from(&unsafe { std::mem::transmute::<_, [u8; 4]>(v) }[..]),
					&ConstantEntry::Uint(v) => Vec::from(&unsafe { std::mem::transmute::<_, [u8; 4]>(v) }[..])
				});
			}
			(map_entries, const_values)
		};

		IntoNativeShaderStageCreateInfoStruct
		{
			stage_bits: match self.0
			{
				&ShaderProgram::Vertex { .. } => VK_SHADER_STAGE_VERTEX_BIT,
				&ShaderProgram::Geometry { .. } => VK_SHADER_STAGE_GEOMETRY_BIT,
				&ShaderProgram::Fragment { .. } => VK_SHADER_STAGE_FRAGMENT_BIT,
				&ShaderProgram::TessControl { .. } => VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
				&ShaderProgram::TessEvaluate { .. } => VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT
			},
			module: self.0.get_internal().get(), entry_point: self.0.get_entry_point().as_ptr(),
			specialization_structure: if map_entries.is_empty() { None } else { Some(VkSpecializationInfo
			{
				mapEntryCount: map_entries.len() as u32, pMapEntries: map_entries.as_ptr(),
				dataSize: const_values.len(), pData: const_values.as_ptr() as *const std::os::raw::c_void
			})},
			specialization_entry: map_entries, specialization_values: const_values
		}
	}
}
impl<'a> std::convert::Into<VkPipelineShaderStageCreateInfo> for &'a IntoNativeShaderStageCreateInfoStruct
{
	fn into(self) -> VkPipelineShaderStageCreateInfo
	{
		VkPipelineShaderStageCreateInfo
		{
			sType: VkStructureType::Pipeline_ShaderStageCreateInfo, pNext: std::ptr::null(), flags: 0,
			stage: self.stage_bits, module: self.module, pName: self.entry_point, pSpecializationInfo: self.specialization_structure.as_ref().map(|n| n as *const VkSpecializationInfo).unwrap_or(std::ptr::null())
		}
	}
}
pub struct GraphicsPipelineBuilder<'a>
{
	layout: &'a PipelineLayout, render_pass: &'a RenderPass, subpass_index: u32,
	vertex_shader: Option<PipelineShaderProgram<'a>>, geometry_shader: Option<PipelineShaderProgram<'a>>, fragment_shader: Option<PipelineShaderProgram<'a>>,
	primitive_topology: PrimitiveTopology, vp_sc: Vec<ViewportWithScissorRect>,
	rasterizer_state: RasterizerState, use_alpha_to_coverage: bool, attachment_blend_states: Vec<AttachmentBlendState>
}
impl <'a> GraphicsPipelineBuilder<'a>
{
	pub fn new(layout: &'a PipelineLayout, render_pass: &'a RenderPass, subpass_index: u32) -> Self
	{
		GraphicsPipelineBuilder
		{
			layout: layout, render_pass: render_pass, subpass_index: subpass_index,
			vertex_shader: None, geometry_shader: None, fragment_shader: None,
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
			vertex_shader: base.vertex_shader.clone(), geometry_shader: base.geometry_shader.clone(), fragment_shader: base.fragment_shader.clone(),
			primitive_topology: base.primitive_topology, vp_sc: base.vp_sc.clone(), rasterizer_state: base.rasterizer_state.clone(),
			use_alpha_to_coverage: base.use_alpha_to_coverage, attachment_blend_states: base.attachment_blend_states.clone()
		}
	}
	pub fn for_postprocess(engine: &'a Engine, layout: &'a PipelineLayout, render_pass: &'a RenderPass, subpass_index: u32,
		fragment_shader: PipelineShaderProgram<'a>, processing_viewport: VkViewport) -> Self
	{
		GraphicsPipelineBuilder
		{
			layout: layout, render_pass: render_pass, subpass_index: subpass_index,
			vertex_shader: Some(PipelineShaderProgram::unspecialized(&engine.postprocess_vsh)), geometry_shader: None, fragment_shader: Some(fragment_shader),
			primitive_topology: PrimitiveTopology::TriangleStrip(false),
			vp_sc: vec![ViewportWithScissorRect::default_scissor(processing_viewport)],
			rasterizer_state: RasterizerState { wired_render: false, cull_side: None },
			use_alpha_to_coverage: false, attachment_blend_states: vec![AttachmentBlendState::Disabled]
		}
	}

	pub fn vertex_shader(mut self, vshader: PipelineShaderProgram<'a>) -> Self
	{
		match vshader
		{
			PipelineShaderProgram(&ShaderProgram::Vertex { .. }, _) => { self.vertex_shader = Some(vshader); self },
			_ => panic!("Prelude Assertion: GraphicsPIpelineBuilder::geometry_shader is called with not a geometry shader")
		}
	}
	pub fn geometry_shader(mut self, gshader: PipelineShaderProgram<'a>) -> Self
	{
		match gshader
		{
			PipelineShaderProgram(&ShaderProgram::Geometry { .. }, _) => { self.geometry_shader = Some(gshader); self },
			_ => panic!("Prelude Assertion: GraphicsPIpelineBuilder::geometry_shader is called with not a geometry shader")
		}
	}
	pub fn fragment_shader(mut self, fshader: PipelineShaderProgram<'a>) -> Self
	{
		match fshader
		{
			PipelineShaderProgram(&ShaderProgram::Fragment { .. }, _) => { self.fragment_shader = Some(fshader); self },
			_ => panic!("Prelude Assertion: GraphicsPIpelineBuilder::fragment_shader is called with not a fragment shader")
		}
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
	#[allow(dead_code)] into_vertex_input_state: IntoNativeVertexInputState,
	#[allow(dead_code)] into_shader_stage: Vec<IntoNativeShaderStageCreateInfoStruct>,
	shader_stage: Vec<VkPipelineShaderStageCreateInfo>,
	vertex_input_state: VkPipelineVertexInputStateCreateInfo,
	input_assembly_state: VkPipelineInputAssemblyStateCreateInfo,
	viewport_state: VkPipelineViewportStateCreateInfo,
	rasterization_state: VkPipelineRasterizationStateCreateInfo,
	multisample_state: VkPipelineMultisampleStateCreateInfo,
	color_blend_state: VkPipelineColorBlendStateCreateInfo
}
impl <'a> std::convert::Into<IntoNativeGraphicsPipelineCreateInfoStruct<'a>> for &'a GraphicsPipelineBuilder<'a>
{
	fn into(self) -> IntoNativeGraphicsPipelineCreateInfoStruct<'a>
	{
		let vshader = self.vertex_shader.as_ref().expect("VertexShader is required");
		let mut shader_stage_vec = vec![Into::into(vshader)];
		if let Some(ref gs) = self.geometry_shader { shader_stage_vec.push(Into::into(gs)); }
		if let Some(ref fs) = self.fragment_shader { shader_stage_vec.push(Into::into(fs)); }
		let shader_stage = shader_stage_vec.iter().map(Into::into).collect();
		let into_input_state = vshader.0.into_native_vertex_input_state();
		let vports = self.vp_sc.iter().map(|&ViewportWithScissorRect(vp, _)| vp).collect::<Vec<_>>();
		let scissors = self.vp_sc.iter().map(|&ViewportWithScissorRect(_, sc)| sc).collect::<Vec<_>>();
		let attachment_blend_states = self.attachment_blend_states.iter().map(|&x| x.into()).collect::<Vec<_>>();
		IntoNativeGraphicsPipelineCreateInfoStruct
		{
			into_shader_stage: shader_stage_vec,
			shader_stage: shader_stage,
			vertex_input_state: (&into_input_state).into(),
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
			into_vertex_input_state: into_input_state, attachment_blend_states: attachment_blend_states,
			viewports: vports, scissors: scissors, base: self
		}
	}
}
impl <'a> std::convert::Into<VkGraphicsPipelineCreateInfo> for &'a IntoNativeGraphicsPipelineCreateInfoStruct<'a>
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
			layout: self.base.layout.internal.get(), renderPass: self.base.render_pass.get_internal().get(), subpass: self.base.subpass_index,
			basePipelineHandle: std::ptr::null_mut(), basePipelineIndex: 0
		}
	}
}

pub struct GraphicsPipeline { internal: vk::Pipeline }
pub trait GraphicsPipelineInternals { fn new(p: vk::Pipeline) -> Self; }
impl GraphicsPipelineInternals for GraphicsPipeline
{
	fn new(p: vk::Pipeline) -> Self { GraphicsPipeline { internal: p } }
}
impl InternalExports<vk::Pipeline> for GraphicsPipeline
{
	fn get_internal(&self) -> &vk::Pipeline { &self.internal }
}
