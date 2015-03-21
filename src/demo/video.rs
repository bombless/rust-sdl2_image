use sdl2;
use sdl2_image;
use sdl2_image::LoadSurface;
use sdl2_image::LoadTexture;

pub fn main(png: &Path) {
    let sdl2_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

    let window = match sdl2::video::Window::new(
            "rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered,
            sdl2::video::WindowPos::PosCentered, 800, 600, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(
            window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    // Load a surface, and convert it to a texture bound to the renderer
    let surface = match LoadSurface::from_file(png) {
        Ok(surface) => surface,
        Err(err) => panic!(format!("Failed to load png: {}", err))
    };
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err) => panic!(format!("Failed to create surface: {}", err))
    };

    // Load a texture directly via the renderer
    let texture = match renderer.load_texture(png) {
        Ok(texture) => texture,
        Err(err) => panic!(format!("Could not set render target: {}", err))
    };

    let mut drawer = renderer.drawer();
    let _ = drawer.copy(&texture, None, None);
    drawer.present();

    let mut event_pump = sdl2_context.event_pump();
    'main : loop {
        'event : for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit{..} => break 'main,
                sdl2::event::Event::KeyDown{ keycode: key, .. } => {
                    if key == sdl2::keycode::KeyCode::Escape {
                        break 'main
                    }
                }
                _ => {}
            }
        }
    }
    sdl2_image::quit();
}
