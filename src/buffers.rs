use filament_sys::ffi;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use std::{convert::TryInto, sync::Arc};

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
};

pub use ffi::filament_VertexAttribute as VertexAttribute;
pub use ffi::filament_VertexBuffer_AttributeType as AttributeType;
pub use ffi::filament_VertexBuffer_QuatTangentContext as QuatTangentContext;

use ffi::filament_backend_BufferDescriptor as BufferDescriptor;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum IndexType {
    USHORT = ffi::filament_IndexBuffer_IndexType_USHORT,
    UINT = ffi::filament_IndexBuffer_IndexType_UINT,
}

pub struct VertexBufferBuilder {
    inner: ffi::filament_VertexBuffer_Builder,
}
impl Drop for VertexBufferBuilder {
    fn drop(&mut self) {
        log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
        unsafe { self.inner.destruct() }
    }
}
impl VertexBufferBuilder {
    pub fn buffer_count(mut self, count: usize) -> Self {
        unsafe {
            self.inner.bufferCount(count.try_into().unwrap());
        }

        self
    }
    pub fn vertex_count(mut self, count: usize) -> Self {
        unsafe {
            self.inner.vertexCount(count.try_into().unwrap());
        }

        self
    }

    pub fn attribute(
        mut self,
        attribute: VertexAttribute,
        index: usize,
        ty: AttributeType,
        offset: usize,
        stride: usize,
    ) -> Self {
        unsafe {
            self.inner.attribute(
                attribute,
                index.try_into().unwrap(),
                ty,
                offset.try_into().unwrap(),
                stride.try_into().unwrap(),
            );
        }

        self
    }

    pub fn normalized(mut self, attribute: VertexAttribute, normalized: bool) -> Self {
        unsafe {
            self.inner.normalized(attribute, normalized);
        }
        self
    }

    pub fn build(mut self, engine: &mut Engine) -> Result<VertexBuffer, EngineError> {
        let ptr = unsafe { self.inner.build(engine.as_raw_ptr()) };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(VertexBuffer {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }

    pub fn new() -> Self {
        Self {
            inner: unsafe { ffi::filament_VertexBuffer_Builder::new() },
        }
    }
}

impl_handle!(VertexBuffer, ffi::filament_VertexBuffer);
impl Drop for VertexBuffer {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy2(*ptr) }
        }
    }
}
impl VertexBuffer {
    pub fn write_at<T: Sized>(&mut self, index: usize, offset: usize, buffer: Vec<T>) {
        let mut desc = make_buffer_descriptor(buffer);
        unsafe {
            let engine = self.engine.as_raw_ptr();
            self.as_raw_mut().setBufferAt(
                engine,
                index.try_into().unwrap(),
                &mut desc as *mut _,
                offset.try_into().unwrap(),
            )
        };
        std::mem::forget(desc);
    }

    pub fn vertex_count(&self) -> usize {
        unsafe { self.as_raw_ref().getVertexCount().try_into().unwrap() }
    }

    pub fn populate_tangent_quaternions(ctx: &QuatTangentContext) {
        unsafe { ffi::filament_VertexBuffer::populateTangentQuaternions(ctx as *const _) }
    }

    pub fn builder() -> VertexBufferBuilder {
        VertexBufferBuilder::new()
    }
}

pub struct IndexBufferBuilder {
    inner: ffi::filament_IndexBuffer_Builder,
}
impl Drop for IndexBufferBuilder {
    fn drop(&mut self) {
        log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
        unsafe { self.inner.destruct() }
    }
}
impl IndexBufferBuilder {
    pub fn index_count(mut self, count: usize) -> Self {
        unsafe {
            self.inner.indexCount(count.try_into().unwrap());
        }

        self
    }

    pub fn ty(mut self, ty: IndexType) -> Self {
        unsafe {
            self.inner.bufferType(ty.to_u8().unwrap());
        }

        self
    }

    pub fn build(mut self, engine: &mut Engine) -> Result<IndexBuffer, EngineError> {
        let ptr = unsafe { self.inner.build(engine.as_raw_ptr()) };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(IndexBuffer {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }

    pub fn new() -> Self {
        Self {
            inner: unsafe { ffi::filament_IndexBuffer_Builder::new() },
        }
    }
}

impl_handle!(IndexBuffer, ffi::filament_IndexBuffer);
impl Drop for IndexBuffer {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy4(*ptr) }
        }
    }
}
impl IndexBuffer {
    pub fn index_count(&self) -> usize {
        unsafe { self.as_raw_ref().getIndexCount().try_into().unwrap() }
    }

    pub fn write<T: Sized>(&mut self, offset: usize, buffer: Vec<T>) {
        let mut desc = make_buffer_descriptor(buffer);
        unsafe {
            let engine = self.engine.as_raw_ptr();
            self.as_raw_mut()
                .setBuffer(engine, &mut desc as *mut _, offset.try_into().unwrap())
        };
        std::mem::forget(desc);
    }

    pub fn builder() -> IndexBufferBuilder {
        IndexBufferBuilder::new()
    }
}

pub fn make_buffer_descriptor<T: Sized>(mut data: Vec<T>) -> BufferDescriptor {
    let desc = BufferDescriptor {
        buffer: data.as_mut_ptr() as *mut _,
        size: (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
        callback: Some(filament_sys::deallocate_rust_buffer),
        user: std::ptr::null_mut(),
    };
    std::mem::forget(data);
    desc
}
