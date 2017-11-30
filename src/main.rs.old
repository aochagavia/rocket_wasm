#![deny(missing_docs)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

//! A 2D toy game written in Rust, using the Piston library.

extern crate piston_window;
extern crate itertools_num;
extern crate opengl_graphics;
extern crate rand;

mod controllers;
mod drawing;
mod game_state;
mod geometry;
mod models;
mod resources;
mod util;
mod view;

use piston_window::{Button, EventLoop, Input, Motion, OpenGL, PistonWindow, WindowSettings};
use opengl_graphics::GlGraphics;

use controllers::{CollisionsController, InputController, TimeController};
use game_state::GameState;
use geometry::Size;
use resources::Resources;

fn main() {
    let opengl = OpenGL::V3_2;

    let game_size = Size::new(1024.0, 600.0);

    let mut window: PistonWindow = WindowSettings::new(
        "Rocket!", [game_size.width as u32, game_size.height as u32])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);
    let mut resources = Resources::new();
    let mut input_controller = InputController::new();
    let mut time_controller = TimeController::new();
    let mut state = GameState::new(game_size);

    // The game loop
    while let Some(e) = window.next() {
        // Event handling
        match e {
            Input::Press(Button::Keyboard(key)) => {
                input_controller.key_press(key);
            }

            Input::Release(Button::Keyboard(key)) => {
                input_controller.key_release(key);
            }

            Input::Press(Button::Controller(button)) => {
                input_controller.button_press(button);
            }

            Input::Release(Button::Controller(button)) => {
                input_controller.button_release(button);
            }

            Input::Move(Motion::ControllerAxis(axis)) => {
                input_controller.handle_axis(axis);
            }

            Input::Update(args) => {
                time_controller.update_seconds(args.dt, input_controller.actions(), &mut state);
                CollisionsController::handle_collisions(&mut state);
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |c, g| view::render_game(c, g, &mut resources, &state));
            }

            _ => {}
        }
    }
}
