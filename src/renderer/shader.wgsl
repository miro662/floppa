struct CameraUniform {
    view_proj: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) tex: vec2<f32>
}

struct InstanceInput {
    @location(2) matrix_1: vec3<f32>,
    @location(3) matrix_2: vec3<f32>,
    @location(4) matrix_3: vec3<f32>,
    @location(5) tex_lower_bounds: vec2<f32>,
    @location(6) tex_higher_bounds: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex: vec2<f32>
}

fn real_tex_coords(vertex: VertexInput, instance: InstanceInput) -> vec2<f32> {
    var real_tex = vec2<f32>(0.0, 0.0);

    if (vertex.tex.x < 0.5) {
        real_tex.x = instance.tex_lower_bounds.x;
    } else {
        real_tex.x = instance.tex_higher_bounds.x;
    }

    if (vertex.tex.y < 0.5) {
        real_tex.y = instance.tex_lower_bounds.y;
    } else {
        real_tex.y = instance.tex_higher_bounds.y;
    }
    return real_tex;
}

@vertex
fn vertex(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    var instance_matrix = mat3x3<f32>(instance.matrix_1, instance.matrix_2, instance.matrix_3);

    var pos = vec3<f32>(model.pos.xy, 1.0);
    var transformed_pos = instance_matrix * pos;
    out.clip_pos = vec4<f32>(transformed_pos.xy, 0.0, transformed_pos.z);
    out.clip_pos = camera.view_proj * out.clip_pos;

    out.tex = real_tex_coords(model, instance);

    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1)@binding(1)
var sampler_: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(texture, sampler_, in.tex);
    return color;
}