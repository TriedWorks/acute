use acute::prelude::*;
use acute::vulkan::internal::{ash::vk, shader_macros::include_glsl, *};
use std::mem;

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        pos: [-1.0, 1.0, 0.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 0.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        pos: [0.0, -1.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
];

const VERTEX_CODE: &[u32] = include_glsl!("assets/test.vert");
const FRAG_CODE: &[u32] = include_glsl!("assets/test.frag");

fn main() {
    App::new()
        .init_plugins::<DefaultPlugins>()
        .add_system(triangle_example_system)
        .add_system(test_input_system)
        .run();
}

fn triangle_example_system(mut commands: Commands) {
    // let render = renderer.get_mut_first_unchecked().unwrap();
    //
    // let renderpass_attachments = [
    //     vk::AttachmentDescription {
    //         format: render.surface_format.format,
    //         samples: vk::SampleCountFlags::TYPE_1,
    //         load_op: vk::AttachmentLoadOp::CLEAR,
    //         store_op: vk::AttachmentStoreOp::STORE,
    //         final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
    //         ..default()
    //     },
    //     vk::AttachmentDescription {
    //         format: vk::Format::D16_UNORM,
    //         samples: vk::SampleCountFlags::TYPE_1,
    //         load_op: vk::AttachmentLoadOp::CLEAR,
    //         initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    //         final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    //         ..Default::default()
    //     },
    // ];
    //
    // let color_attachment_refs = [vk::AttachmentReference {
    //     attachment: 0,
    //     layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    // }];
    // let depth_attachment_ref = vk::AttachmentReference {
    //     attachment: 1,
    //     layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    // };
    //
    // let dependencies = [vk::SubpassDependency {
    //     src_subpass: vk::SUBPASS_EXTERNAL,
    //     src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    //     dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
    //         | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
    //     dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    //     ..Default::default()
    // }];
    //
    // let subpass = vk::SubpassDescription::builder()
    //     .color_attachments(&color_attachment_refs)
    //     .depth_stencil_attachment(&depth_attachment_ref)
    //     .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);
    //
    // let renderpass_create_info = vk::RenderPassCreateInfo::builder()
    //     .attachments(&renderpass_attachments)
    //     .subpasses(std::slice::from_ref(&subpass))
    //     .dependencies(&dependencies);
    //
    // let renderpass = unsafe {
    //     render
    //         .device
    //         .create_render_pass(&renderpass_create_info, None).unwrap()
    // };
    //
    // let framebuffers: Vec<vk::Framebuffer> = render
    //     .present_image_views
    //     .iter()
    //     .map(|&present_image_view| {
    //         let framebuffer_attachments = [present_image_view, render.depth_image_view];
    //         let frame_buffer_create_info = vk::FramebufferCreateInfo::builder()
    //             .render_pass(renderpass)
    //             .attachments(&framebuffer_attachments)
    //             .width(render.surface_resolution.width)
    //             .height(render.surface_resolution.height)
    //             .layers(1);
    //
    //         unsafe {
    //             render.device
    //                 .create_framebuffer(&frame_buffer_create_info, None)
    //                 .unwrap()
    //         }
    //     })
    //     .collect();
    //
    // let index_buffer_data = [0u32, 1, 2];
    // let index_buffer_info = vk::BufferCreateInfo::builder()
    //     .size(std::mem::size_of_val(&index_buffer_data) as u64)
    //     .usage(vk::BufferUsageFlags::INDEX_BUFFER)
    //     .sharing_mode(vk::SharingMode::EXCLUSIVE);
    //
    // let index_buffer = unsafe { render.device.create_buffer(&index_buffer_info, None) }.unwrap();
    // let index_buffer_memory_req = unsafe { render.device.get_buffer_memory_requirements(index_buffer) };
    // let index_buffer_memory_index = find_memory_type_index(
    //     &index_buffer_memory_req,
    //     &base.device_memory_properties,
    //     vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    // )
    //     .expect("Unable to find suitable memory type for the index buffer.");
    //
    // let index_allocate_info = vk::MemoryAllocateInfo {
    //     allocation_size: index_buffer_memory_req.size,
    //     memory_type_index: index_buffer_memory_index,
    //     ..Default::default()
    // };
    // let index_buffer_memory = base
    //     .device
    //     .allocate_memory(&index_allocate_info, None)
    //     .unwrap();
    // let index_ptr = base
    //     .device
    //     .map_memory(
    //         index_buffer_memory,
    //         0,
    //         index_buffer_memory_req.size,
    //         vk::MemoryMapFlags::empty(),
    //     )
    //     .unwrap();
    // let mut index_slice = unsafe {
    //     ash::util::Align::new(
    //         index_ptr,
    //         mem::align_of::<u32>() as u64,
    //         index_buffer_memory_req.size,
    //     )
    // };
    // index_slice.copy_from_slice(&index_buffer_data);
    // unsafe { render.device.unmap_memory(index_buffer_memory); }
    // unsafe {
    //     render.device
    //         .bind_buffer_memory(index_buffer, index_buffer_memory, 0)
    //         .unwrap();
    // }
    //
    // let vertex_input_buffer_info = vk::BufferCreateInfo {
    //     size: 3 * std::mem::size_of::<Vertex>() as u64,
    //     usage: vk::BufferUsageFlags::VERTEX_BUFFER,
    //     sharing_mode: vk::SharingMode::EXCLUSIVE,
    //     ..Default::default()
    // };
    //
    // let vert_ptr = base
    //     .device
    //     .map_memory(
    //         vertex_input_buffer_memory,
    //         0,
    //         vertex_input_buffer_memory_req.size,
    //         vk::MemoryMapFlags::empty(),
    //     )
    //     .unwrap();
    //
    // let mut vert_align = unsafe {
    //     ash::util::Align::new(
    //         vert_ptr,
    //         mem::align_of::<Vertex>() as u64,
    //         vertex_input_buffer_memory_req.size,
    //     )
    // };
    // vert_align.copy_from_slice(&vertices);
    // unsafe { render.device.unmap_memory(vertex_input_buffer_memory); }
    // unsafe {
    //     render.device
    //         .bind_buffer_memory(vertex_input_buffer, vertex_input_buffer_memory, 0)
    //         .unwrap();
    // }
    //
    // let vertex_shader_info = vk::ShaderModuleCreateInfo::builder().code(VERTEX_CODE);
    // let frag_shader_info = vk::ShaderModuleCreateInfo::builder().code(FRAG_CODE);
    //
    // let vertex_shader_module = unsafe {
    //     render
    //         .device
    //         .create_shader_module(&vertex_shader_info, None)
    // }
    //     .expect("Vertex shader module error");
    //
    // let fragment_shader_module = unsafe {
    //     render
    //         .device
    //         .create_shader_module(&frag_shader_info, None)
    // }
    //     .expect("Fragment shader module error");
    //
    // let layout_create_info = vk::PipelineLayoutCreateInfo::default();
    //
    // let pipeline_layout = unsafe {
    //     render
    //         .device
    //         .create_pipeline_layout(&layout_create_info, None)
    // }
    //     .unwrap();
    //
    // let shader_entry_name = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0") };
    // let shader_stage_create_infos = [
    //     vk::PipelineShaderStageCreateInfo {
    //         module: vertex_shader_module,
    //         p_name: shader_entry_name.as_ptr(),
    //         stage: vk::ShaderStageFlags::VERTEX,
    //         ..Default::default()
    //     },
    //     vk::PipelineShaderStageCreateInfo {
    //         s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
    //         module: fragment_shader_module,
    //         p_name: shader_entry_name.as_ptr(),
    //         stage: vk::ShaderStageFlags::FRAGMENT,
    //         ..Default::default()
    //     },
    // ];
    // let vertex_input_binding_descriptions = [vk::VertexInputBindingDescription {
    //     binding: 0,
    //     stride: mem::size_of::<Vertex>() as u32,
    //     input_rate: vk::VertexInputRate::VERTEX,
    // }];
    // let vertex_input_attribute_descriptions = [
    //     vk::VertexInputAttributeDescription {
    //         location: 0,
    //         binding: 0,
    //         format: vk::Format::R32G32B32A32_SFLOAT,
    //         offset: offset_of!(Vertex, pos) as u32,
    //     },
    //     vk::VertexInputAttributeDescription {
    //         location: 1,
    //         binding: 0,
    //         format: vk::Format::R32G32B32A32_SFLOAT,
    //         offset: offset_of!(Vertex, color) as u32,
    //     },
    // ];
    //
    // let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::builder()
    //     .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
    //     .vertex_binding_descriptions(&vertex_input_binding_descriptions);
    // let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
    //     topology: vk::PrimitiveTopology::TRIANGLE_LIST,
    //     ..Default::default()
    // };
    // let viewports = [vk::Viewport {
    //     x: 0.0,
    //     y: 0.0,
    //     width: base.surface_resolution.width as f32,
    //     height: base.surface_resolution.height as f32,
    //     min_depth: 0.0,
    //     max_depth: 1.0,
    // }];
    // let scissors = [base.surface_resolution.into()];
    // let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
    //     .scissors(&scissors)
    //     .viewports(&viewports);
    //
    // let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
    //     front_face: vk::FrontFace::COUNTER_CLOCKWISE,
    //     line_width: 1.0,
    //     polygon_mode: vk::PolygonMode::FILL,
    //     ..Default::default()
    // };
    // let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
    //     rasterization_samples: vk::SampleCountFlags::TYPE_1,
    //     ..Default::default()
    // };
    // let noop_stencil_state = vk::StencilOpState {
    //     fail_op: vk::StencilOp::KEEP,
    //     pass_op: vk::StencilOp::KEEP,
    //     depth_fail_op: vk::StencilOp::KEEP,
    //     compare_op: vk::CompareOp::ALWAYS,
    //     ..Default::default()
    // };
    //
    // let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
    //     depth_test_enable: 1,
    //     depth_write_enable: 1,
    //     depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
    //     front: noop_stencil_state,
    //     back: noop_stencil_state,
    //     max_depth_bounds: 1.0,
    //     ..Default::default()
    // };
    // let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
    //     blend_enable: 0,
    //     src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
    //     dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
    //     color_blend_op: vk::BlendOp::ADD,
    //     src_alpha_blend_factor: vk::BlendFactor::ZERO,
    //     dst_alpha_blend_factor: vk::BlendFactor::ZERO,
    //     alpha_blend_op: vk::BlendOp::ADD,
    //     color_write_mask: vk::ColorComponentFlags::RGBA,
    // }];
    // let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
    //     .logic_op(vk::logic_op::CLEAR)
    //     .attachments(&color_blend_attachment_states);
    //
    // let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    // let dynamic_state_info =
    //     vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);
    //
    // let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
    //     .stages(&shader_stage_create_infos)
    //     .vertex_input_state(&vertex_input_state_info)
    //     .input_assembly_state(&vertex_input_assembly_state_info)
    //     .viewport_state(&viewport_state_info)
    //     .rasterization_state(&rasterization_info)
    //     .multisample_state(&multisample_state_info)
    //     .depth_stencil_state(&depth_state_info)
    //     .color_blend_state(&color_blend_state)
    //     .dynamic_state(&dynamic_state_info)
    //     .layout(pipeline_layout)
    //     .render_pass(renderpass);
    //
    // let graphics_pipelines = unsafe {
    //     render
    //         .device
    //         .create_graphics_pipelines(vk::PipelineCache::null(), &[*graphic_pipeline_info], None)
    // }
    //     .expect("Unable to create graphics pipeline");
    //
    // let graphic_pipeline = graphics_pipelines[0];
}

fn test_input_system(keyboard: Res<Keyboard>, mouse: Res<Mouse>) {
    if keyboard.just_pressed(Key::Space) {
        println!("Pressed Space")
    }
    if mouse.just_pressed(MouseButton::Left) {
        println!("click at {} | {}!", mouse.position.0, mouse.position.1)
    }
}
