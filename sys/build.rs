//CC=clang CXX=clang++ CXXFLAGS=-stdlib=libc++ cmake -DFILAMENT_ENABLE_JAVA=off -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=../release/filament -DFILAMENT_SUPPORTS_VULKAN=ON -DUSE_STATIC_LIBCXX=off ../..
fn main() {
    println!("cargo:rerun-if-changed=proxy/wrapper.h");

    // Build filament
    let filament_dst = cmake::Config::new("../filament")
        // Set compiler to clang
        .define("CMAKE_C_COMPILER", "clang")
        .define("CMAKE_CXX_COMPILER", "clang++")
        // Set build specific settings
        .define("FILAMENT_SKIP_SAMPLES", "on")
        .define("FILAMENT_SUPPORTS_VULKAN", "on")
        .define("USE_STATIC_LIBCXX", "off")
        .cxxflag("-stdlib=libc++")
        // do it fast
        .build_arg("-j32")
        .build();

    // Linking and rebuild settings
    println!(
        "cargo:rustc-link-search=native={}/lib/{}",
        filament_dst.display(),
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()
    );

    // Build the helper
    let mut builder = cc::Build::new();
    builder
        .cpp_set_stdlib("c++")
        .include("../filament/libs/math/include")
        .include("../filament/libs/utils/include")
        .include("../filament/libs/filabridge/include")
        .include("../filament/filament/backend/include")
        .include("../filament/filament/include")
        .file("src/cpp/helpers.cpp")
        .file("src/cpp/materials.cpp")
        .file("src/cpp/renderable_manager.cpp")
        .include("src");

    #[cfg(debug_assertions)]
    {
        builder.file("src/cpp/tests.cpp");
    }
    builder.compile("filament-helpers");

    // Filament requirements
    println!("cargo:rustc-link-lib=static=filament");
    println!("cargo:rustc-link-lib=static=backend");
    println!("cargo:rustc-link-lib=static=bluegl");
    println!("cargo:rustc-link-lib=static=bluevk");
    println!("cargo:rustc-link-lib=static=filabridge");
    println!("cargo:rustc-link-lib=static=filaflat");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-lib=static=geometry");
    println!("cargo:rustc-link-lib=static=smol-v");
    println!("cargo:rustc-link-lib=static=ibl");

    println!("cargo:rustc-link-lib=c++");

    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-lib=static=matdbg");
    }
}
