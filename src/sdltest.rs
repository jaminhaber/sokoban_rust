extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashSet;
use std::time::Duration;

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL Test", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut prev_keys = HashSet::new();

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => println!("UP"),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => println!("LEFT"),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => println!("Down"),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => println!("Right"),
                _ => {}
            }
        }

        // Create a set of pressed Keys.
        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
        }

        prev_keys = keys;
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
