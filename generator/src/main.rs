use std::{collections::HashMap, iter::FromIterator, path::Path};

const RENAME_LIST: &'static [(&str, &str)] = &[];

#[derive(Debug)]
pub struct BindgenCallbacks {
    rename_lookup: HashMap<String, String>,
}
impl bindgen::callbacks::ParseCallbacks for BindgenCallbacks {
    fn item_name(&self, original_name: &str) -> Option<String> {
        self.rename_lookup.get(original_name).cloned()
    }
}

fn main() {
    env_logger::init();
    let mut builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(BindgenCallbacks {
            rename_lookup: HashMap::from_iter(
                RENAME_LIST
                    .into_iter()
                    .map(|v| (v.0.to_string(), v.1.to_string())),
            ),
        }))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-Ifilament/libs/math/include")
        .clang_arg("-Ifilament/libs/utils/include")
        .clang_arg("-Ifilament/libs/filabridge/include")
        .clang_arg("-Ifilament/filament/backend/include")
        .clang_arg("-Ifilament/filament/include")
        .header("filament/filament/include/filament/Engine.h")
        .header("filament/filament/include/filament/Renderer.h")
        .header("filament/filament/include/filament/SwapChain.h")
        .header("filament/filament/include/filament/Scene.h")
        .header("filament/filament/include/filament/View.h")
        .header("filament/filament/include/filament/Texture.h")
        .header("filament/filament/include/filament/TextureSampler.h")
        .header("filament/filament/include/filament/Camera.h")
        .header("filament/filament/include/filament/VertexBuffer.h")
        .header("filament/filament/include/filament/IndexBuffer.h")
        .header("filament/libs/utils/include/utils/EntityManager.h")
        .header("filament/libs/utils/include/utils/EntityInstance.h")
        .header("filament/filament/include/filament/TransformManager.h")
        .header("sys/src/cpp/helpers.h")
        .header("sys/src/cpp/renderable_manager.h")
        .header("sys/src/cpp/materials.h")
        .disable_untagged_union()
        .blacklist_type("std::.*")
        .blacklist_type("filament::math::mat4f")
        .opaque_type("filament::math::.*")
        .blacklist_type("filament::utils::.*")
        .opaque_type("filament::MaterialInstance")
        .opaque_type("filament::Material")
        .whitelist_type("filament::Engine")
        .whitelist_type("filament::Renderer")
        .whitelist_type("filament::SwapChain")
        .whitelist_type("filament::Scene")
        .whitelist_type("filament::View")
        .whitelist_type("filament::Viewport")
        .whitelist_type("filament::Camera")
        .whitelist_type("filament::VertexBuffer")
        .whitelist_type("filament::IndexBuffer")
        .whitelist_type("filament::backend::Viewport")
        .whitelist_type("filament::Texture")
        .whitelist_type("filament::TextureSampler")
        .whitelist_type("filament::TransformManager")
        .whitelist_type("utils::EntityManager")
        .whitelist_type("utils::EntityInstance")
        .whitelist_type("filament::backend::.*")
        .whitelist_function("helpers::.*")
        .whitelist_function("test::.*")
        .whitelist_type("test::.*")
        .whitelist_type("filament::backend::SamplerParams")
        .whitelist_type("filament::backend::PixelBufferDescriptor")
        .rustified_enum("filament::Camera::Projection")
        .rustified_enum("filament::Backend")
        .rustified_enum("filament::backend::.*")
        .rustified_enum("filament::VertexBuffer::AttributeType")
        .rustified_enum("filament::VertexBuffer::QuatType")
        .rustified_enum("filament::VertexAttribute")
        .rustified_enum("filament::IndexBuffer::IndexType")
        .opaque_type("filament::RenderableManager");

    #[cfg(debug_assertions)]
    {
        builder = builder.header("sys/src/cpp/tests.h");
    }

    let bindings = builder.generate().expect("Failed");

    bindings
        .write_to_file(Path::new("sys/src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
