//! Plain D3D11 / DXGI data types from CommonLib `REX/W32/{D3D11,DXGI}.h`.
//!
//! `windows-sys` does not ship the Direct3D/DXGI namespaces, so the subset
//! needed by overlay renderers is mirrored here (same names as REX, fields in
//! snake_case). Enum types are `u32` aliases with `pub const` values.
//!
//! TODO: extend `scripts/generate_rex_com.py` to emit the full struct/enum set
//! from the headers instead of this hand-maintained subset.
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_void};

pub type HRESULT = i32;
pub type BOOL = i32;

#[inline(always)]
pub const fn succeeded(hr: HRESULT) -> bool {
    hr >= 0
}

// ---------------------------------------------------------------------------
// DXGI

pub type DXGI_FORMAT = u32;
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = 0;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = 2;
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = 16;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = 28;
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = 42;
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = 57;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = 87;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DXGI_RATIONAL {
    pub numerator: u32,
    pub denominator: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DXGI_MODE_DESC {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: DXGI_RATIONAL,
    pub format: DXGI_FORMAT,
    pub scanline_ordering: u32,
    pub scaling: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DXGI_SAMPLE_DESC {
    pub count: u32,
    pub quality: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub buffer_desc: DXGI_MODE_DESC,
    pub sample_desc: DXGI_SAMPLE_DESC,
    pub buffer_usage: u32,
    pub buffer_count: u32,
    pub output_window: *mut c_void, // HWND
    pub windowed: BOOL,
    pub swap_effect: u32,
    pub flags: u32,
}

const _: () = assert!(core::mem::size_of::<DXGI_SWAP_CHAIN_DESC>() == 0x48);
const _: () = assert!(core::mem::offset_of!(DXGI_SWAP_CHAIN_DESC, output_window) == 0x30);

// ---------------------------------------------------------------------------
// D3D11 enums / flags

pub type D3D11_USAGE = u32;
pub const D3D11_USAGE_DEFAULT: D3D11_USAGE = 0;
pub const D3D11_USAGE_IMMUTABLE: D3D11_USAGE = 1;
pub const D3D11_USAGE_DYNAMIC: D3D11_USAGE = 2;

pub const D3D11_BIND_VERTEX_BUFFER: u32 = 0x1;
pub const D3D11_BIND_INDEX_BUFFER: u32 = 0x2;
pub const D3D11_BIND_CONSTANT_BUFFER: u32 = 0x4;
pub const D3D11_BIND_SHADER_RESOURCE: u32 = 0x8;

pub const D3D11_CPU_ACCESS_WRITE: u32 = 0x10000;

pub type D3D11_MAP = u32;
pub const D3D11_MAP_WRITE_DISCARD: D3D11_MAP = 4;

pub type D3D11_INPUT_CLASSIFICATION = u32;
pub const D3D11_INPUT_PER_VERTEX_DATA: D3D11_INPUT_CLASSIFICATION = 0;

pub type D3D11_BLEND = u32;
pub const D3D11_BLEND_ZERO: D3D11_BLEND = 1;
pub const D3D11_BLEND_ONE: D3D11_BLEND = 2;
pub const D3D11_BLEND_SRC_ALPHA: D3D11_BLEND = 5;
pub const D3D11_BLEND_INV_SRC_ALPHA: D3D11_BLEND = 6;

pub type D3D11_BLEND_OP = u32;
pub const D3D11_BLEND_OP_ADD: D3D11_BLEND_OP = 1;

pub const D3D11_COLOR_WRITE_ENABLE_ALL: u8 = 0xF;

pub type D3D11_FILL_MODE = u32;
pub const D3D11_FILL_SOLID: D3D11_FILL_MODE = 3;

pub type D3D11_CULL_MODE = u32;
pub const D3D11_CULL_NONE: D3D11_CULL_MODE = 1;

pub type D3D11_DEPTH_WRITE_MASK = u32;
pub const D3D11_DEPTH_WRITE_MASK_ZERO: D3D11_DEPTH_WRITE_MASK = 0;
pub const D3D11_DEPTH_WRITE_MASK_ALL: D3D11_DEPTH_WRITE_MASK = 1;

pub type D3D11_COMPARISON_FUNC = u32;
pub const D3D11_COMPARISON_ALWAYS: D3D11_COMPARISON_FUNC = 8;

pub type D3D11_STENCIL_OP = u32;
pub const D3D11_STENCIL_OP_KEEP: D3D11_STENCIL_OP = 1;

pub type D3D11_FILTER = u32;
pub const D3D11_FILTER_MIN_MAG_MIP_LINEAR: D3D11_FILTER = 0x15;

pub type D3D11_TEXTURE_ADDRESS_MODE = u32;
pub const D3D11_TEXTURE_ADDRESS_WRAP: D3D11_TEXTURE_ADDRESS_MODE = 1;
pub const D3D11_TEXTURE_ADDRESS_CLAMP: D3D11_TEXTURE_ADDRESS_MODE = 3;

pub type D3D11_SRV_DIMENSION = u32;
pub const D3D11_SRV_DIMENSION_TEXTURE2D: D3D11_SRV_DIMENSION = 4;

pub type D3D11_PRIMITIVE_TOPOLOGY = u32;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D11_PRIMITIVE_TOPOLOGY = 4;

pub const D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: usize = 16;

// ---------------------------------------------------------------------------
// D3D11 structs

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_BUFFER_DESC {
    pub byte_width: u32,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
    pub structure_byte_stride: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub sys_mem: *const c_void,
    pub sys_mem_pitch: u32,
    pub sys_mem_slice_pitch: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_TEXTURE2D_DESC {
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    pub array_size: u32,
    pub format: DXGI_FORMAT,
    pub sample_desc: DXGI_SAMPLE_DESC,
    pub usage: D3D11_USAGE,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_TEX2D_SRV {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
}

/// Only the `texture2D` member of the C++ union is modelled; the union is 16
/// bytes (largest members: 4 x `u32`).
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub format: DXGI_FORMAT,
    pub view_dimension: D3D11_SRV_DIMENSION,
    pub texture2d: D3D11_TEX2D_SRV,
    pub _union_tail: [u32; 2],
}

const _: () = assert!(core::mem::size_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>() == 0x18);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub semantic_name: *const c_char,
    pub semantic_index: u32,
    pub format: DXGI_FORMAT,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: D3D11_INPUT_CLASSIFICATION,
    pub instance_data_step_rate: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub data: *mut c_void,
    pub row_pitch: u32,
    pub depth_pitch: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub blend_enable: BOOL,
    pub src_blend: D3D11_BLEND,
    pub dest_blend: D3D11_BLEND,
    pub blend_op: D3D11_BLEND_OP,
    pub src_blend_alpha: D3D11_BLEND,
    pub dest_blend_alpha: D3D11_BLEND,
    pub blend_op_alpha: D3D11_BLEND_OP,
    pub render_target_write_mask: u8,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_BLEND_DESC {
    pub alpha_to_coverage_enable: BOOL,
    pub independent_blend_enable: BOOL,
    pub render_target: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_RASTERIZER_DESC {
    pub fill_mode: D3D11_FILL_MODE,
    pub cull_mode: D3D11_CULL_MODE,
    pub front_counter_clockwise: BOOL,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: BOOL,
    pub scissor_enable: BOOL,
    pub multisample_enable: BOOL,
    pub antialiased_line_enable: BOOL,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub stencil_fail_op: D3D11_STENCIL_OP,
    pub stencil_depth_fail_op: D3D11_STENCIL_OP,
    pub stencil_pass_op: D3D11_STENCIL_OP,
    pub stencil_func: D3D11_COMPARISON_FUNC,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub depth_enable: BOOL,
    pub depth_write_mask: D3D11_DEPTH_WRITE_MASK,
    pub depth_func: D3D11_COMPARISON_FUNC,
    pub stencil_enable: BOOL,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: D3D11_DEPTH_STENCILOP_DESC,
    pub back_face: D3D11_DEPTH_STENCILOP_DESC,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct D3D11_SAMPLER_DESC {
    pub filter: D3D11_FILTER,
    pub address_u: D3D11_TEXTURE_ADDRESS_MODE,
    pub address_v: D3D11_TEXTURE_ADDRESS_MODE,
    pub address_w: D3D11_TEXTURE_ADDRESS_MODE,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: D3D11_COMPARISON_FUNC,
    pub border_color: [f32; 4],
    pub min_lod: f32,
    pub max_lod: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub top_left_x: f32,
    pub top_left_y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

/// `D3D11_RECT` (= Win32 `RECT`)
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct D3D11_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

// ---------------------------------------------------------------------------
// D3DCompiler

/// C++ `REX::W32::ID3DBlob` (`ID3D10Blob`).
#[repr(C)]
pub struct ID3DBlob {
    pub vtbl: *const ID3DBlobVtbl,
}

#[repr(C)]
pub struct ID3DBlobVtbl {
    pub base: super::IUnknownVtbl,
    pub GetBufferPointer: *const c_void,
    pub GetBufferSize: *const c_void,
}

impl ID3DBlob {
    /// # Safety
    /// `this` must be a live blob.
    pub unsafe fn as_bytes<'a>(this: *mut ID3DBlob) -> &'a [u8] {
        let ptr = crate::com_call!(this, GetBufferPointer: fn() -> *const u8;);
        let len = crate::com_call!(this, GetBufferSize: fn() -> usize;);
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}
