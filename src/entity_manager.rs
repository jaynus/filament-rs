use filament_sys::ffi;
use std::convert::TryInto;

pub struct EntityManager<'a> {
    inner: *mut ffi::utils_EntityManager,
    _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> EntityManager<'a> {
    pub fn get() -> EntityManager<'a> {
        Self {
            inner: unsafe { &mut *ffi::utils_EntityManager_get() },
            _marker: std::marker::PhantomData::default(),
        }
    }

    pub fn destroy(&self, entities: &[crate::Entity]) {
        unsafe {
            ffi::utils_EntityManager_destroy(
                self.inner,
                entities.len().try_into().unwrap(),
                entities.as_ptr() as *mut _,
            );
        }
    }

    pub fn create(&self) -> crate::Entity {
        let mut buffer: [crate::Entity; 1] = [crate::Entity::default(); 1];

        unsafe {
            ffi::utils_EntityManager_create(self.inner, 1, buffer.as_mut_ptr());
        }
        buffer[0]
    }

    pub fn create_vec(&self, count: usize) -> Vec<crate::Entity> {
        let mut buffer = Vec::with_capacity(count);
        buffer.resize(count, crate::Entity::default());

        unsafe {
            ffi::utils_EntityManager_create(
                self.inner,
                count.try_into().unwrap(),
                buffer.as_mut_ptr(),
            );
        }
        buffer
    }

    pub fn fill_slice(&self, entities: &mut [crate::Entity]) {
        unsafe {
            ffi::utils_EntityManager_create(
                self.inner,
                entities.len().try_into().unwrap(),
                entities.as_mut_ptr(),
            );
        }
    }
}
