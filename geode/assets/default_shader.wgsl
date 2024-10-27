struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(3) model_0: vec4<f32>,
    @location(4) model_1: vec4<f32>,
    @location(5) model_2: vec4<f32>,
    @location(6) model_3: vec4<f32>,
    @location(7) normal_0: vec3<f32>,
    @location(8) normal_1: vec3<f32>,
    @location(9) normal_2: vec3<f32>,
};

struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(
    mesh: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model = mat4x4<f32>(
        instance.model_0,
        instance.model_1,
        instance.model_2,
        instance.model_3,
    );

    let normal_mat = mat3x3<f32>(
        instance.normal_0,
        instance.normal_1,
        instance.normal_2,
    );

    var out: VertexOutput;
    out.uv = mesh.uv;
    out.normal = normal_mat * mesh.normal;
    out.position = camera.view_proj * model * vec4<f32>(mesh.position, 1.0);

    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var sampler_in: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.normal);
    let direction = normalize(vec3<f32>(0.0, -1.0, 1.0));
    let light = dot(normal, -direction) + 0.1;
    return textureSample(texture, sampler_in, in.uv) * light;
}