#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var<uniform> sequence: array<vec4<f32>, 4>;
@group(2) @binding(1) var<uniform> active_color: vec4<f32>;
@group(2) @binding(2) var<uniform> passive_color: vec4<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var total_length: f32 = 0.0;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        for (var j: i32 = 0; j < 4; j = j + 1) {
            total_length = total_length + sequence[i][j];
        }
    }

    if (total_length <= 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let t_mod = globals.time * 1000 % total_length;

    var accumulated: f32 = 0.0;
    var interval_index: i32 = -1;

    for (var i: i32 = 0; i < 4; i = i + 1) {
        for (var j: i32 = 0; j < 4; j = j + 1) {
            let current_interval = sequence[i][j];

            if (t_mod < accumulated + current_interval) {
                interval_index = i * 4 + j;
                break;
            }

            accumulated = accumulated + current_interval;
        }

        if (interval_index != -1) {
            break;
        }
    }

    if (interval_index == -1) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    if ((interval_index % 2) == 1) {
        return passive_color;
    } else {
        return active_color;
    }
}
