// : Debug Printing

#![allow(mutable_transmutes)]
#![allow(dead_code)]

use super::internals::*;
use std;
use vk::ffi::*;
use std::collections::{LinkedList, HashMap};
use std::cell::RefCell;
use freetype_sys::*;
use std::ffi::CString;
use unicode_normalization::*;
use nalgebra::*;

const TEXTURE_SIZE: u32 = 512;
const DEBUG_LEFT_OFFSET: f32 = 6.0;

trait FreeTypeErrorHandler
{
	fn map<F, T>(self, f: F) -> Result<T, EngineError> where F: FnOnce() -> T;
	fn into_result(self) -> Result<(), EngineError>;
}
impl FreeTypeErrorHandler for FT_Error
{
	fn map<F, T>(self, f: F) -> Result<T, EngineError> where F: FnOnce() -> T
	{
		if self.succeeded() { Ok(f()) } else { Err(EngineError::from(self)) }
	}
	fn into_result(self) -> Result<(), EngineError>
	{
		if self.succeeded() { Ok(()) } else { Err(EngineError::from(self)) }
	}
}

// FreeType Provider
pub struct TypefaceProvider
{
	lib: FT_Library, face: FT_Face,
	pub baseline: i32, pub line_height: f32
}
impl TypefaceProvider
{
	fn new(engine: &Engine) -> Result<Self, EngineError>
	{
		let ftl = unsafe { let mut ptr: FT_Library = std::mem::uninitialized(); try!(FT_Init_FreeType(&mut ptr).map(|| ptr)) };
		let face = unsafe
		{
			let mut ptr: FT_Face = std::mem::uninitialized();
			let path = CString::new(engine.parse_asset("engine.fonts.OpenSans-Regular", "ttf").to_str().unwrap()).unwrap();
			try!(FT_New_Face(ftl, path.as_ptr(), 0, &mut ptr).map(|| ptr))
		};
		try!(unsafe { FT_Set_Char_Size(face, 0, 9 << 6, 100, 100).into_result() });

		Ok(TypefaceProvider
		{
			baseline: unsafe { &*face }.ascender as i32 * unsafe { &*(*face).size }.metrics.y_ppem as i32 / unsafe { &*face }.units_per_EM as i32,
			line_height: unsafe { &*face }.height as f32 * unsafe { &*(*face).size }.metrics.y_ppem as f32 / unsafe { &*face }.units_per_EM as f32,
			lib: ftl, face: face
		})
	}
	fn load_char(&self, chr: char) -> Result<(), EngineError>
	{
		unsafe { FT_Load_Char(self.face, chr as FT_ULong, FT_LOAD_RENDER).into_result() }
	}
	fn glyph_ref(&self) -> FT_GlyphSlot
	{
		unsafe { (*self.face).glyph }
	}
}
impl std::ops::Drop for TypefaceProvider
{
	fn drop(&mut self)
	{
		unsafe
		{
			FT_Done_Face(self.face);
			FT_Done_FreeType(self.lib);
		}
	}
}

// Shelf method
#[derive(Debug, Clone, Copy)]
pub struct TextureRegion
{
	u: f32, v: f32, uw: f32, vh: f32
}
#[derive(Clone, Copy)]
pub struct StrRenderData
{
	texcoord: TextureRegion,
	width: f32, height: f32, offset_from_baseline: f32, advance_left: f32
}
pub struct Horizon
{
	base_height: u32, maximum_height: u32, placement_left: u32
}
impl Horizon
{
	fn new(base_height: u32, init_height: u32, init_left: u32) -> Self
	{
		Horizon { base_height: base_height, maximum_height: init_height, placement_left: init_left }
	}
}

