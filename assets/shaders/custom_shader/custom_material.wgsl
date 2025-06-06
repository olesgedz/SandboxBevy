#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(3) var<uniform> frame : u32;

struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;


@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
//    // Chromatic aberration strength
//    let offset_strength = settings.intensity;
//
//    // Sample each color channel with an arbitrary shift
//    return vec4<f32>(
//        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
//        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
//        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
//        1.0
//    );
  return vec4(1, sin(f32(frame) / 128), 0, 1);
}


//vec2 curve(vec2 uv)
//{
//	uv = (uv - 0.5) * 2.0;
//	uv *= 1.1;	
//	uv.x *= 1.0 + pow((abs(uv.y) / 5.0), 2.0);
//	uv.y *= 1.0 + pow((abs(uv.x) / 4.0), 2.0);
//	uv  = (uv / 2.0) + 0.5;
//	uv =  uv *0.92 + 0.04;
//	return uv;
//}
//void mainImage( out vec4 fragColor, in vec2 fragCoord )
//{
//    vec2 q = fragCoord.xy / iResolution.xy;
//    vec2 uv = q;
//    uv = curve( uv );
//    vec3 oricol = texture( iChannel0, vec2(q.x,q.y) ).xyz;
//    vec3 col;
//	float x =  sin(0.3*iTime+uv.y*21.0)*sin(0.7*iTime+uv.y*29.0)*sin(0.3+0.33*iTime+uv.y*31.0)*0.0017;
//
//    col.r = texture(iChannel0,vec2(x+uv.x+0.001,uv.y+0.001)).x+0.05;
//    col.g = texture(iChannel0,vec2(x+uv.x+0.000,uv.y-0.002)).y+0.05;
//    col.b = texture(iChannel0,vec2(x+uv.x-0.002,uv.y+0.000)).z+0.05;
//    col.r += 0.08*texture(iChannel0,0.75*vec2(x+0.025, -0.027)+vec2(uv.x+0.001,uv.y+0.001)).x;
//    col.g += 0.05*texture(iChannel0,0.75*vec2(x+-0.022, -0.02)+vec2(uv.x+0.000,uv.y-0.002)).y;
//    col.b += 0.08*texture(iChannel0,0.75*vec2(x+-0.02, -0.018)+vec2(uv.x-0.002,uv.y+0.000)).z;
//
//    col = clamp(col*0.6+0.4*col*col*1.0,0.0,1.0);
//
//    float vig = (0.0 + 1.0*16.0*uv.x*uv.y*(1.0-uv.x)*(1.0-uv.y));
//	col *= vec3(pow(vig,0.3));
//
//    col *= vec3(0.95,1.05,0.95);
//	col *= 2.8;
//
//	float scans = clamp( 0.35+0.35*sin(3.5*iTime+uv.y*iResolution.y*1.5), 0.0, 1.0);
//	
//	float s = pow(scans,1.7);
//	col = col*vec3( 0.4+0.7*s) ;
//
//    col *= 1.0+0.01*sin(110.0*iTime);
//	if (uv.x < 0.0 || uv.x > 1.0)
//		col *= 0.0;
//	if (uv.y < 0.0 || uv.y > 1.0)
//		col *= 0.0;
//	
//	col*=1.0-0.65*vec3(clamp((mod(fragCoord.x, 2.0)-1.0)*2.0,0.0,1.0));
//	
//    float comp = smoothstep( 0.1, 0.9, sin(iTime) );
// 
//	// Remove the next line to stop cross-fade between original and postprocess
////	col = mix( col, oricol, comp );
//
//    fragColor = vec4(col,1.0);
//}