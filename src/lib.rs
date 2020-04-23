#![allow(dead_code)]

pub use filament_sys as sys;
use sys::ffi;

use thiserror::Error;

pub mod buffers;
pub use buffers::*;
pub mod camera;
pub use camera::*;
pub mod engine;
pub use engine::*;
pub mod renderer;
pub use renderer::*;
pub mod scene;
pub use scene::*;
pub mod swapchain;
pub use swapchain::*;
pub mod view;
pub use view::*;

pub mod texture;
pub use texture::*;

pub mod entity_manager;
pub mod renderable;

pub mod material;
pub mod transform;

pub use ffi::filament_backend_Backend as Backend;
pub use filament_sys::Viewport;

pub use ffi::utils_Entity as Entity;

#[macro_export]
macro_rules! impl_ptr_functions {
    ($ty:ident, $ptr:path) => {
        impl $ty {
            pub(crate) fn handle(&mut self) -> Arc<*mut $ptr> {
                self.ptr.clone()
            }
            pub(crate) fn as_raw_ptr(&self) -> *mut $ptr {
                *self.ptr
            }
            pub(crate) fn as_raw_mut(&mut self) -> &mut $ptr {
                unsafe { &mut **self.ptr }
            }
            pub(crate) fn as_raw_ref(&self) -> &$ptr {
                unsafe { &**self.ptr }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_handle {
    ($ty:ident, $ptr:path) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $ty {
            engine: $crate::engine::Engine,
            ptr: std::sync::Arc<*mut $ptr>,
        }
        $crate::impl_ptr_functions!($ty, $ptr);
    };
}

#[derive(Error, Debug)]
pub enum FilamentError {
    #[error("Creation of a type failed")]
    CreationFailed,
}
