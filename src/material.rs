use filament_sys::ffi;
use std::{convert::TryInto, ffi::CString, sync::Arc};

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
    texture::{Texture, TextureSampler},
};

pub enum MaterialParameter<'a> {
    Texture(&'a Texture, &'a TextureSampler),
}

impl_handle!(MaterialInstance, ffi::filament_MaterialInstance);
impl MaterialInstance {
    pub fn set<S>(&mut self, name: S, parameter: MaterialParameter)
    where
        S: AsRef<str>,
    {
        let name = CString::new(name.as_ref()).unwrap();
        match parameter {
            MaterialParameter::Texture(texture, sampler) => unsafe {
                ffi::helpers_material_instance_setParameter_texture(
                    self.as_raw_ptr(),
                    name.as_ptr(),
                    texture.as_raw_ptr(),
                    sampler as *const _,
                )
            },
        }
    }
}

impl_handle!(Material, ffi::filament_Material);
impl Drop for Material {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            //unsafe { self.engine.as_raw_mut().destroy9(*ptr) }
        }
    }
}
impl Material {
    pub fn default_instance(&self) -> MaterialInstance {
        unsafe {
            MaterialInstance {
                engine: self.engine.clone(),
                ptr: Arc::new(ffi::helpers_material_getDefaultInstance(self.as_raw_ptr())),
            }
        }
    }

    pub fn new(engine: &mut Engine, package: &[u8]) -> Result<Self, EngineError> {
        let ptr = unsafe {
            ffi::helpers_material_build(
                engine.as_raw_ptr(),
                package.as_ptr() as _,
                package.len().try_into().unwrap(),
            )
        };

        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                engine: engine.clone(),
                ptr: Arc::new(ptr),
            })
        }
    }
}
