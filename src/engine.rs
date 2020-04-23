use filament_sys::ffi;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Creation of a type failed")]
    CreationFailed,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Engine {
    ptr: std::sync::Arc<*mut ffi::filament_Engine>,
}
impl Engine {
    pub(crate) fn handle(&mut self) -> Arc<*mut ffi::filament_Engine> {
        self.ptr.clone()
    }

    pub(crate) fn as_raw_ptr(&self) -> *mut ffi::filament_Engine {
        *self.ptr
    }
    pub(crate) fn as_raw_mut(&mut self) -> &mut ffi::filament_Engine {
        unsafe { &mut **self.ptr }
    }
    pub(crate) fn as_raw_ref(&self) -> &ffi::filament_Engine {
        unsafe { &**self.ptr }
    }
}
impl Engine {
    pub fn transform_manager(&self) -> Result<crate::transform::TransformManager<'_>, EngineError> {
        crate::transform::TransformManager::new(self)
    }

    pub fn new(backend: crate::Backend) -> Result<Self, EngineError> {
        let ptr = unsafe {
            ffi::filament_Engine::create(backend, std::ptr::null_mut(), std::ptr::null_mut())
        };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self { ptr: Arc::new(ptr) })
        }
    }

    pub fn execute(&mut self) {
        unsafe { self.as_raw_mut().execute() }
    }

    pub fn flush_and_wait(&mut self) {
        unsafe { self.as_raw_mut().flushAndWait() }
    }

    pub fn backend(&self) -> crate::Backend {
        unsafe { self.as_raw_ref().getBackend() }
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { ffi::filament_Engine::destroy(ptr) }
        }
    }
}
