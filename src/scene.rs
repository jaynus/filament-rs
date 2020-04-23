use filament_sys::ffi;
use std::{convert::TryInto, sync::Arc};

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
};

impl_handle!(Scene, ffi::filament_Scene);
impl Drop for Scene {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy9(*ptr) }
        }
    }
}
impl Scene {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.renderable_count() + self.light_count()
    }

    #[inline]
    pub fn renderable_count(&self) -> usize {
        unsafe { self.as_raw_ref().getRenderableCount().try_into().unwrap() }
    }

    #[inline]
    pub fn light_count(&self) -> usize {
        unsafe { self.as_raw_ref().getLightCount().try_into().unwrap() }
    }

    #[inline]
    pub fn contains(&self, entity: crate::Entity) -> bool {
        unsafe { self.as_raw_ref().hasEntity(entity) }
    }

    #[inline]
    pub fn push(&mut self, entity: crate::Entity) {
        unsafe { self.as_raw_mut().addEntity(entity) }
    }

    pub fn extend<I>(&mut self, entities: I)
    where
        I: Iterator<Item = crate::Entity>,
    {
        entities.for_each(|e| self.push(e));
    }

    #[inline]
    pub fn extend_from_slice(&mut self, entities: &[crate::Entity]) {
        unsafe {
            self.as_raw_mut()
                .addEntities(entities.as_ptr(), entities.len().try_into().unwrap())
        }
    }

    #[inline]
    pub fn remove(&mut self, entity: crate::Entity) {
        unsafe { self.as_raw_mut().remove(entity) }
    }

    pub fn new(engine: &mut Engine) -> Result<Self, EngineError> {
        let ptr = unsafe { engine.as_raw_mut().createScene() };

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
