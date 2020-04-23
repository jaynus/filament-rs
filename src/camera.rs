use filament_sys::ffi;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use std::sync::Arc;

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum Projection {
    PERSPECTIVE = ffi::filament_Camera_Projection_PERSPECTIVE,
    ORTHOGRAPHIC = ffi::filament_Camera_Projection_ORTHO,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum FovDirection {
    VERTICAL = ffi::filament_Camera_Fov_VERTICAL,
    HORIZONTAL = ffi::filament_Camera_Fov_HORIZONTAL,
}
impl Default for FovDirection {
    fn default() -> Self {
        Self::VERTICAL
    }
}

impl_handle!(Camera, ffi::filament_Camera);
impl Drop for Camera {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy17(*ptr) }
        }
    }
}
impl Camera {
    pub fn set_projection(
        &mut self,
        projection: Projection,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) {
        unsafe {
            self.as_raw_mut().setProjection(
                projection.to_i32().unwrap(),
                left,
                right,
                bottom,
                top,
                near,
                far,
            )
        }
    }
    pub fn set_projection_fov(
        &mut self,
        fov: f64,
        aspect: f64,
        near: f64,
        far: f64,
        direction: FovDirection,
    ) {
        unsafe {
            self.as_raw_mut()
                .setProjection1(fov, aspect, near, far, direction.to_i32().unwrap())
        }
    }

    pub fn new(engine: &mut Engine, entity: crate::Entity) -> Result<Self, EngineError> {
        let ptr = unsafe { engine.as_raw_mut().createCamera(entity) };

        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }

    pub fn new_orphan(engine: &mut Engine) -> Result<Self, EngineError> {
        let ptr = unsafe { engine.as_raw_mut().createCamera1() };

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
