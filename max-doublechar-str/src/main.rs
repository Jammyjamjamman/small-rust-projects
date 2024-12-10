use std::{str::from_utf8, sync::mpsc::channel};

use pollster::block_on;
use wgpu::util::DeviceExt;

fn main() {
    block_on(run());
}

async fn run() {
    let s = "abcbbbbcccbdddadacbz";
    let chars: Vec<char> = s.chars().collect();
    println!("chars: {:?}", &chars);
    println!("len: {}", chars.len());
    println!("char sz: {}", size_of::<char>());
    // println!("{:?}", bytemuck::cast_slice::<char, u8>(&chars));

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("max doublechar substr"),
        source: wgpu::ShaderSource::Wgsl(include_str!("count_doublechar.wgsl").into()),
    });

    let str_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("List of levels"),
        contents: bytemuck::cast_slice(&chars),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let max_len_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Max char pair length"),
        contents: bytemuck::cast_slice(&vec![2u32; chars.len() - 1]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: ((chars.len() - 1) * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: str_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: max_len_buf.as_entire_binding(),
            },
        ],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: Default::default(),
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(64, 1, 1);
    }

    encoder.copy_buffer_to_buffer(
        &max_len_buf,
        0,
        &output_buffer,
        0,
        ((chars.len() - 1) * std::mem::size_of::<u32>()) as u64,
    );

    queue.submit(Some(encoder.finish()));

    let buffer_slice = output_buffer.slice(..);

    let (sender, _receiver) = channel();
    let _buffer_future =
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();
    let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
    // .into_iter()
    // .map(|v: u32| char::from_u32(v).expect("cannot convert int to char!"))
    // .collect();

    drop(data);
    output_buffer.unmap();

    println!("output arr {:?}", result);
    println!("result {}", result.iter().max().expect("error getting max"));
}
