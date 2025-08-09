struct Camera2DUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera2DUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct InstanceInput {
    @location(2) position_offset: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) scale: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let instance_transformation = mat4x4<f32>(
        vec4<f32>(instance.scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, instance.scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(instance.position_offset.x, instance.position_offset.y, 0.0, 1.0),
    );

    let instance_position = instance_transformation * vec4<f32>(in.position, 0.0, 1.0);

    out.clip_position = camera.view_proj * instance_position;

    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}