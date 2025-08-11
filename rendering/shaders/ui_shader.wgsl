struct Camera2DUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera2DUniform;

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var texture_sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) quad_uv: vec2<f32>,
};

struct InstanceInput {
    @location(2) id: u32,
    @location(3) position_offset: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) scale: vec2<f32>,
    @location(6) atlas_coords: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};

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

    let atlas_start = instance.atlas_coords.xy;
    let atlas_end = instance.atlas_coords.zw;
    let atlas_size = atlas_end - atlas_start;
    out.tex_coords = atlas_start + in.quad_uv * atlas_size;

    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var final_color: vec4<f32>;

    final_color = textureSample(texture, texture_sampler, in.tex_coords);
    final_color = final_color * vec4<f32>(in.color);

    return final_color;
}