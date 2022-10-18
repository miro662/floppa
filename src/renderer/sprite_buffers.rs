use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pos: [f32; 2],
    tex: [f32; 2],
}
const VERTICES: &[Vertex] = &[
    Vertex {
        pos: [0.0, 0.0],
        tex: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, 0.0],
        tex: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0],
        tex: [0.0, 0.0],
    },
    Vertex {
        pos: [0.0, 1.0],
        tex: [0.0, 0.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 3, 2, 0, 0, 0];

#[derive(Debug)]
pub(in crate::renderer) struct SpriteBuffers {
    pub(in crate::renderer) vertex: wgpu::Buffer,
    pub(in crate::renderer) index: wgpu::Buffer,
}

impl SpriteBuffers {
    pub(in crate::renderer) fn create(device: &wgpu::Device, label: Option<&str>) -> SpriteBuffers {
        let vertex_descriptor = wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(&VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        };
        let vertex = device.create_buffer_init(&vertex_descriptor);

        let index_descriptor = wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        };
        let index = device.create_buffer_init(&index_descriptor);

        SpriteBuffers { vertex, index }
    }
}
