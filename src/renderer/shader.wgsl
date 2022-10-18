struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) tex: vec2<f32>
}

struct InstanceInput {
    @location(2) matrix_1: vec3<f32>,
    @location(3) matrix_2: vec3<f32>,
    @location(4) matrix_3: vec3<f32>
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex: vec2<f32>
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

    out.tex = model.tex;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}