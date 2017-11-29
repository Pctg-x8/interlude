
extern crate interlude;
use interlude::*;

const VERTEX_FORMAT: Format = Format::Component(32, PackedPixelOrder::RGBA, FormatType::Float);

fn main() { game().or_crash(); }
#[allow(unused_variables)]
fn game() -> EngineResult<()>
{
	let color_subres = ImageSubresourceRange { aspect: ImageAspect::Color.into(), .. Default::default() };

	let engine = EngineBuilder::<EmptyInput>::new("com.cterm2.interlude.examples.triangle".into(), (0, 1, 0), "Rendering Triangle".into(), &Size2(640, 480))
		.asset_base(std::env::current_dir().unwrap().into()).launch()?;

	// make buffer and staging data
	let bp = BufferPreallocator::new(&engine, &[BufferContent::Vertex(std::mem::size_of::<[[CVector4; 2]; 3]>())]);
	let (dev, stg) = bp.instantiate()?;
	stg.map().map(|m|
	{
		*m.map_mut::<[[CVector4; 2]; 3]>(bp.offset(0)) = [
			[[0.0, -0.25, 0.5, 1.0], [1.0, 1.0, 1.0, 1.0]],
			[[0.375, 0.25, 0.5, 1.0], [1.0, 0.0, 0.0, 1.0]],
			[[-0.375, 0.25, 0.5, 1.0], [0.0, 1.0, 1.0, 1.0]]
		];
	})?;

	// make Framebuffer
	let vport = Viewport::from(engine.render_window().size());
	let fb = engine.render_window().render_targets().iter().map(|v| Framebuffer::new_for_presented(&engine, v, Some(true), engine.render_window().size()))
		.collect::<EngineResult<Vec<_>>>()?;

	// load shaders and build pipeline state
	let vshader = VertexShader::from_asset(&engine, "examples.triangle.vert", "main",
		&[VertexBinding::PerVertex(std::mem::size_of::<[CVector4; 2]>() as u32)],
		&[VertexAttribute(0, VERTEX_FORMAT, 0), VertexAttribute(0, VERTEX_FORMAT, std::mem::size_of::<CVector4>() as u32)])?;
	let fshader = FragmentShader::from_asset(&engine, "engine.shaders.TrivialFragment", "main")?;
	let psl = PipelineLayout::new(&engine, &[], &[])?;
	let ps_mold = GraphicsPipelineBuilder::new(&psl, PreciseRenderPass(fb[0].renderpass(), 0))
		.primitive_topology(PrimitiveTopology::TriangleList(false))
		.vertex_shader(PipelineShaderProgram::unspecialized(&vshader))
		.fragment_shader(PipelineShaderProgram::unspecialized(&fshader))
		.viewport_scissors(&[ViewportWithScissorRect::default_scissor(&vport)])
		.blend_state(&[AttachmentBlendState::Disabled]);
	let ps = GraphicsPipelines::new(&engine, &[&ps_mold])?.pop().unwrap();

	// Transfer data / Setting image layout
	{
		let bmbarriers = [
			BufferMemoryBarrier { buffer: &stg, range: 0 .. bp.total_size(), dst_access: AccessFlag::TransferRead.into(), .. Default::default() },
			BufferMemoryBarrier { buffer: &dev, range: 0 .. bp.total_size(), dst_access: AccessFlag::TransferWrite.into(), .. Default::default() }
		];
		ImmediateTransferCommandSubmission::begin(&engine)?
			.pipeline_barrier_on(PipelineStage::Transfer, false, &[], &bmbarriers, &engine.render_window().render_targets().iter()
				.map(|x| ImageMemoryBarrier::initialize_undef(x, color_subres.clone(), AccessFlag::MemoryRead.into(), ImageLayout::PresentSrc))
				.collect::<Vec<_>>())
			.copy_buffer(&stg, &dev, &[BufferCopyRegion(0, 0, bp.total_size())])
			.pipeline_barrier_on(PipelineStage::Transfer, false, &[],
				&[BufferMemoryBarrier { src_access: AccessFlag::VertexAttributeRead.into(), .. bmbarriers[1].clone() }.flip()], &[]);
	}

	// Forward Presenting(For Intel Graphics)
	/*info!("Forward Presenting...");
	{
		let ordersem = engine.create_queue_fence()?;
		wframe.acquire_next_backbuffer_index(&ordersem).and_then(|index|
		{
			engine.allocate_transient_graphics_command_buffers(1).and_then(|fp_commands|
			{
				let imbarrier = ImageMemoryBarrier::hold_ownership(wframe.get_back_images()[index as usize], ImageSubresourceRange::base_color(), VK_ACCESS_MEMORY_READ_BIT, VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
					VkImageLayout::PresentSrcKHR, VkImageLayout::ColorAttachmentOptimal);
				try!(fp_commands.begin(0).and_then(|recorder| recorder
					.pipeline_barrier(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, false, &[], &[], &[imbarrier])
					.begin_render_pass(&fb[index as usize], &[AttachmentClearValue::Color(0.0, 0.0, 0.0, 1.0)], false).end_render_pass().end()
				));
				fp_commands.execute(Some((&ordersem, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)))
			}).map(|_| index)
		}).and_then(|index| wframe.present(engine.graphics_queue_ref(), index, None))?;
		engine.wait_device()?;
	}*/

	// Draw commands and submit them
	let ordersem = QueueFence::new(&engine)?;
	let render_completion = QueueFence::new(&engine)?;
	let index = engine.render_window().acquire_next_target_index(&ordersem)? as usize;
	let gc = ImmediateGraphicsCommandSubmission::begin(&engine)?
		.pipeline_barrier_on(PipelineStage::ColorAttachmentOutput, false, &[], &[], &[
			ImageMemoryBarrier
			{
				image: &engine.render_window().render_targets()[index], subresource_range: color_subres.clone(),
				src_access: AccessFlag::MemoryRead.into(), dst_access: AccessFlag::ColorAttachmentWrite.into(),
				src_layout: ImageLayout::PresentSrc, dst_layout: ImageLayout::ColorAttachmentOptimal, .. Default::default()
			}
		])
		.begin_render_pass(&fb[index], &[AttachmentClearValue::Color(0.0, 0.0, 0.0, 1.0)], false)
		.bind_pipeline(&ps)
		.bind_vertex_buffers(&[(&dev, bp.offset(0))])
		.draw(3, 1)
		.end_render_pass()
		.submit_opt(&[(&ordersem, &PipelineStage::ColorAttachmentOutput)], Some(&render_completion), None)?;
	engine.render_window().present(&engine, index as _, Some(&render_completion))?;

	engine.process_all_messages();
	engine.wait_device()
}
