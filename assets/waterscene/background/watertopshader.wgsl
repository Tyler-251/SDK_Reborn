[[group(0), binding(0)]] var<uniform> texture: texture_2d<f32>;
[[group(0), binding(1)]] var<uniform> sampler: sampler;
[[group(0), binding(2)]] var<uniform> time: f32;

[[stage(fragment)]]
fn main(
    [[location(0)]] in_uv: vec2<f32>
) -> [[location(0)]] vec4<f32> {
    let color = textureSample(texture, sampler, in_uv);
    let wave = 0.5 + 0.5 * sin(in_uv.x * 10.0 + time);
    if (in_uv.y < wave) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return color;
}