pub enum DebugLine<'a>
{
	Integer(String, &'a RefCell<i32>, Option<String>),
	UnsignedInt(String, &'a RefCell<u32>, Option<String>),
	Float(String, &'a RefCell<f64>, Option<String>)
}
impl <'a> DebugLine<'a>
{
	fn has_unit(&self) -> bool
	{
		match self
		{
			&DebugLine::Integer(_, _, ref opt) => opt.is_some(),
			&DebugLine::UnsignedInt(_, _, ref opt) => opt.is_some(),
			&DebugLine::Float(_, _, ref opt) => opt.is_some()
		}
	}
}
enum OptimizedDebugLine<'a>
{
	// cp_offs, base_y, param_name, value_ref, unit
	Integer(usize, f32, StrRenderData, &'a RefCell<i32>, Option<StrRenderData>),
	UnsignedInt(usize, f32, StrRenderData, &'a RefCell<u32>, Option<StrRenderData>),
	Float(usize, f32, StrRenderData, &'a RefCell<f64>, Option<StrRenderData>)
}
impl <'a> OptimizedDebugLine<'a>
{
	fn has_unit(&self) -> bool
	{
		match self
		{
			&OptimizedDebugLine::Integer(_, _, _, _, ref opt) => opt.is_some(),
			&OptimizedDebugLine::UnsignedInt(_, _, _, _, ref opt) => opt.is_some(),
			&OptimizedDebugLine::Float(_, _, _, _, ref opt) => opt.is_some()
		}
	}
	fn render_param_offset(&self) -> u32
	{
		match self
		{
			&OptimizedDebugLine::Integer(n, _, _, _, _) => n as u32,
			&OptimizedDebugLine::UnsignedInt(n, _, _, _, _) => n as u32,
			&OptimizedDebugLine::Float(n, _, _, _, _) => n as u32
		}
	}
}

// xoffs, yoffs, wscale hscale, uoffs, voffs, uscale, vscale
#[repr(C)] struct StrRenderInstanceData(f32, f32, f32, f32, f32, f32, f32, f32);
// x, y, z, w
#[repr(C)] struct Position(f32, f32, f32, f32);
type CMatrix4 = [[f32; 4]; 4];

pub struct DebugInfo<'a>
{
	dres_buf: DeviceBuffer, sres_buf: StagingBuffer,
	dres_image: DeviceImage, sres_image: StagingImage,
	optimized_lines: Vec<OptimizedDebugLine<'a>>,
	update_commands: TransferCommandBuffers,
	ds_layout: DescriptorSetLayout, playout: PipelineLayout,
	render_tech: GraphicsPipeline, descriptor_sets: DescriptorSets,
	vertex_offs: usize, instance_offs: usize, indirect_param_offs: usize,
	buffer_mapped_ptr: *mut std::os::raw::c_void,
	baseline: i32, rendering_params_count: usize,
	num_glyph_data: [StrRenderData; 12],
	transfer_completion: QueueFence,
	use_optimized_render: bool
}
pub trait DebugInfoInternals
{
	fn get_transfer_completion_qfence(&self) -> &QueueFence;
}
impl <'a> DebugInfoInternals for DebugInfo<'a>
{
	fn get_transfer_completion_qfence(&self) -> &QueueFence
	{
		&self.transfer_completion
	}
}
impl <'a> std::ops::Drop for DebugInfo<'a>
{
	fn drop(&mut self)
	{
		unsafe { MemoryMappedRange::from_raw(&self.sres_buf, self.buffer_mapped_ptr) };
	}
}
impl <'a> DebugInfo<'a>
{
	pub fn new(engine: &Engine, lines: &[DebugLine<'a>],
		rendered_pass: &RenderPass, subindex: u32, framebuffer_size: VkViewport) -> Result<Box<Self>, EngineError>
	{
		info!(target: "Interlude::DebugInfo", "Starting Visual Debugger...");

		let max_instance_count = lines.iter().fold(0usize, |acc, x| if x.has_unit() { acc + 2 + 8 } else { acc + 1 + 8 });
		let rendering_params_prealloc = engine.buffer_preallocate(&[
			(std::mem::size_of::<[Position; 4]>(), BufferDataType::Vertex),
			(std::mem::size_of::<IndirectCallParameter>() * lines.len(), BufferDataType::IndirectCallParam),
			(std::mem::size_of::<StrRenderInstanceData>() * max_instance_count, BufferDataType::Vertex),
			(std::mem::size_of::<CMatrix4>(), BufferDataType::Uniform)
		]);
		let texture_atlas_desc = ImageDescriptor2::new(VkFormat::R8_UNORM, VkExtent2D(TEXTURE_SIZE, TEXTURE_SIZE),
			ImageUsagePresets::AsColorTexture);
		let image_prealloc = ImagePreallocator::new()
			.image_2d(vec![&texture_atlas_desc]);
		let (bdev, bstage) = try!(engine.create_double_buffer(&rendering_params_prealloc));
		let (idev, istage) = try!(engine.create_double_image(&image_prealloc));
		let (idev, istage) = (idev, istage.unwrap());
		let sampler_state = SamplerState::new();
		let (image_view, sampler) = (
			try!(engine.create_image_view_2d(idev.dim2(0), VkFormat::R8_UNORM,
				ComponentMapping::single_swizzle(ComponentSwizzle::R), ImageSubresourceRange::base_color())),
			try!(engine.create_sampler(&sampler_state))
		);

		let (vshader, fshader) = (
			try!(engine.create_vertex_shader_from_asset("engine.shaders.DebugInfoV", "main", &[
				VertexBinding::PerVertex(std::mem::size_of::<Position>() as u32),
				VertexBinding::PerInstance(std::mem::size_of::<StrRenderInstanceData>() as u32)
			], &[
				VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0),
				VertexAttribute(1, VkFormat::R32G32B32A32_SFLOAT, 0),
				VertexAttribute(1, VkFormat::R32G32B32A32_SFLOAT, std::mem::size_of::<f32>() as u32 * 4)
			])),
			try!(engine.create_fragment_shader_from_asset("engine.shaders.DebugInfoF", "main"))
		);
		let ds_layout = try!(engine.create_descriptor_set_layout(&[
			Descriptor::Uniform(1, vec![ShaderStage::Vertex]),
			Descriptor::CombinedSampler(1, vec![ShaderStage::Fragment])
		]));
		let playout = try!(engine.create_pipeline_layout(&[&ds_layout], &[]));
		let pipeline = {
			let pipeline_builder = GraphicsPipelineBuilder::new(&playout, rendered_pass, subindex)
				.vertex_shader(PipelineShaderProgram::unspecialized(&vshader)).fragment_shader(PipelineShaderProgram::unspecialized(&fshader))
				.primitive_topology(PrimitiveTopology::TriangleStrip(false))
				.viewport_scissors(&[ViewportWithScissorRect::default_scissor(framebuffer_size)])
				.blend_state(&[AttachmentBlendState::PremultipliedAlphaBlend]);
			try!(engine.create_graphics_pipelines(&[&pipeline_builder])).remove(0)
		};
		let descriptor_sets = try!(engine.preallocate_all_descriptor_sets(&[&ds_layout]));
		engine.update_descriptors(&[
			DescriptorSetWriteInfo::UniformBuffer(descriptor_sets[0], 0,
				vec![BufferInfo(&bdev, rendering_params_prealloc.offset(3) .. rendering_params_prealloc.offset(4))]),
			DescriptorSetWriteInfo::CombinedImageSampler(descriptor_sets[0], 1,
				vec![ImageInfo(&sampler, &image_view, VkImageLayout::ShaderReadOnlyOptimal)])
		]);

		let typeface = try!(TypefaceProvider::new(engine));
		let mut glyph_coords = HashMap::new();
		let mut horizons = LinkedList::new();
		let mut optimized_lines = Vec::new();

		// Generate Textures and Rendering Params
		let mut num_glyph_data: [StrRenderData; 12] = unsafe { std::mem::uninitialized() };
		try!(bstage.map().and_then(|mapped_buf| istage.map().map(move |mapped| (mapped_buf, mapped))).and_then(|(mapped_buf, mapped)|
		{
			*mapped_buf.map_mut::<[Position; 4]>(rendering_params_prealloc.offset(0)) = [
				Position(0.0f32, 0.0f32, 0.0f32, 1.0f32),
				Position(0.0f32, 1.0f32, 0.0f32, 1.0f32),
				Position(1.0f32, 0.0f32, 0.0f32, 1.0f32),
				Position(1.0f32, 1.0f32, 0.0f32, 1.0f32)
			];
			let VkViewport(_, _, w, h, _, _) = framebuffer_size;
			let pp_matrix = OrthographicMatrix3::new(0.0f32, w as f32, 0.0f32, h as f32, -2.0f32, 1.0f32);
			*mapped_buf.map_mut::<CMatrix4>(rendering_params_prealloc.offset(3)) = *pp_matrix.as_matrix().transpose().as_ref();

			let rendering_params = mapped_buf.range_mut::<StrRenderInstanceData>(rendering_params_prealloc.offset(2), max_instance_count);
			let mapped_pixels = mapped.map_mut::<[u8; TEXTURE_SIZE as usize * TEXTURE_SIZE as usize]>(istage.image2d_offset(0) as usize);
			for (n, c) in "0123456789.-".nfc().enumerate()
			{
				try!(typeface.load_char(c));
				let gref = unsafe { &*typeface.glyph_ref() };
				let ref bitmap = gref.bitmap;
				let (xo, yo) = (gref.bitmap_left, gref.bitmap_top);
				let (width, height) = (bitmap.width, bitmap.rows);
				let bitmap_buffer = unsafe { std::slice::from_raw_parts(bitmap.buffer, bitmap.pitch as usize * bitmap.rows as usize) };
				let texcoord = try!(Self::allocate_rect(&mut horizons, VkExtent2D(bitmap.width as u32, bitmap.rows as u32)).ok_or(EngineError::GenericError("Unable to allocate region for number chars")));
				let coordinate_list = (0 .. width).flat_map(|x| (0 .. height).map(move |y| (x, y)))
					.map(|(x, y)| (x, y, bitmap_buffer[(x + y * bitmap.pitch) as usize]));
				for (x, y, p) in coordinate_list
				{
					mapped_pixels[((x as f32 + xo as f32 + texcoord.u * TEXTURE_SIZE as f32) + (y as f32 + texcoord.v * TEXTURE_SIZE as f32) * TEXTURE_SIZE as f32) as usize] = p;
				}
				num_glyph_data[n] = StrRenderData
				{
					texcoord: texcoord,
					width: (xo + width) as f32, height: height as f32,
					offset_from_baseline: yo as f32, advance_left: gref.advance.x as f32 / 64.0f32
				};
			}

			// Add Debug Lines //
			let call_params = mapped_buf.range_mut::<IndirectCallParameter>(rendering_params_prealloc.offset(1), lines.len());
			let mut rp_current_index = 0;
			let mut top = 4u32;
			let offset_mult = if engine.is_optimized_debug_render_support() { 1 } else { 0 };
			for (n, line) in lines.into_iter().enumerate()
			{
				let base = top as f32 + typeface.baseline as f32;
				optimized_lines.push(match line
				{
					&DebugLine::Integer(ref param, vref, ref unit) =>
					{
						let param_name = Self::string_entry(&mut glyph_coords, &typeface, &mut horizons, mapped_pixels, param.clone() + ": ");
						let unit_str = unit.as_ref().map(|x| Self::string_entry(&mut glyph_coords, &typeface, &mut horizons, mapped_pixels, x.clone()));
						let left = DEBUG_LEFT_OFFSET;
						let start_rp_index = rp_current_index;
						rendering_params[rp_current_index] = StrRenderInstanceData(left, base - param_name.offset_from_baseline, param_name.width, param_name.height, param_name.texcoord.u, param_name.texcoord.v, param_name.texcoord.uw, param_name.texcoord.vh);
						let left = left + param_name.advance_left.ceil();
						rp_current_index += 1;
						if let Some(unit_str) = unit_str
						{
							rendering_params[rp_current_index] = StrRenderInstanceData(left, base - unit_str.offset_from_baseline, unit_str.width, unit_str.height, unit_str.texcoord.u, unit_str.texcoord.v, unit_str.texcoord.uw, unit_str.texcoord.vh);
							rp_current_index += 1;
						}
						rp_current_index += 8;
						call_params[n] = IndirectCallParameter(4, rp_current_index as u32 - start_rp_index as u32 - 8, 0, start_rp_index as u32 * offset_mult);
						OptimizedDebugLine::Integer(start_rp_index, base, param_name, vref, unit_str)
					},
					&DebugLine::UnsignedInt(ref param, vref, ref unit) =>
					{
						let param_name = Self::string_entry(&mut glyph_coords, &typeface, &mut horizons, mapped_pixels, param.clone() + ": ");
						let unit_str = unit.as_ref().map(|x| Self::string_entry(&mut glyph_coords, &typeface, &mut horizons, mapped_pixels, x.clone()));
						let left = DEBUG_LEFT_OFFSET;
						let start_rp_index = rp_current_index;
						rendering_params[rp_current_index] = StrRenderInstanceData(left, base - param_name.offset_from_baseline, param_name.width, param_name.height, param_name.texcoord.u, param_name.texcoord.v, param_name.texcoord.uw, param_name.texcoord.vh);
						let left = left + param_name.advance_left.ceil();
						rp_current_index += 1;
						if let Some(unit_str) = unit_str
						{
							rendering_params[rp_current_index] = StrRenderInstanceData(left, base - unit_str.offset_from_baseline, unit_str.width, unit_str.height, unit_str.texcoord.u, unit_str.texcoord.v, unit_str.texcoord.uw, unit_str.texcoord.vh);
							rp_current_index += 1;
						}
						rp_current_index += 8;
						call_params[n] = IndirectCallParameter(4, rp_current_index as u32 - start_rp_index as u32 - 8, 0, start_rp_index as u32 * offset_mult);
						OptimizedDebugLine::UnsignedInt(start_rp_index, base, param_name, vref, unit_str)
					},
					&DebugLine::Float(ref param, vref, ref unit) =>
					{
						let param_name = Self::string_entry(&mut glyph_coords, &typeface, &mut horizons, mapped_pixels, param.clone() + ": ");
						let unit_str = unit.as_ref().map(|x| Self::string_entry(&mut glyph_coords, &typeface, &mut horizons, mapped_pixels, x.clone()));
						let left = DEBUG_LEFT_OFFSET;
						let start_rp_index = rp_current_index;
						rendering_params[rp_current_index] = StrRenderInstanceData(left, base - param_name.offset_from_baseline, param_name.width, param_name.height, param_name.texcoord.u, param_name.texcoord.v, param_name.texcoord.uw, param_name.texcoord.vh);
						let left = left + param_name.advance_left.ceil();
						rp_current_index += 1;
						if let Some(unit_str) = unit_str
						{
							rendering_params[rp_current_index] = StrRenderInstanceData(left, base - unit_str.offset_from_baseline, unit_str.width, unit_str.height, unit_str.texcoord.u, unit_str.texcoord.v, unit_str.texcoord.uw, unit_str.texcoord.vh);
							rp_current_index += 1;
						}
						rp_current_index += 8;
						call_params[n] = IndirectCallParameter(4, rp_current_index as u32 - start_rp_index as u32 - 8, 0, start_rp_index as u32 * offset_mult);
						OptimizedDebugLine::Float(start_rp_index, base, param_name, vref, unit_str)
					}
				});
				top += typeface.line_height.ceil() as u32;
			}
			Ok(())
		}));

		// setup updating commands //
		let update_commands = try!(engine.allocate_transfer_command_buffers(1));
		try!(update_commands.begin(0).and_then(|recorder|
		{
			let update_start_offs = rendering_params_prealloc.offset(1);
			let update_end_offs = rendering_params_prealloc.offset(3);
			let imb_stage_template = ImageMemoryBarrier::template(istage.dim2(0), ImageSubresourceRange::base_color());
			let imb_dev_template = ImageMemoryBarrier::template(&**idev.dim2(0), ImageSubresourceRange::base_color());
			let image_memory_barriers = [
				imb_stage_template.hold_ownership(VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_READ_BIT, VkImageLayout::General, VkImageLayout::TransferSrcOptimal),
				imb_dev_template.hold_ownership(VK_ACCESS_SHADER_READ_BIT, VK_ACCESS_TRANSFER_WRITE_BIT, VkImageLayout::ShaderReadOnlyOptimal, VkImageLayout::TransferDestOptimal)
			];
			let image_memory_barriers_ret = [
				imb_stage_template.hold_ownership(VK_ACCESS_TRANSFER_READ_BIT, VK_ACCESS_HOST_WRITE_BIT, VkImageLayout::TransferSrcOptimal, VkImageLayout::General),
				imb_dev_template.hold_ownership(VK_ACCESS_TRANSFER_WRITE_BIT, VK_ACCESS_SHADER_READ_BIT, VkImageLayout::TransferDestOptimal, VkImageLayout::ShaderReadOnlyOptimal)
			];
			let buffer_memory_barriers = [
				BufferMemoryBarrier::hold_ownership(&bstage, update_start_offs .. update_end_offs, VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_READ_BIT),
				BufferMemoryBarrier::hold_ownership(&bdev, update_start_offs .. update_end_offs, VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT | VK_ACCESS_UNIFORM_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_READ_BIT, VK_ACCESS_TRANSFER_WRITE_BIT)
			];
			let buffer_memory_barriers_ret = [
				BufferMemoryBarrier::hold_ownership(&bstage, update_start_offs .. update_end_offs, VK_ACCESS_TRANSFER_READ_BIT, VK_ACCESS_HOST_WRITE_BIT),
				BufferMemoryBarrier::hold_ownership(&bdev, update_start_offs .. update_end_offs, VK_ACCESS_TRANSFER_WRITE_BIT, VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT | VK_ACCESS_UNIFORM_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_READ_BIT)
			];

			recorder
				.pipeline_barrier(VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false,
					&[], &buffer_memory_barriers, &image_memory_barriers)
				.copy_image(istage.dim2(0), &**idev.dim2(0), VkImageLayout::TransferSrcOptimal, VkImageLayout::TransferDestOptimal,
					&[ImageCopyRegion(ImageSubresourceLayers::base_color(), VkOffset3D(0, 0, 0), ImageSubresourceLayers::base_color(), VkOffset3D(0, 0, 0), VkExtent3D(TEXTURE_SIZE, TEXTURE_SIZE, 1))])
				.copy_buffer(&bstage, &bdev, &[BufferCopyRegion(update_start_offs, update_start_offs, (update_end_offs - update_start_offs) as usize)])
				.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, false,
					&[], &buffer_memory_barriers_ret, &image_memory_barriers_ret)
			.end()
		}));

		// initial update //
		{
			let setup_commands = try!(engine.allocate_transient_transfer_command_buffers(1));

			try!(setup_commands.begin(0).and_then(|recorder|
			{
				let imb_stage_template = ImageMemoryBarrier::template(istage.dim2(0), ImageSubresourceRange::base_color());
				let imb_dev_template = ImageMemoryBarrier::template(&**idev.dim2(0), ImageSubresourceRange::base_color());
				let image_memory_barriers = [
					imb_stage_template.hold_ownership(VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_READ_BIT, VkImageLayout::Preinitialized, VkImageLayout::TransferSrcOptimal),
					imb_dev_template.hold_ownership(VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_WRITE_BIT, VkImageLayout::Preinitialized, VkImageLayout::TransferDestOptimal)
				];
				let image_memory_barriers_ret = [
					imb_stage_template.hold_ownership(VK_ACCESS_TRANSFER_READ_BIT, VK_ACCESS_HOST_WRITE_BIT, VkImageLayout::TransferSrcOptimal, VkImageLayout::General),
					imb_dev_template.hold_ownership(VK_ACCESS_TRANSFER_WRITE_BIT, VK_ACCESS_SHADER_READ_BIT, VkImageLayout::TransferDestOptimal, VkImageLayout::ShaderReadOnlyOptimal)
				];
				let buffer_memory_barriers = [
					BufferMemoryBarrier::hold_ownership(&bstage, 0 .. rendering_params_prealloc.total_size(), VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_READ_BIT),
					BufferMemoryBarrier::hold_ownership(&bdev, 0 .. rendering_params_prealloc.total_size(), VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_WRITE_BIT)
				];
				let buffer_memory_barriers_ret = [
					BufferMemoryBarrier::hold_ownership(&bstage, 0 .. rendering_params_prealloc.total_size(), VK_ACCESS_TRANSFER_READ_BIT, VK_ACCESS_HOST_WRITE_BIT),
					BufferMemoryBarrier::hold_ownership(&bdev, 0 .. rendering_params_prealloc.total_size(), VK_ACCESS_TRANSFER_WRITE_BIT, VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT | VK_ACCESS_UNIFORM_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_READ_BIT)
				];

				recorder
					.pipeline_barrier(VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false,
						&[], &buffer_memory_barriers, &image_memory_barriers)
					.copy_image(istage.dim2(0), &**idev.dim2(0), VkImageLayout::TransferSrcOptimal, VkImageLayout::TransferDestOptimal,
						&[ImageCopyRegion(ImageSubresourceLayers::base_color(), VkOffset3D(0, 0, 0), ImageSubresourceLayers::base_color(), VkOffset3D(0, 0, 0), VkExtent3D(TEXTURE_SIZE, TEXTURE_SIZE, 1))])
					.copy_buffer(&bstage, &bdev, &[BufferCopyRegion(0, 0, rendering_params_prealloc.total_size() as usize)])
					.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, false,
						&[], &buffer_memory_barriers_ret, &image_memory_barriers_ret)
				.end()
			}));
			try!(setup_commands.execute());
		}

		let buffer_mapped_ptr = try!(bstage.map().map(|x| unsafe { x.into_raw() }));

		Ok(Box::new(DebugInfo
		{
			dres_buf: bdev, sres_buf: bstage,
			dres_image: idev, sres_image: istage,
			optimized_lines: optimized_lines,
			update_commands: update_commands,
			ds_layout: ds_layout, playout: playout,
			render_tech: pipeline, descriptor_sets: descriptor_sets,
			vertex_offs: rendering_params_prealloc.offset(0),
			instance_offs: rendering_params_prealloc.offset(2),
			indirect_param_offs: rendering_params_prealloc.offset(1),
			buffer_mapped_ptr: buffer_mapped_ptr,
			baseline: typeface.baseline,
			rendering_params_count: max_instance_count,
			num_glyph_data: num_glyph_data,
			transfer_completion: try!(engine.create_queue_fence()),
			use_optimized_render: engine.is_optimized_debug_render_support()
		}))
	}
	fn allocate_rect(horizons: &mut LinkedList<Horizon>, rect: VkExtent2D) -> Option<TextureRegion>
	{
		let VkExtent2D(tw, th) = rect;

		fn recursive_find<'a, IterMut: 'a + std::iter::Iterator<Item=&'a mut Horizon>>(mut iter: IterMut, target: VkExtent2D) -> Option<TextureRegion>
		{
			let VkExtent2D(tw, th) = target;
			match iter.next()
			{
				Some(h) => if h.maximum_height >= th && h.placement_left + th <= TEXTURE_SIZE
				{
					// use this
					let new_left = h.placement_left;
					h.placement_left += tw;
					Some(TextureRegion
					{
						u: new_left as f32 / TEXTURE_SIZE as f32, v: h.base_height as f32 / TEXTURE_SIZE as f32,
						uw: tw as f32 / TEXTURE_SIZE as f32, vh: th as f32 / TEXTURE_SIZE as f32
					})
				}
				else { recursive_find(iter, target) },
				_ => None
			}
		}

		recursive_find(horizons.iter_mut(), rect).or_else(||
			// cannot find free space
			match horizons.back_mut()
			{
				Some(ref mut lh) if lh.placement_left + tw <= TEXTURE_SIZE =>
				{
					// use this with height expansion
					let new_left = lh.placement_left;
					lh.maximum_height = std::cmp::max(th, lh.maximum_height);
					lh.placement_left += tw;
					Some(TextureRegion
					{
						u: new_left as f32 / TEXTURE_SIZE as f32, v: lh.base_height as f32 / TEXTURE_SIZE as f32,
						uw: tw as f32 / TEXTURE_SIZE as f32, vh: th as f32 / TEXTURE_SIZE as f32
					})
				},
				_ => None
			}.or_else(||
			{
				// no available horizons found
				let new_base_height = if let Some(lh) = horizons.back() { lh.base_height + lh.maximum_height } else { 0 };
				if new_base_height + th < TEXTURE_SIZE
				{
					horizons.push_back(Horizon::new(new_base_height, th, tw));
					Some(TextureRegion
					{
						u: 0.0f32, v: new_base_height as f32 / TEXTURE_SIZE as f32,
						uw: tw as f32 / TEXTURE_SIZE as f32, vh: th as f32 / TEXTURE_SIZE as f32
					})
				}
				else { None }
			})
		)
	}
	fn string_entry(glyph_coords: &mut HashMap<String, StrRenderData>, typeface: &TypefaceProvider, horizons: &mut LinkedList<Horizon>,
		mapper: &mut [u8; (TEXTURE_SIZE * TEXTURE_SIZE) as usize], key: String) -> StrRenderData
	{
		*glyph_coords.entry(key.clone()).or_insert({
			let mut character_bitmaps: Vec<(Vec<u8>, i32, i32, i32, i32, i32, f32)> = Vec::new();
			let gref = unsafe { &*typeface.glyph_ref() };
			let ref bitmap = gref.bitmap;
			let (mut max_yo, mut max_desc) = (0, 0);
			let mut current_left = 0.0f32;
			for c in key.nfc()
			{
				typeface.load_char(c).unwrap();
				let (xo, yo) = (gref.bitmap_left, gref.bitmap_top);
				let (width, height) = (bitmap.width, bitmap.rows);
				let bitmap_buffer = unsafe { std::slice::from_raw_parts(bitmap.buffer, bitmap.pitch as usize * height as usize) };
				let mut new_buffer = vec![0u8; bitmap.pitch as usize * height as usize];
				new_buffer[..].copy_from_slice(bitmap_buffer);
				character_bitmaps.push((new_buffer, bitmap.pitch, xo, yo, width, height, current_left));
				max_yo = std::cmp::max(max_yo, yo);
				max_desc = std::cmp::max(max_desc, height - yo);
				current_left += gref.advance.x as f32 / 64.0f32;
			}
			let max_height = max_yo + max_desc;
			let &(_, _, xo, _, w, _, left) = character_bitmaps.iter().last().unwrap();
			let texcoord = Self::allocate_rect(horizons, VkExtent2D((left.ceil() as i32 + xo + w) as u32, max_height as u32)).unwrap();
			for (bmp, pitch, xo, yo, w, h, left) in character_bitmaps
			{
				let y_offs = max_yo - yo;
				let coords = (0 .. w).flat_map(|x| (0 .. h)
					.map(move |y| (x, y, (texcoord.u * TEXTURE_SIZE as f32) as i32 + left as i32 + xo + x, (texcoord.v * TEXTURE_SIZE as f32) as i32 + y + y_offs)));
				for (bx, by, dx, dy) in coords
				{
					mapper[(dx + dy * TEXTURE_SIZE as i32) as usize] = bmp[(bx + by * pitch) as usize];
				}
			}
			StrRenderData
			{
				texcoord: texcoord,
				width: left.ceil() + xo as f32 + w as f32, height: max_height as f32, offset_from_baseline: max_yo as f32, advance_left: current_left
			}
		})
	}

	pub fn inject_render_commands<ComRec: DrawingCommandRecorder>(&self, recorder: ComRec) -> ComRec
	{
		let recorder = recorder
			.bind_pipeline(&self.render_tech)
			.bind_descriptor_sets(&self.playout, &self.descriptor_sets[0..1])
			.bind_vertex_buffers(&[(&self.dres_buf, self.vertex_offs), (&self.dres_buf, self.instance_offs)]);

		if self.use_optimized_render
		{
			recorder.draw_indirect_mult(&self.dres_buf, self.indirect_param_offs, self.optimized_lines.len() as u32)
		}
		else
		{
			let mut recorder_state = recorder;
			for (n, l) in self.optimized_lines.iter().enumerate()
			{
				recorder_state = recorder_state
					.bind_vertex_buffers_partial(1, &[(&self.dres_buf, self.instance_offs + std::mem::size_of::<StrRenderInstanceData>() * l.render_param_offset() as usize)])
					.draw_indirect(&self.dres_buf, self.indirect_param_offs + n * std::mem::size_of::<IndirectCallParameter>());
			}
			recorder_state
		}
	}

	#[allow(dead_code)]
	pub fn update(&self)
	{
		let (call_params, rendering_params) = unsafe
		{
			(std::slice::from_raw_parts_mut(self.buffer_mapped_ptr.offset(self.indirect_param_offs as isize) as *mut IndirectCallParameter,
				self.optimized_lines.len()),
			std::slice::from_raw_parts_mut(self.buffer_mapped_ptr.offset(self.instance_offs as isize) as *mut StrRenderInstanceData,
				self.rendering_params_count))
		};
		let offset_mult = if self.use_optimized_render { 1 } else { 0 };
		for (line, call_param) in self.optimized_lines.iter().zip(call_params)
		{
			// max 8 characters
			let mut character_indices = [0usize; 8];
			let (ybase, rp_index, param_advance, has_unit, total_len) = match line
			{
				&OptimizedDebugLine::Integer(start_rp_index, base, ref param, vref, ref unit) => (base, start_rp_index, param.advance_left, unit.is_some(), if *vref.borrow() == 0
				{
					character_indices[0] = 0;
					1
				}
				else
				{
					// maximum 7 digits + sign
					let is_minus = *vref.borrow() < 0;
					let mut ipart_abs = if is_minus { -*vref.borrow() } else { *vref.borrow() } % 10_000_000;
					let total_len = if is_minus { 1 } else { 0 } + (ipart_abs as f32).log(10.0f32) as usize + 1;
					let mut current_ptr = total_len;
					while ipart_abs > 0
					{
						current_ptr -= 1;
						character_indices[current_ptr] = (ipart_abs % 10) as usize;
						ipart_abs = ipart_abs / 10;
					}
					if is_minus { character_indices[0] = 11; }		// minus sign
					total_len
				}),
				&OptimizedDebugLine::UnsignedInt(start_rp_index, base, ref param, vref, ref unit) => (base, start_rp_index, param.advance_left, unit.is_some(), if *vref.borrow() == 0
				{
					character_indices[0] = 0;
					1
				}
				else
				{
					// maximum 8 digits
					let mut ipart = *vref.borrow() % 100_000_000;
					let total_len = (ipart as f32).log(10.0f32) as usize + 1;
					let mut current_ptr = total_len;
					while ipart > 0
					{
						current_ptr -= 1;
						character_indices[current_ptr] = (ipart % 10) as usize;
						ipart = ipart / 10;
					}
					total_len
				}),
				&OptimizedDebugLine::Float(start_rp_index, base, ref param, vref, ref unit) => (base, start_rp_index, param.advance_left, unit.is_some(),
				{
					// maximum 3.3 digits + sign
					let is_minus = vref.borrow().is_sign_negative();
					let (mut ipart_abs, mut fpart) = (vref.borrow().abs().floor() as u32 % 1_000, *vref.borrow() - vref.borrow().floor());
					let first_idx = if is_minus { 1 } else { 0 };
					let ipart_len = if ipart_abs == 0
					{
						character_indices[first_idx] = 0;
						1
					}
					else
					{
						let ipart_len = (ipart_abs as f32).log(10.0f32) as usize + 1;
						let mut current_ptr = ipart_len;
						while ipart_abs > 0
						{
							current_ptr -= 1;
							character_indices[first_idx + current_ptr] = (ipart_abs % 10) as usize;
							ipart_abs = ipart_abs / 10;
						}
						ipart_len
					};
					let cont_len = if fpart > 0.0f64
					{
						character_indices[first_idx + ipart_len] = 10;		// period
						let mut fplen = 0;
						while fpart > 0.0f64 && fplen < 3
						{
							fplen += 1;
							fpart *= 10.0f64;
							character_indices[first_idx + ipart_len + fplen] = fpart as usize % 10;
							fpart -= fpart.floor();
						}
						fplen + 1
					}
					else { 0 };
					if is_minus { character_indices[0] = 11; }		// minus

					first_idx + ipart_len + cont_len
				})
			};

			// build digit characters and update call parameter
			let header_offs = if has_unit { 2 } else { 1 };
			let mut left = DEBUG_LEFT_OFFSET + param_advance.ceil();
			for (n, ci) in character_indices[..total_len].into_iter().enumerate()
			{
				let ref glyph_data = self.num_glyph_data[*ci];
				rendering_params[rp_index + header_offs + n] = StrRenderInstanceData(left, ybase - glyph_data.offset_from_baseline,
					glyph_data.width, glyph_data.height, glyph_data.texcoord.u, glyph_data.texcoord.v,
					glyph_data.texcoord.uw, glyph_data.texcoord.vh);
				left += glyph_data.advance_left;
			}
			if has_unit { rendering_params[rp_index + 1].0 = left; }
			*call_param = IndirectCallParameter(4, (header_offs + total_len) as u32, 0, line.render_param_offset() * offset_mult);
		}
	}

	pub fn get_transfer_commands(&self) -> &TransferCommandBuffers
	{
		&self.update_commands
	}
}
