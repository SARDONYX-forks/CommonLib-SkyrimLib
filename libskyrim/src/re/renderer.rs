//! C++ `RE::BSGraphics::Renderer` (formerly `RE::BSRenderManager`).
//!
//! Partial translation: the D3D11 device/context and the render windows, which
//! is what overlay/UI plugins need. Direct3D/DXGI interfaces are exposed as raw
//! pointers so callers can wrap them with their COM binding of choice.

use core::ffi::c_void;

use crate::relocation::RelocationID;
use crate::rex::{
    ID3D11Device, ID3D11DeviceContext, ID3D11RenderTargetView, ID3D11ShaderResourceView,
    IDXGISwapChain,
};

/// C++ `RE::BSGraphics::RendererWindow`
#[repr(C)]
#[derive(Debug)]
pub struct RendererWindow {
    pub hwnd: *mut c_void,                            // 00 - HWND
    pub window_x: i32,                                // 08
    pub window_y: i32,                                // 0C
    pub window_width: i32,                            // 10
    pub window_height: i32,                           // 14
    pub swap_chain: *mut IDXGISwapChain,              // 18 - IDXGISwapChain*
    pub swap_chain_render_target: u32,                // 20 - RENDER_TARGET
    pub unk24: u32,                                   // 24
    pub unk28: u64,                                   // 28
    pub render_view: *mut ID3D11RenderTargetView,     // 30 - ID3D11RenderTargetView1*
    pub resource_view: *mut ID3D11ShaderResourceView, // 38 - ID3D11ShaderResourceView1*
    pub unk40: u64,                                   // 40
    pub unk48: u64,                                   // 48
}

const _: () = assert!(core::mem::size_of::<RendererWindow>() == 0x50);

/// Leading part of C++ `RE::BSGraphics::RendererData` (`RUNTIME_DATA_CONTENT`).
///
/// TODO: `renderTargets` and the trailing render-target arrays are not
/// translated yet; only the prefix up to `renderWindows` is modelled.
#[repr(C)]
#[derive(Debug)]
pub struct RendererData {
    pub ui_adapter: u32,                      // 00
    pub desired_refresh_rate: [u32; 2],       // 04 - DXGI_RATIONAL
    pub actual_refresh_rate: [u32; 2],        // 0C - DXGI_RATIONAL
    pub scale_mode: u32,                      // 14 - DXGI_MODE_SCALING
    pub scanline_ordering: u32,               // 18 - DXGI_MODE_SCANLINE_ORDER
    pub is_not_windowed: u32,                 // 1C
    pub full_screen: bool,                    // 20
    pub borderless_display: bool,             // 21
    pub read_only_depth: bool,                // 22
    pub instantiated: bool,                   // 23
    pub requested_window_size_change: bool,   // 24
    pub unk25: bool,                          // 25
    pub pad26: u16,                           // 26
    pub new_width: u32,                       // 28
    pub new_height: u32,                      // 2C
    pub present_interval: u32,                // 30
    pub pad34: u32,                           // 34
    pub forwarder: *mut ID3D11Device,         // 38 - ID3D11Device*
    pub context: *mut ID3D11DeviceContext,    // 40 - ID3D11DeviceContext*
    pub render_windows: [RendererWindow; 32], // 48
}

const _: () = assert!(core::mem::offset_of!(RendererData, forwarder) == 0x38);
const _: () = assert!(core::mem::offset_of!(RendererData, context) == 0x40);
const _: () = assert!(core::mem::offset_of!(RendererData, render_windows) == 0x48);

/// C++ `RE::BSGraphics::Renderer` (opaque; use the accessors).
#[repr(C)]
pub struct Renderer {
    pub unk000: u64,         // 00
    pub unk008: *mut c_void, // 08 - window resize callback in SE
}

impl Renderer {
    crate::relocation_variable! {
        fn singleton() -> &'static mut Renderer => RelocationID::new(524907, 411393)
    }

    /// C++ `Renderer::GetSingleton()`. The renderer is a static object; this
    /// never returns null once the Address Library resolved.
    #[inline(always)]
    pub fn get_singleton() -> *mut Renderer {
        Self::singleton()
    }

    // C++ `RUNTIME_DATA_ACCESSOR(RendererData, 0x10, 0x18)`
    crate::runtime_data_accessor! {
        pub fn renderer_data() -> RendererData {
            se: 0x10, ae: 0x10, vr: 0x18
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn renderer_data_mut() -> RendererData {
            se: 0x10, ae: 0x10, vr: 0x18
        }
    }

    /// `ID3D11Device*`
    #[inline(always)]
    pub fn device(&self) -> *mut ID3D11Device {
        self.renderer_data().forwarder
    }

    /// `ID3D11DeviceContext*`
    #[inline(always)]
    pub fn context(&self) -> *mut ID3D11DeviceContext {
        self.renderer_data().context
    }

    /// The main window (`renderWindows[0]`), which owns the presentation swap chain.
    #[inline(always)]
    pub fn main_window(&self) -> &RendererWindow {
        &self.renderer_data().render_windows[0]
    }

    crate::relocation_func! {
        pub fn update_camera_data(&mut self, flags: u32) => RelocationID::new(75472, 77258)
    }

    crate::relocation_func! {
        /// Returns `NiTexture::RendererData*`.
        pub fn create_render_texture(&mut self, width: u32, height: u32) -> *mut c_void => RelocationID::new(75507, 77299)
    }
}
