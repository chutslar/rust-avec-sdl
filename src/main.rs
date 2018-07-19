extern crate sdl2;
extern crate sdl2_sys;
extern crate gl;

// Associated library
extern crate ras;

// For exiting process
use std::process;

// For FPS tracking
use std::time::{Duration, Instant};

// For SDL2 interaction
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    // Create SDL2 and Video contexts
    let sdl_context = sdl2::init().unwrap();
    let video_ctx = sdl_context.video().unwrap();
    
    // Check for controllers
    let controller_subsystem = sdl_context.game_controller().unwrap();
    let controllers = ras::controller::Controllers::new(&controller_subsystem);


    // Create the Event Pool and a new game object
    let mut event_pool = ras::events::EventPool::new();
    let mut game = ras::game::Game::new(&mut event_pool, &controllers);

    // Set OpenGL attributes
    let gl_attr = video_ctx.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Init window
    let window  = video_ctx
        .window("SDL + OpenGL Demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Create GL context
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(
        |name| video_ctx.gl_get_proc_address(name) as *const _);

    // Check ctx was initialized properly
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    // Create event pump
    let mut events = sdl_context.event_pump().unwrap();

    // Enable vsync at start, can be disabled later
    unsafe {
        enable_vsync(true);
    }

    //*** Render Test Code
    let tri = ras::graphics::Triangles::new(
        vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ]);
    //***

    // Set flag for when to stop program
    let mut is_running = true;

    // Set clear color
    unsafe {
        gl::ClearColor(0.24, 0.4, 0.27, 1.0);
    }

    // Start frame timer
    let mut now = Instant::now();
    while is_running {
        // Poll events
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    is_running = false;
                }
                _ => {}
            }
        }

        // Track framerate
        let delta_time_ms = duration_to_ms(now.elapsed());
        now = Instant::now();

        game.update(delta_time_ms);

        // Clear screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        tri.draw();

        // Swap what we just rendered onto screen
        // Remember that if vsync is enabled this is blocking
        window.gl_swap_window();
    }

    // Exit program
    process::exit(0);
}

// Wrapper for enabling SDL vsync
unsafe fn enable_vsync(flag : bool) {
    if flag {
        sdl2_sys::SDL_GL_SetSwapInterval(1);
    } else {
        sdl2_sys::SDL_GL_SetSwapInterval(0);
    }
}

// Convenience method for getting ms elapsed from duration
fn duration_to_ms(dur : Duration) -> u64 {
    dur.as_secs() * 1000 + dur.subsec_millis() as u64
}
