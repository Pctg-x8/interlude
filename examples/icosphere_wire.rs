
extern crate interlude;
extern crate thread_scoped;
extern crate nalgebra;
extern crate time;
use interlude::*;
use interlude::ffi::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use nalgebra::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)] enum InputNames { A }

fn main()
{
	let engine = Engine::new("com.cterm2.interlude.examples.icosphere_wire", 0x01,
		Some(std::env::current_dir().unwrap()), DeviceFeatures::new().enable_nonsolid_fillmode()).or_crash();
	let target = engine.create_render_window(&Size2(640, 480), "Rendering Icosphere[Wireframe]").or_crash();

	// make framebuffer
	let Size2(w, h) = target.size();
	let vport = Viewport::from(target.size());
	let fb = target.get_back_images().iter().map(|&v| engine.create_presented_framebuffer(v, Some(true), &Size3(w, h, 1))).collect::<Result<Vec<_>, _>>().or_crash();

	let bp = engine.buffer_preallocate(&[
		(std::mem::size_of::<[CMatrix4; 2]>(), BufferDataType::Uniform),
		// (std::mem::size_of::<[CVector4; 12]>(), BufferDataType::Vertex),
		// (std::mem::size_of::<[[u16; 3]; 20]>(), BufferDataType::Index)
		(std::mem::size_of::<[[CVector4; 3]; 80]>(), BufferDataType::Vertex)
	]);
	let (dev, stg) = engine.create_double_buffer(&bp).or_crash();
	stg.map().map(|m|
	{
		let (v, i) = generate_icosphere();
		m.map_mut::<[[CVector4; 3]; 80]>(bp.offset(1)).copy_from_slice(&subdiv_icosahedron(associate_vertex_indices(&v, &i))[..]);
		// *m.map_mut::<[CVector4; 12]>(bp.offset(1)) = v;
		// *m.map_mut::<[[u16; 3]; 20]>(bp.offset(2)) = i;
		let proj = PerspectiveMatrix3::new(w as f32 / h as f32, 30.0f32.to_radians(), 0.1, 100.0).to_matrix() *
			view_matrix(Vector3::new(5.0, 2.0, 30.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
		*m.map_mut::<[CMatrix4; 2]>(bp.offset(0)) = [*proj.as_ref(), *Rotation3::new(Vector3::new(0.0, 1.0, 0.0).normalize() * 0.0).submatrix().to_homogeneous().as_ref()];
	}).or_crash();

	// load shaders and build pipeline state
	let vshader = engine.create_vertex_shader_from_asset("examples.icosphere.vert", "main",
		&[VertexBinding::PerVertex(std::mem::size_of::<CVector4>() as u32)], &[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0)]).or_crash();
	let fshader = engine.create_fragment_shader_from_asset("examples.icosphere.frag", "main").or_crash();
	let dsl_cam = engine.create_descriptor_set_layout(&[Descriptor::Uniform(1, vec![ShaderStage::Vertex])]).or_crash();
	let psl = engine.create_pipeline_layout(&[&dsl_cam], &[]).or_crash();
	let ps_mold = GraphicsPipelineBuilder::new(&psl, fb[0].renderpass(), 0)
		.primitive_topology(PrimitiveTopology::TriangleList(false))
		.vertex_shader(PipelineShaderProgram::unspecialized(&vshader))
		.rasterizer_enable_wired_mode()
		.viewport_scissors(&[ViewportWithScissorRect::default_scissor(&vport)])
		.fragment_shader(PipelineShaderProgram(&fshader, vec![
			(0, ConstantEntry::Float(1.0)),
			(1, ConstantEntry::Float(1.0)),
			(2, ConstantEntry::Float(1.0)),
			(3, ConstantEntry::Float(1.0))
		]))
		.blend_state(&[AttachmentBlendState::Disabled]);
	let ps = engine.create_graphics_pipelines(&[&ps_mold]).or_crash().pop().unwrap();

	// create descriptor sets
	let descriptor_sets = engine.preallocate_all_descriptor_sets(&[&dsl_cam]).or_crash();
	let ubuf_info = BufferInfo(&dev, bp.offset(0) .. bp.offset(1));
	engine.update_descriptors(&[DescriptorSetWriteInfo::UniformBuffer(descriptor_sets[0], 0, vec![ubuf_info])]);

	// transfer data / setting image layout
	engine.allocate_transient_transfer_command_buffers(1).and_then(|setup_commands|
	{
		let bmbarriers = [
			BufferMemoryBarrier::hold_ownership(&stg, 0 .. bp.total_size(), 0, VK_ACCESS_TRANSFER_READ_BIT),
			BufferMemoryBarrier::hold_ownership(&dev, 0 .. bp.total_size(), 0, VK_ACCESS_TRANSFER_WRITE_BIT)
		];
		let bmbarrier_ret = BufferMemoryBarrier::hold_ownership(&dev, 0 .. bp.total_size(), VK_ACCESS_TRANSFER_WRITE_BIT,
			VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT | VK_ACCESS_INDEX_READ_BIT | VK_ACCESS_UNIFORM_READ_BIT);
		let imbarriers = target.get_back_images().iter().map(|&x|
			ImageMemoryBarrier::hold_ownership(x, ImageSubresourceRange::base_color(), 0, VK_ACCESS_MEMORY_READ_BIT, VkImageLayout::Undefined, VkImageLayout::PresentSrcKHR)
		).collect::<Vec<_>>();

		try!(setup_commands.begin(0).and_then(|recorder| recorder
			.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false, &[], &bmbarriers, &imbarriers)
			.copy_buffer(&stg, &dev, &[BufferCopyRegion(0, 0, bp.total_size())])
			.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false, &[], &[bmbarrier_ret], &[])
		.end()));
		setup_commands.execute()
	}).or_crash();

	// Draw commands and submit it
	let cb = engine.allocate_graphics_command_buffers(target.backimage_count()).or_crash();
	for (n, recorder) in cb.begin_all().or_crash()
	{
		let clear_value = AttachmentClearValue::Color(0.0, 0.0, 0.0, 1.0);
		let imbarrier_rt = ImageMemoryBarrier::hold_ownership(target.get_back_images()[n], ImageSubresourceRange::base_color(), VK_ACCESS_MEMORY_READ_BIT, VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
			VkImageLayout::PresentSrcKHR, VkImageLayout::ColorAttachmentOptimal);
		recorder
			.pipeline_barrier(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, true, &[], &[], &[imbarrier_rt])
			.begin_render_pass(&fb[n], &[clear_value], false)
			.bind_pipeline(&ps)
			.bind_descriptor_sets(&psl, &[descriptor_sets[0]])
			.bind_vertex_buffers(&[(&dev, bp.offset(1))])
			// .bind_index_buffer(&dev, bp.offset(2))
			// .draw_indexed(20 * 3, 1, 0)
			.draw(80 * 3, 1)
			.end_render_pass()
		.end().or_crash();
	}

	// Update commands
	let ucb = engine.allocate_transfer_command_buffers(1).or_crash();
	ucb.begin(0).and_then(|recorder|
	{
		let bmbarrier = [
			BufferMemoryBarrier::hold_ownership(&stg, bp.offset(0) + std::mem::size_of::<CMatrix4>() .. bp.offset(1), VK_ACCESS_HOST_WRITE_BIT, VK_ACCESS_TRANSFER_READ_BIT),
			BufferMemoryBarrier::hold_ownership(&dev, bp.offset(0) + std::mem::size_of::<CMatrix4>() .. bp.offset(1),
				VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT | VK_ACCESS_INDEX_READ_BIT | VK_ACCESS_UNIFORM_READ_BIT, VK_ACCESS_TRANSFER_WRITE_BIT)
		];
		let bmbarrier_ret = bmbarrier.iter().map(|x| x.flipped_access_mask()).collect::<Vec<_>>();
		recorder.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false, &[], &bmbarrier, &[])
			.copy_buffer(&stg, &dev, &[BufferCopyRegion(bp.offset(0) + std::mem::size_of::<CMatrix4>(), bp.offset(0) + std::mem::size_of::<CMatrix4>(), bp.offset(1))])
			.pipeline_barrier(VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, false, &[], &bmbarrier_ret, &[])
		.end()
	}).or_crash();

	// dummy key setting
	if let Ok(mut isw) = engine.input_system_ref().write()
	{
		isw.add_input(InputNames::A, InputType::Key(InputKeys::Character('z')));
	}

	// Process Loop(Saving CPU usage)
	{
		let window_system = engine.window_system_ref().clone();
		let ordersem = engine.create_queue_fence().or_crash();
		let copy_completion = engine.create_fence().or_crash();
		let render_completion = engine.create_fence().or_crash();
		let exit_signal = Arc::new(AtomicBool::new(false));
		let exit_signal_uo = exit_signal.clone();
		let update_event = Arc::new(Event::new("Update Event").or_crash());
		let update_event_uo = update_event.clone();
		let update_observer = unsafe { thread_scoped::scoped(move ||
		{
			let mut frame_index = target.acquire_next_backbuffer_index(&ordersem).and_then(|f|
				engine.submit_graphics_commands(&[cb[f as usize]], &[(&ordersem, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)],
					None, Some(&render_completion)).map(|_| f)
				).or_crash();
			while !exit_signal_uo.load(Ordering::Acquire)
			{
				render_completion.wait().and_then(|()| render_completion.clear()).or_crash();
				engine.submit_transfer_commands(&ucb[..], &[], None, Some(&copy_completion)).or_crash();
				copy_completion.wait().and_then(|()| copy_completion.clear()).or_crash();
				update_event_uo.set();
				frame_index = target.present(engine.graphics_queue_ref(), frame_index, None).and_then(|_|
					target.acquire_next_backbuffer_index(&ordersem).and_then(|f|
						engine.submit_graphics_commands(&[cb[f as usize]], &[(&ordersem, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)],
							None, Some(&render_completion)).map(|_| f)
						)
					).or_crash();
			}

			engine.wait_device().or_crash();
			engine
		}) };

		let mapped = stg.map().or_crash();
		let mut model_rot = mapped.map_mut::<CMatrix4>(bp.offset(0) + std::mem::size_of::<CMatrix4>());
		let start_time = time::PreciseTime::now();
		loop
		{
			match window_system.process_events_and_messages(&[&update_event])
			{
				ApplicationState::Exited => break,
				ApplicationState::EventArrived(0) =>
				{
					update_event.reset();
					let elapsed = start_time.to(time::PreciseTime::now());
					*model_rot = *Rotation3::new(Vector3::new(0.0, 1.0, 0.0).normalize() * (300.0f32 * elapsed.num_microseconds().unwrap() as f32 / 1_000_000.0f32).to_radians())
						.submatrix().to_homogeneous().as_ref();
				},
				_ => ()
			}
		}
		exit_signal.store(true, Ordering::Release);
		update_observer.join()
	};
}

