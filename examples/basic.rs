use filament::{
    camera::{Camera, Projection},
    entity_manager::EntityManager,
    material::{Material, MaterialParameter},
    renderable::{PrimitiveType, RenderableBuilder},
    sys::Viewport,
    texture::{InternalFormat, PixelDataFormat, PixelDataType, Texture, TextureSampler},
    AttributeType, Backend, ClearOptions, Engine, FovDirection, IndexBuffer, IndexType, Renderer,
    Scene, SwapChain, SwapChainFlags, VertexAttribute, VertexBuffer, View,
};
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const MATERIAL_BYTES: &'static [u8] = include_bytes!("texture_unlit.filamat");

#[repr(C)]
#[derive(Clone, Default)]
struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

#[cfg(target_os = "macos")]
fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::filament_c_void {
    use winit::os::macos::WindowExt;
    window.get_nsview()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::filament_c_void {
    use winit::os::windows::WindowExt;
    window.get_hwnd()
}

#[cfg(target_os = "linux")]
fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::c_void {
    use winit::platform::unix::WindowExtUnix;
    window.xlib_window().unwrap() as *mut std::ffi::c_void
}

fn init_window() -> (EventLoop<()>, Window, *mut std::ffi::c_void) {
    let _ = env_logger::builder().is_test(true).try_init();

    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Basic example window");

    let surface = get_active_surface(&window);

    (event_loop, window, surface)
}

fn triangle_data() -> (Vec<Vertex>, Vec<u16>, Vec<RgbColor>) {
    let mut texture_data = vec![RgbColor::default(); 256 * 256];
    for y in 0..256 {
        for x in 0..256 {
            texture_data[y * 256 + x] = RgbColor {
                r: x as u8,
                g: y as u8,
                b: 0,
            };
        }
    }

    (
        vec![
            Vertex {
                position: [1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 1.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.0],
                uv: [0.0, 0.0],
            },
        ],
        vec![0, 1, 2],
        texture_data,
    )
}

fn main() {
    env_logger::init();

    let (event_loop, window, surface) = init_window();

    let mut engine = Engine::new(Backend::DEFAULT).unwrap();
    let mut swapchain = SwapChain::new(&mut engine, surface, SwapChainFlags::empty()).unwrap();
    let mut renderer = Renderer::new(&mut engine).unwrap();
    let mut view = View::new(&mut engine).unwrap();
    let mut scene = Scene::new(&mut engine).unwrap();

    let entity_manager = EntityManager::get();

    let mut camera = Camera::new(&mut engine, entity_manager.create()).unwrap();
    //camera.set_projection_fov(60.0, 800.0 / 600.0, 0.1, 10000.0, FovDirection::default());
    let aspect = 800.0 / 600.0;
    let zoom = 1.0;
    camera.set_projection(
        Projection::ORTHOGRAPHIC,
        -aspect * zoom,
        aspect * zoom,
        -zoom,
        zoom,
        0.0,
        10.0,
    );

    view.set_viewport(Viewport::new(0, 0, 800, 600));
    view.set_scene(&scene);
    view.set_camera(&camera);
    renderer.set_clear_options(ClearOptions::default());

    let (vertices, indices, texture_data) = triangle_data();

    let mut vertex_buffer = VertexBuffer::builder()
        .vertex_count(3)
        .buffer_count(1)
        .attribute(VertexAttribute::POSITION, 0, AttributeType::FLOAT2, 0, 16)
        .attribute(VertexAttribute::UV0, 0, AttributeType::FLOAT2, 8, 16)
        .build(&mut engine)
        .unwrap();
    vertex_buffer.write_at(0, 0, vertices);

    let mut index_buffer = IndexBuffer::builder()
        .index_count(3)
        .ty(IndexType::USHORT)
        .build(&mut engine)
        .unwrap();
    index_buffer.write(0, indices);

    let sampler = TextureSampler::default();
    let mut texture = Texture::builder()
        .unwrap()
        .width(256)
        .height(256)
        .format(InternalFormat::RGB8)
        .build(&mut engine)
        .unwrap();
    texture.set(0, texture_data, PixelDataFormat::RGB, PixelDataType::UBYTE);

    let material = Material::new(&mut engine, MATERIAL_BYTES).unwrap();
    let mut instance = material.default_instance();
    instance.set("texture", MaterialParameter::Texture(&texture, &sampler));

    let renderable = entity_manager.create();
    RenderableBuilder::new(1)
        .unwrap()
        .culling(false)
        .cast_shadows(false)
        .receive_shadows(false)
        .material(0, &instance)
        .geometry(0, PrimitiveType::TRIANGLES, &vertex_buffer, &index_buffer)
        .build(&mut engine, renderable)
        .unwrap();

    scene.push(renderable);

    let mut close_requested = false;

    // Poll window events first.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        use winit::event::{ElementState, VirtualKeyCode};

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    close_requested = true;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match virtual_code {
                    VirtualKeyCode::Escape => close_requested = true,
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();

                if close_requested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(_window_id) => {
                if renderer.begin_frame(&mut swapchain, 0) {
                    renderer.render(&view);
                    renderer.end_frame();
                }
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
            _ => {}
        }
    })
}
