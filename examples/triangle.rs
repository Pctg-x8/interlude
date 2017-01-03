
extern crate interlude;
#[macro_use] extern crate log;
use interlude::*;
use interlude::ffi::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)] enum InputNames { A }

fn main()
{
	let engine = Engine::new("com.cterm2.interlude.examples.triangle", 0x01, Some(std::env::current_dir().unwrap()), DeviceFeatures::new()).or_crash();
	let wframe = engine.create_render_window(&Size2(640, 480), "Rendering Triangle").or_crash();

	// make buffer and staging data
	let bp = engine.buffer_preallocate(&[(std::mem::size_of::<[[CVector4; 2]; 3]>(), BufferDataType::Vertex)]);
	let (dev, stg) = engine.create_double_buffer(&bp).or_crash();
	stg.map().map(|m|
	{
		*m.map_mut::<[[CVector4; 2]; 3]>(bp.offset(0)) = [
			[[0.0, -0.25, 0.5, 1.0], [1.0, 1.0, 1.0, 1.0]],
			[[0.375, 0.25, 0.5, 1.0], [1.0, 0.0, 0.0, 1.0]],
			[[-0.375, 0.25, 0.5, 1.0], [0.0, 1.0, 1.0, 1.0]]
		];
	}).or_crash();

	// define RenderPass and make Framebuffer
	let Size2(w, h) = wframe.size();
	let vport = Viewport::from(wframe.size());
	let fb = wframe.get_back_images().iter().map(|&v| engine.create_presented_framebuffer(v, Some(true), &Size3(w, h, 1))).collect::<Result<Vec<_>, _>>().or_crash();

	// load shaders and build pipeline state
	let vshader = engine.create_vertex_shader_from_asset("examples.triangle.vert", "main", &[VertexBinding::PerVertex(std::mem::size_of::<[CVector4; 2]>() as u32)],
		&[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0), VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, std::mem::size_of::<CVector4>() as u32)]).or_crash();
	let fshader = engine.create_fragment_shader_from_asset("engine.shaders.TrivialFragment", "main").or_crash();
	let psl = engine.create_pipeline_layout(&[], &[]).or_crash();
	let ps_mold = interlude::GraphicsPipelineBuilder::new(&psl, fb[0].renderpass(), 0)
		.primitive_topology(PrimitiveTopology::TriangleList(false))
		.vertex_shader(PipelineShaderProgram::unspecialized(&vshader))
		.fragment_shader(PipelineShaderProgram::unspecialized(&fshader))
		.viewport_scissors(&[ViewportWithScissorRect::default_scissor(&vport)])
		.blend_state(&[AttachmentBlendState::Disabled]);
	let ps = engine.create_graphics_pipelines(&[&ps_mold]).or_crash().pop().unwrap();

	// Transfer data / Setting image layout
	engine.allocate_transient_transfer_command_buffers(1).and_then(|setup_commands|
	{
		let bmbarriers = [
			BufferMemoryBarrier::hold_ownership(&stg, 0 .. bp.total_size(), 0, VK_ACCESS_TRANSFER_READ_BIT),
			BufferMemoryBarrier::hold_ownership(&dev, 0 .. bp.total_size(), 0, VK_ACCESS_TRANSFER_WRITE_BIT)
		];
		let bmbarrier_ret = BufferMemoryBarrier::hold_ownership(&dev, 0 .. bp.total_size(), VK_ACCESS_TRANSFER_WRITE_BIT, VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT);
		let imbarriers = wframe.get_back_images().iter()
			.map(|&x| ImageMemoryBarrier::hold_ownership(x, ImageSubresourceRange::base_color(), 0, VK_ACCESS_MEMORY_READ_BIT, VkImageLayout::Undefined, VkImageLayout::PresentSrcKHR))
			.collect::<Vec<_>>();
		
		try!(setup_commands.begin(0).and_then(|recorder| recorder
			.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false, &[], &bmbarriers, &imbarriers)
			.copy_buffer(&stg, &dev, &[BufferCopyRegion(0, 0, bp.total_size())])
			.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false, &[], &[bmbarrier_ret], &[])
		.end()));
		setup_commands.execute()
	}).or_crash();

	// Forward Presenting(For Intel Graphics)
	/*info!("Forward Presenting...");
	{
		let ordersem = engine.create_queue_fence().or_crash();
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
		}).and_then(|index| wframe.present(engine.graphics_queue_ref(), index, None)).or_crash();
		engine.wait_device().or_crash();
	}*/

	// Draw commands and submit it
	let cb = engine.allocate_graphics_command_buffers(wframe.backimage_count()).or_crash();
	for (n, recorder) in cb.begin_all().or_crash()
	{
		let clear_value = AttachmentClearValue::Color(0.0, 0.0, 0.0, 1.0);
		let imbarrier_rt = ImageMemoryBarrier::hold_ownership(wframe.get_back_images()[n], ImageSubresourceRange::base_color(), VK_ACCESS_MEMORY_READ_BIT, VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
			VkImageLayout::PresentSrcKHR, VkImageLayout::ColorAttachmentOptimal);
		recorder
			.pipeline_barrier(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, false, &[], &[], &[imbarrier_rt])
			.begin_render_pass(&fb[n], &[clear_value], false)
			.bind_pipeline(&ps)
			.bind_vertex_buffers(&[(&dev, bp.offset(0))])
			.draw(3, 1)
			.end_render_pass()
		.end().or_crash();
	}

	let ordersem = engine.create_queue_fence().or_crash();
	let render_completion = engine.create_queue_fence().or_crash();
	wframe.acquire_next_backbuffer_index(&ordersem).and_then(|index|
		engine.submit_graphics_commands(&[cb[index as usize]], &[(&ordersem, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)], Some(&render_completion), None).map(|_| index)
	).and_then(|index| wframe.present(engine.graphics_queue_ref(), index, Some(&render_completion)).map(|_| index)).or_crash();

	// dummy key setting
	if let Ok(mut isw) = engine.input_system_ref().write()
	{
		isw.add_input(InputNames::A, InputType::Key(InputKeys::Character('z')));
	}

	engine.process_all_messages();
	engine.wait_device().or_crash();
}
