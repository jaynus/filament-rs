use filament_sys::ffi;
use std::sync::Arc;

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
    swapchain::SwapChain,
    view::View,
    Viewport,
};

pub use ffi::filament_Renderer_ClearOptions as ClearOptions;
pub use ffi::filament_Renderer_DisplayInfo as DisplayInfo;
pub use ffi::filament_Renderer_FrameRateOptions as FrameRateOptions;

bitflags::bitflags! {
    pub struct CopyFrameFlags: ffi::filament_Renderer_CopyFrameFlag {
        const COMMIT = ffi::filament_Renderer_COMMIT;
        const SET_PRESENTATION_TIME = ffi::filament_Renderer_SET_PRESENTATION_TIME;
        const CLEAR = ffi::filament_Renderer_CLEAR;
    }
}

impl_handle!(Renderer, ffi::filament_Renderer);
impl Drop for Renderer {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy8(*ptr) }
        }
    }
}
impl Renderer {
    pub fn set_display_info(&mut self, display_info: DisplayInfo) {
        unsafe { self.as_raw_mut().setDisplayInfo(&display_info as *const _) }
    }

    pub fn set_frame_rate_options(&mut self, frame_rate_options: FrameRateOptions) {
        unsafe {
            self.as_raw_mut()
                .setFrameRateOptions(&frame_rate_options as *const _)
        }
    }

    pub fn set_clear_options(&mut self, clear_options: ClearOptions) {
        unsafe {
            self.as_raw_mut()
                .setClearOptions(&clear_options as *const _)
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }

    pub fn render(&mut self, view: &View) {
        unsafe { self.as_raw_mut().render(view.as_raw_ptr() as _) }
    }

    pub fn copy_frame(
        &mut self,
        dst_swapchain: &mut SwapChain,
        dst_viewport: Viewport,
        src_viewport: Viewport,
        flags: CopyFrameFlags,
    ) {
        unsafe {
            self.as_raw_mut().copyFrame(
                dst_swapchain.as_raw_ptr(),
                &dst_viewport as *const _ as _,
                &src_viewport as *const _ as _,
                flags.bits,
            )
        }
    }

    pub fn begin_frame(
        &mut self,
        swapchain: &mut SwapChain,
        vsync_steady_clock_time_nano: u64,
    ) -> bool {
        unsafe {
            self.as_raw_mut().beginFrame(
                swapchain.as_raw_ptr(),
                vsync_steady_clock_time_nano,
                None,
                std::ptr::null_mut(),
            )
        }
    }

    pub fn end_frame(&mut self) {
        unsafe { self.as_raw_mut().endFrame() }
    }

    pub fn user_time(&self) -> f64 {
        unsafe { self.as_raw_ref().getUserTime() }
    }

    pub fn reset_user_time(&mut self) {
        unsafe { self.as_raw_mut().resetUserTime() }
    }

    pub fn new(engine: &mut Engine) -> Result<Self, EngineError> {
        let ptr = unsafe { engine.as_raw_mut().createRenderer() };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }
}
