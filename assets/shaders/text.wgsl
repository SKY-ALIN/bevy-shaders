#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct Text {
    count: u32,
    chars: array<u32>,
}

@group(2) @binding(0) var<uniform> width: f32;
@group(2) @binding(1) var<uniform> height: f32;
@group(2) @binding(2) var<uniform> char_width: f32;
@group(2) @binding(3) var<uniform> char_height: f32;
@group(2) @binding(4) var<uniform> margin: f32;
@group(2) @binding(5) var<uniform> rotation: u32;
@group(2) @binding(6) var<uniform> color: vec4<f32>;
@group(2) @binding(7) var<uniform> background_color: vec4<f32>;
@group(2) @binding(8) var<uniform> gap: f32;
@group(2) @binding(9) var<storage, read> text: Text;

const DIGIT_MAP : array<array<u32, 7>, 38> = array<array<u32, 7>, 38>(
    array<u32,7>(0x1F, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F),  // 0
    array<u32,7>(0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01),  // 1
    array<u32,7>(0x1F, 0x10, 0x10, 0x1F, 0x01, 0x01, 0x1F),  // 2
    array<u32,7>(0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F),  // 3
    array<u32,7>(0x11, 0x11, 0x11, 0x1F, 0x10, 0x10, 0x10),  // 4
    array<u32,7>(0x1F, 0x01, 0x01, 0x1F, 0x10, 0x10, 0x1F),  // 5
    array<u32,7>(0x1F, 0x01, 0x01, 0x1F, 0x11, 0x11, 0x1F),  // 6
    array<u32,7>(0x1F, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10),  // 7
    array<u32,7>(0x1F, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x1F),  // 8
    array<u32,7>(0x1F, 0x11, 0x11, 0x1F, 0x10, 0x10, 0x1F),  // 9

    array<u32,7>(0x1F, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11),  // A
    array<u32,7>(0x1F, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x1F),  // B
    array<u32,7>(0x1F, 0x11, 0x01, 0x01, 0x01, 0x11, 0x1F),  // C
    array<u32,7>(0x1F, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F),  // D
    array<u32,7>(0x1F, 0x01, 0x01, 0x1F, 0x01, 0x01, 0x1F),  // E
    array<u32,7>(0x1F, 0x01, 0x01, 0x1F, 0x01, 0x01, 0x01),  // F
    array<u32,7>(0x1F, 0x11, 0x01, 0x1D, 0x11, 0x11, 0x1F),  // G
    array<u32,7>(0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11),  // H
    array<u32,7>(0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x1F),  // I
    array<u32,7>(0x1F, 0x10, 0x10, 0x10, 0x11, 0x11, 0x1F),  // J
    array<u32,7>(0x11, 0x09, 0x05, 0x03, 0x05, 0x09, 0x11),  // K
    array<u32,7>(0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x1F),  // L
    array<u32,7>(0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11),  // M
    array<u32,7>(0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11),  // N
    array<u32,7>(0x1F, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F),  // O
    array<u32,7>(0x1F, 0x11, 0x11, 0x1F, 0x01, 0x01, 0x01),  // P
    array<u32,7>(0x1F, 0x11, 0x11, 0x11, 0x15, 0x09, 0x17),  // Q
    array<u32,7>(0x1F, 0x11, 0x11, 0x1F, 0x09, 0x11, 0x11),  // R
    array<u32,7>(0x1F, 0x01, 0x01, 0x1F, 0x10, 0x10, 0x1F),  // S
    array<u32,7>(0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04),  // T
    array<u32,7>(0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F),  // U
    array<u32,7>(0x11, 0x11, 0x11, 0x11, 0x0A, 0x0A, 0x04),  // V
    array<u32,7>(0x11, 0x11, 0x11, 0x15, 0x15, 0x15, 0x1B),  // W
    array<u32,7>(0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11),  // X
    array<u32,7>(0x11, 0x11, 0x11, 0x0A, 0x04, 0x04, 0x04),  // Y
    array<u32,7>(0x1F, 0x10, 0x08, 0x04, 0x02, 0x01, 0x1F),  // Z

    array<u32,7>(0x0E, 0x11, 0x10, 0x0C, 0x04, 0x00, 0x04),  // ?
    array<u32,7>(0x0E, 0x11, 0x11, 0x0E, 0x00, 0x00, 0x00)   // °
);

const DIGIT_WIDTH  : f32 = 5.0;
const DIGIT_HEIGHT : f32 = 7.0;

fn rotate_uv(uv_in: vec2<f32>, rot: u32) -> vec2<f32> {
    let center_uv = uv_in - vec2<f32>(0.5, 0.5);

    var uv_out = center_uv;
    switch rot {
        default {}
        // 90°
        case 1u {
            uv_out = vec2<f32>(center_uv.y, -center_uv.x);
        }
        // 180°
        case 2u {
            uv_out = -center_uv;
        }
        // 270°
        case 3u {
            uv_out = vec2<f32>(-center_uv.y, center_uv.x);
        }
    }

    return uv_out + vec2<f32>(0.5, 0.5);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixelPos = rotate_uv(in.uv, rotation) * vec2<f32>(width, height);

    let chars_count = text.count;

    if pixelPos.x < margin
    || pixelPos.x >= (width  - margin)
    || pixelPos.y < margin
    || pixelPos.y >= (height - margin)
    {
        return background_color;
    }

    let totalSymbolsWidth = f32(chars_count) * char_width + f32(chars_count - 1u) * gap;

    let leftBound  = margin;
    let rightBound = margin + f32(totalSymbolsWidth);

    if pixelPos.x < leftBound || pixelPos.x >= rightBound {
        return background_color;
    }

    let topBound = margin + char_height;
    if pixelPos.y >= topBound {
        return background_color;
    }

    let pxInside = pixelPos.x - margin;
    let blockSize = char_width + gap;
    let symbolIndex = u32(floor(pxInside / blockSize));

    if symbolIndex >= chars_count {
        return background_color;
    }

    let localX = pxInside % blockSize;

    if localX >= char_width {
        return background_color; // gap
    }

    let localY = pixelPos.y - margin;

    let digitX = (localX / char_width) * DIGIT_WIDTH;
    let digitY = (localY / char_height) * DIGIT_HEIGHT;

    let cellX = i32(floor(digitX));
    let cellY = i32(floor(digitY));

    if cellX < 0 || cellX >= i32(DIGIT_WIDTH)
    || cellY < 0 || cellY >= i32(DIGIT_HEIGHT) {
        return background_color;
    }

    let digit = text.chars[symbolIndex];

    let rowMask = DIGIT_MAP[digit][cellY];
    let bitShift = u32(cellX);

    let pixelOn = ((rowMask >> bitShift) & 0x1u) == 1u;
    if pixelOn {
        return color;
    } else {
        return background_color;
    }
}
