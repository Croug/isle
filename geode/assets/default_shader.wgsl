struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
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
    view_pos: vec3<f32>,
};

struct PointLight {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
};

struct SpotLight {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    direction: vec3<f32>,
    outer: f32,
    inner: f32,
}

const MAX_LIGHTS: u32 = 128;
struct Lights {
    ambient_color: vec3<f32>,
    ambient_intensity: f32,
    num_point_lights: u32,
    num_spot_lights: u32,
    point_lights: array<PointLight, MAX_LIGHTS>,
    spot_lights: array<SpotLight, MAX_LIGHTS>,
};

@group(0) @binding(0)
var<storage, read> lights: Lights;
@group(1) @binding(0)
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

    out.world_pos = (model * vec4<f32>(mesh.position, 1.0)).xyz;

    return out;
}

@group(2) @binding(0)
var texture: texture_2d<f32>;
@group(2) @binding(1)
var sampler_in: sampler;

const shininess: f32 = 35.0;

struct FragmentOutput {
    @location(0) texture_out: vec4<f32>,
    @location(1) surface_out: vec4<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var diffuse_color = lights.ambient_color * lights.ambient_intensity;
    let color = textureSample(texture, sampler_in, in.uv);
    let normal = normalize(in.normal);
    let view_dir = normalize(camera.view_pos - in.world_pos);

    for (var i = 0u; i < lights.num_point_lights; i = i + 1u) {
        let light = lights.point_lights[i];
        let light_dir = normalize(light.position - in.world_pos);

        let diffuse_strength = max(dot(normal, light_dir), 0.0);
        let local_diffuse = light.color * diffuse_strength;

        let half_dir = normalize(light_dir + view_dir);
        let specular_strength = pow(max(dot(normal, half_dir), 0.0), shininess);
        let specular_color = light.color * specular_strength;

        let distance = length(light.position - in.world_pos);
        let attenuation = clamp(light.intensity / distance, 0.0, 1.0);

        diffuse_color += (local_diffuse + specular_color) * attenuation;
    }

    for (var i = 0u; i < lights.num_spot_lights; i = i + 1u) {
        let light = lights.spot_lights[i];
        let light_dir = normalize(light.direction);
        let sl_dir = normalize(light.position - in.world_pos);
        let half_dir = normalize(sl_dir + view_dir);

        let dot = dot(sl_dir, -light_dir);
        let in_light = clamp(
            (dot - light.outer) / (light.inner - light.outer),
            0.0,
            1.0
        );
        let diffuse_strength = in_light * max(dot(normal, sl_dir), 0.0);
        let local_diffuse = light.color * diffuse_strength;

        let specular_strength = in_light * pow(max(dot(normal, half_dir), 0.0), shininess);
        let specular_color = light.color * specular_strength;

        let distance = length(light.position - in.world_pos);
        let attenuation = clamp(light.intensity / distance, 0.0, 1.0);

        diffuse_color += (local_diffuse + specular_color) * attenuation;
    }

    let out_color = vec4<f32>(
        color.rgb * diffuse_color.rgb,
        color.a
    );

    var out: FragmentOutput;
    out.texture_out = out_color;
    out.surface_out = out_color;

    return out;
}