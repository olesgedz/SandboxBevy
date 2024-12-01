struct Uniforms {
    millis: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(0) @binding(1) var texture: texture_2d<f32>;
@group(0) @binding(2) var textureSampler: sampler;

fn curve(uv: vec2<f32>) -> vec2<f32> {
    var transformed_uv = (uv - vec2<f32>(0.5)) * 2.0;
    transformed_uv *= 1.1;
    transformed_uv.x *= 1.0 + pow(abs(transformed_uv.y) / 5.0, 2.0);
    transformed_uv.y *= 1.0 + pow(abs(transformed_uv.x) / 4.0, 2.0);
    transformed_uv = (transformed_uv / 2.0) + 0.5;
    return transformed_uv * 0.92 + 0.04;
}

@fragment
fn main(@location(0) in_uv: vec2<f32>, @builtin(position) screen_coords: vec4<f32>) -> @location(0) vec4<f32> {
    var uv = in_uv;
    uv = curve(uv);

    let oricol = textureSample(texture, textureSampler, uv).rgb;
    let millis = uniforms.millis;

    let x = sin(0.3 * millis + uv.y * 21.0) * 
            sin(0.7 * millis + uv.y * 29.0) * 
            sin(0.3 + 0.33 * millis + uv.y * 31.0) * 0.0017;

    var col = vec3<f32>(0.0);
    col.r = textureSample(texture, sampler, vec2<f32>(x + uv.x + 0.001, uv.y + 0.001)).r + 0.05;
    col.g = textureSample(texture, sampler, vec2<f32>(x + uv.x + 0.000, uv.y - 0.002)).g + 0.05;
    col.b = textureSample(texture, sampler, vec2<f32>(x + uv.x - 0.002, uv.y + 0.000)).b + 0.05;

    col.r += 0.08 * textureSample(texture, sampler, 0.75 * vec2<f32>(x + 0.025, -0.027) + vec2<f32>(uv.x + 0.001, uv.y + 0.001)).r;
    col.g += 0.05 * textureSample(texture, sampler, 0.75 * vec2<f32>(x - 0.022, -0.02) + vec2<f32>(uv.x + 0.000, uv.y - 0.002)).g;
    col.b += 0.08 * textureSample(texture, sampler, 0.75 * vec2<f32>(x - 0.02, -0.018) + vec2<f32>(uv.x - 0.002, uv.y + 0.000)).b;

    col = clamp(col * 0.6 + 0.4 * col * col, vec3<f32>(0.0), vec3<f32>(1.0));

    let vig = 1.0 * 16.0 * uv.x * uv.y * (1.0 - uv.x) * (1.0 - uv.y);
    col *= pow(vig, 0.3);

    col *= vec3<f32>(0.95, 1.05, 0.95);
    col *= 2.8;

    let scans = clamp(0.35 + 0.35 * sin(3.5 * millis + uv.y * screen_coords.y * 1.5), 0.0, 1.0);
    let s = pow(scans, 5.7);
    col *= vec3<f32>(0.4 + 0.7 * s);

    col *= 1.0 + 0.01 * sin(10.0 * millis);
    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        col *= 0.0;
    }

    col *= 1.0 - 0.65 * vec3<f32>(clamp((mod(in_uv.x, 2.0) - 1.0) * 2.0, 0.0, 1.0));

    return vec4<f32>(col, 1.0);
}