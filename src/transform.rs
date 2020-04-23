use filament_sys::ffi;
use std::{convert::TryInto, sync::Arc};

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
    sys::{FromFilament, IntoFilament},
    Entity,
};

pub use ffi::filament_TransformManager_Instance as TransformInstance;

// No drop, we get this as a ref from the engine
pub struct TransformManager<'a> {
    ptr: *mut ffi::filament_TransformManager,
    _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> TransformManager<'a> {
    pub fn create(
        &mut self,
        entity: Entity,
        parent: Option<TransformInstance>,
        transform: [f64; 16],
    ) {
    }

    pub fn get(&self, entity: Entity) -> Option<TransformInstance> {
        let instance = unsafe { (*self.ptr).getInstance(entity) };
        if instance == 0 {
            None
        } else {
            Some(instance)
        }
    }

    pub fn set_transform(&mut self, instance: TransformInstance, transform: [f32; 16]) {
        unsafe { (*self.ptr).setTransform(instance, &transform.into_filament() as *const _) }
    }
    pub fn get_transform(&self, instance: TransformInstance) -> Option<[f32; 16]> {
        let ptr = unsafe { (*self.ptr).getTransform(instance) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { (*ptr).into_array() })
        }
    }
    pub fn remove(&mut self, instance: TransformInstance) {}

    pub fn set_parent(&mut self, instance: TransformInstance, parent: Option<TransformInstance>) {}
    pub fn get_parent(&self, instance: TransformInstance) -> Option<TransformInstance> {
        unimplemented!()
    }

    pub(crate) fn new(engine: &'a Engine) -> Result<TransformManager<'a>, EngineError> {
        let ptr = unsafe { ffi::filament_Engine_getTransformManager(engine.as_raw_ptr()) };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr,
                _marker: Default::default(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransformManager;
    use crate::{engine::Engine, entity_manager::EntityManager, sys::ffi, Backend, Camera};

    #[test]
    fn transform_setget() {
        #[rustfmt::skip]
        let identity = [1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0,];
        #[rustfmt::skip]
        let test_transform = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, ];

        let mut engine = Engine::new(Backend::NOOP).unwrap();
        let camera_entity = EntityManager::get().create();
        let camera = Camera::new(&mut engine, camera_entity).unwrap();
        {
            let mut transform_manager = engine.transform_manager().unwrap();
            let camera_transform = engine
                .transform_manager()
                .unwrap()
                .get(camera_entity)
                .unwrap();

            assert_eq!(
                transform_manager.get_transform(camera_transform).unwrap(),
                identity
            );

            transform_manager.set_transform(camera_transform, test_transform);

            assert_eq!(
                transform_manager.get_transform(camera_transform).unwrap(),
                test_transform,
            );
        }
    }
}
