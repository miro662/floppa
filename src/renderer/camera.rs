use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    fn from_screen_size(screen_size: cgmath::Vector2<u32>) -> CameraUniform {
        let translation_matrix = cgmath::Matrix4::from_translation((-1.0, -1.0, 0.0).into());
        let two_scale_matrix = cgmath::Matrix4::from_scale(2.0);
        let res_scale_matrix = cgmath::Matrix4::from_nonuniform_scale(
            1.0 / (screen_size.x as f32),
            1.0 / (screen_size.y as f32),
            1.0,
        );
        let matrix = translation_matrix * two_scale_matrix * res_scale_matrix;
        CameraUniform {
            view_proj_matrix: matrix.into(),
        }
    }
}


#[derive(Debug)]
pub struct Camera {
    pub bind_group: wgpu::BindGroup
}

impl Camera {
    pub fn new(device: &wgpu::Device, screen_size: cgmath::Vector2<u32>, layout: &wgpu::BindGroupLayout) -> Camera {
        let camera_uniforms = [CameraUniform::from_screen_size(screen_size)];

        let camera_buffer_description = wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&camera_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };
        let camera_buffer = device.create_buffer_init(&camera_buffer_description);

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Camera {bind_group: camera_bind_group}
    }

}