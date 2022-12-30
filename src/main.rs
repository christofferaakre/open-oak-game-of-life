use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use life::{Grid, Object};
use open_oak::{
    events::handle_events,
    glutin::event::VirtualKeyCode,
    init::{init, Game},
    resource_manager::ResourceManager,
    shapes::rect::Rectangle,
    traits::{Renderable, Shaders},
};

use open_oak::Surface;
use open_oak::{Rad, Rgba, Vector2};

fn main() {
    let mut grid = Grid::new(64, 64);

    let glider = Object::from_file("objects/glider.life");
    grid.load_object(&glider, (0, 0));

    let game = init();

    let Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = game;

    // init rectangle
    Rectangle::init(&mut resource_manager, &display);

    let texture_name = String::from("cell");
    let texture = ResourceManager::load_texture(&display, "textures/cell.png");
    resource_manager.add_texture(&texture_name, texture);

    const ALIVE_COLOR: Rgba<f32> = Rgba([0.0, 1.0, 0.0, 1.0]);
    const DEAD_COLOR: Rgba<f32> = Rgba([0.0, 0.1, 0.0, 1.0]);

    let mut rects: Vec<Vec<Rectangle>> = grid
        .cells
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    Rectangle::new(
                        Vector2::new(
                            ((x as f32) * 1.0 + 0.5) / grid.width as f32,
                            ((y as f32) * 1.0 + 0.5) / grid.height as f32,
                        ),
                        Vector2::new(0.9 / grid.width as f32, 0.9 / grid.height as f32),
                        Rad(0.0),
                        DEAD_COLOR,
                        texture_name.clone(),
                    )
                })
                .collect()
        })
        .collect();

    const SECONDS_PER_GENERATION: f32 = 0.1;
    let mut timer = Duration::new(0, 0);

    let mut pressed_keys: HashSet<VirtualKeyCode> = HashSet::new();
    let mut last_frame = Instant::now();
    // game loop
    event_loop.run(move |ev, _, _control_flow| {
        let keyboard_input = handle_events(ev, &mut pressed_keys);
        if let Some(keyboard_input) = keyboard_input {
            handle_keyboard_input(keyboard_input, &pressed_keys);
        }
        // calculate time since last frame
        let dt = last_frame.elapsed();
        last_frame += dt;
        timer += dt;
        if timer.as_secs_f32() > SECONDS_PER_GENERATION {
            timer = Duration::new(0, 0);
            grid.advance();
        }

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        // DRAW START
        for (y, row) in rects.iter_mut().enumerate() {
            for (x, rect) in row.iter_mut().enumerate() {
                rect.color = match grid.cells[y][x].alive {
                    true => ALIVE_COLOR,
                    false => DEAD_COLOR,
                };

                rect.draw(&mut frame, &resource_manager).unwrap();
            }
        }

        frame.finish().unwrap();
        // DRAW END
    });
}

fn handle_keyboard_input(
    keyboard_input: open_oak::glutin::event::KeyboardInput,
    _pressed_keys: &HashSet<VirtualKeyCode>,
) {
    match keyboard_input.virtual_keycode.unwrap() {
        VirtualKeyCode::Escape => {
            std::process::exit(0);
        }
        _ => {}
    }
}
