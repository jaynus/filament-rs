use filament_sys::ffi;
use std::sync::Arc;

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
};

bitflags::bitflags! {
    pub struct SwapChainFlags: u64 {
        const TRANSPARENT = ffi::filament_SwapChain_CONFIG_TRANSPARENT;
        const READABLE = ffi::filament_SwapChain_CONFIG_READABLE;
    }
}

impl_handle!(SwapChain, ffi::filament_SwapChain);
impl Drop for SwapChain {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy11(*ptr) }
        }
    }
}
impl SwapChain {
    pub fn new(
        engine: &mut Engine,
        raw_window_handle: *mut ::std::os::raw::c_void,
        flags: SwapChainFlags,
    ) -> Result<Self, EngineError> {
        let ptr = unsafe {
            engine
                .as_raw_mut()
                .createSwapChain(raw_window_handle, flags.bits)
        };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }

    pub fn new_headless(
        engine: &mut Engine,
        width: u32,
        height: u32,
        flags: SwapChainFlags,
    ) -> Result<Self, EngineError> {
        let ptr = unsafe {
            engine
                .as_raw_mut()
                .createSwapChain1(width, height, flags.bits)
        };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }

    pub fn raw_handle(&self) -> *mut ::std::os::raw::c_void {
        unsafe { self.as_raw_ref().getNativeWindow() }
    }
}
