extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::convert::TryInto;
use std::time::Duration;

mod state;

static HEIGHT: u32 = 600;
static WIDTH: u32 = 800;

// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/ttf-demo.rs

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    // let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Sokoban", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut game = state::GameState::new("src/assets/sokoban.txt");
    // let mut font = ttf_context
    //     .load_font("src/assets/Cocogoose Pro-trial.ttf", 128)
    //     .unwrap();
    // let surface = font
    //     .render("Hello Rust!")
    //     .blended(Color::RGBA(255, 0, 0, 255))
    //     .map_err(|e| e.to_string());
    // let texture = texture_creator
    //     .create_texture_from_surface(&surface)
    //     .map_err(|e| e.to_string());

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
                } => {
                    game.attempt_move(state::Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    game.attempt_move(state::Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    game.attempt_move(state::Direction::Down);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    game.attempt_move(state::Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::U),
                    ..
                } => {
                    game.undo();
                }
                _ => {}
            }
        }

        let arr = game.history.last().unwrap();
        // let height: u32 = HEIGHT / arr.num_rows() as u32;

        for (i, row) in arr.rows_iter().enumerate() {
            for (j, col) in row.enumerate() {
                let color = match col {
                    '$' => Color::RGB(100, 100, 100),
                    '@' => Color::RGB(100, 0, 0),
                    '#' => Color::RGB(0, 100, 0),
                    '.' => Color::RGB(0, 255, 0),
                    '*' => Color::RGB(150, 150, 150),
                    '+' => Color::RGB(150, 0, 0),
                    _ => Color::RGBA(0, 0, 0, 0),
                };
                canvas.set_draw_color(color);

                canvas
                    .fill_rect(Rect::new(
                        (100 + j * 50).try_into().unwrap(),
                        (100 + i * 50).try_into().unwrap(),
                        50,
                        50,
                    ))
                    .unwrap();
            }
        }

        if game.victory {
            game.next();
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
