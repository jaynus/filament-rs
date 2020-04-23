use crate::{
    buffers::{IndexBuffer, VertexBuffer},
    material::MaterialInstance,
    Engine, EngineError,
};
use filament_sys::ffi;
use std::convert::TryInto;
use thiserror::Error;

pub use ffi::filament_backend_PrimitiveType as PrimitiveType;

#[derive(Error, Debug)]
pub enum RenderableBuildError {
    #[error("Creation of a type failed")]
    CreationFailed,
}

pub struct RenderableBuilder<'a> {
    ptr: *mut ffi::helpers_BuilderProxy,
    _marker: std::marker::PhantomData<&'a ()>,
}
impl<'a> Drop for RenderableBuilder<'a> {
    fn drop(&mut self) {
        unsafe { ffi::helpers_renderable_builder_destroy(self.ptr) }
    }
}
impl<'a> RenderableBuilder<'a> {
    pub fn material(self, index: usize, material: &MaterialInstance) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_material(
                self.ptr,
                index.try_into().unwrap(),
                material.as_raw_ptr(),
            );
        }

        self
    }

    pub fn geometry(
        self,
        index: usize,
        primitive_type: PrimitiveType,
        vertices: &'a VertexBuffer,
        indices: &'a IndexBuffer,
    ) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_geometry(
                self.ptr,
                index.try_into().unwrap(),
                primitive_type,
                vertices.as_raw_ptr(),
                indices.as_raw_ptr(),
            );
        }

        self
    }

    pub fn culling(self, value: bool) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_culling(self.ptr, value);
        }
        self
    }
    pub fn cast_shadows(self, value: bool) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_castShadows(self.ptr, value);
        }
        self
    }
    pub fn receive_shadows(self, value: bool) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_receiveShadows(self.ptr, value);
        }
        self
    }
    pub fn screen_space_contact_shadows(self, value: bool) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_screenSpaceContactShadows(self.ptr, value);
        }
        self
    }
    pub fn morphing(self, value: bool) -> Self {
        unsafe {
            ffi::helpers_renderable_builder_morphing(self.ptr, value);
        }
        self
    }

    pub fn build(
        self,
        engine: &mut Engine,
        entity: crate::Entity,
    ) -> Result<(), RenderableBuildError> {
        match unsafe {
            ffi::helpers_renderable_builder_build(self.ptr, engine.as_raw_ptr(), entity)
        } {
            true => Ok(()),
            false => Err(RenderableBuildError::CreationFailed),
        }
    }

    pub fn new(count: usize) -> Result<Self, EngineError> {
        let ptr = unsafe { ffi::helpers_renderable_builder_new(count.try_into().unwrap()) };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr,
                _marker: std::marker::PhantomData::default(),
            })
        }
    }
}