/// Interlude:drafting Generate Icosphere mesh and indices
fn generate_icosphere() -> ([CVector4; 12], [[u16; 3]; 20])
{
	let t = (1.0 + 5.0f32.sqrt()) / 2.0;
	let vertices = [
		[-1.0, t, 0.0, 1.0], [1.0, t, 0.0, 1.0], [-1.0, -t, 0.0, 1.0], [1.0, -t, 0.0, 1.0],
		[0.0, -1.0, t, 1.0], [0.0, 1.0, t, 1.0], [0.0, -1.0, -t, 1.0], [0.0, 1.0, -t, 1.0],
		[t, 0.0, -1.0, 1.0], [t, 0.0, 1.0, 1.0], [-t, 0.0, -1.0, 1.0], [-t, 0.0, 1.0, 1.0]
	];

	(vertices, [
		[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
		[1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
		[3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
		[4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]
	])
}
fn associate_vertex_indices<T: Copy>(v: &[T], i: &[[u16; 3]]) -> Vec<[T; 3]>
{
	i.into_iter().map(|iv| [v[iv[0] as usize], v[iv[1] as usize], v[iv[2] as usize]]).collect()
}
fn icosahedron_middle(a: CVector4, b: CVector4) -> CVector4
{
	let temp_v = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5, (a[3] + b[3]) * 0.5];
	let temp_vlen = (temp_v[0].powf(2.0) + temp_v[1].powf(2.0) + temp_v[2].powf(2.0)).sqrt();
	let temp_v = [temp_v[0] / temp_vlen, temp_v[1] / temp_vlen, temp_v[2] / temp_vlen, temp_v[3]];
	let gratio = (1.0 + 5.0f32.sqrt()) / 2.0;
	let offs = (gratio * gratio + 1.0).sqrt();
	[temp_v[0] * offs, temp_v[1] * offs, temp_v[2] * offs, temp_v[3]]
}
fn subdiv_triangle(v: [CVector4; 3]) -> [[CVector4; 3]; 4]
{
	let newv = (icosahedron_middle(v[0], v[1]), icosahedron_middle(v[1], v[2]), icosahedron_middle(v[2], v[0]));

	[
		[v[0], newv.0, newv.2],
		[newv.0, v[1], newv.1],
		[newv.2, newv.1, v[2]],
		[newv.0, newv.1, newv.2]
	]
}
fn subdiv_icosahedron(v: Vec<[CVector4; 3]>) -> Vec<[CVector4; 3]>
{
	v.into_iter().flat_map(|v| Vec::from(&subdiv_triangle(v)[..]).into_iter()).collect()
}

fn view_matrix<N: BaseFloat>(eye: Vector3<N>, target: Vector3<N>, up: Vector3<N>) -> Matrix4<N>
{
	let zaxis = (eye - target).normalize();
	let xaxis = up.cross(&zaxis).normalize();
	let yaxis = zaxis.cross(&xaxis);

	Matrix4::new(xaxis.x, xaxis.y, xaxis.z, -eye.dot(&xaxis),
		yaxis.x, yaxis.y, yaxis.z, -eye.dot(&yaxis),
		zaxis.x, zaxis.y, zaxis.z, -eye.dot(&zaxis),
		N::zero(), N::zero(), N::zero(), N::one())
}
