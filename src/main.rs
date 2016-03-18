// TODO: Make an input module that will allow code based input configuration
// Should have options like whileDown(Keyboard::Key(W), MoveForward)
// onPress(Mouse::Button(1), FireWeapon)
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::{ HashMap, HashSet };

#[derive(Debug)]
pub enum GameAction {
    MoveForward(f32),
    MoveBackward(f32),
    MoveLeft(f32),
    MoveRight(f32),
    Trigger(bool)
}

struct KeyboardState {
    keys_down: HashSet<Key>,
    new_keys: HashSet<Key>
}

impl KeyboardState {
    fn new() -> KeyboardState {
        return KeyboardState {
            keys_down: HashSet::new(),
            new_keys: HashSet::new()
        }
    }
}

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    keyboard_state: KeyboardState
}

impl App {
    fn new(opengl: OpenGL) -> App {
        return App {
            gl: GlGraphics::new(opengl),
            rotation: 0.0,
            keyboard_state: KeyboardState::new()
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 1.0, 0.5];
        const RECT_SIZE : f64 = 100.0;

        let square = rectangle::square(0.0, 0.0, RECT_SIZE);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-(RECT_SIZE / 2.0), -(RECT_SIZE / 2.0));

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.

        // println!("{:?}", self.keyboard_state.keys_down);
        println!("{:?}", self.keyboard_state.new_keys);
        self.keyboard_state.new_keys.clear();
        self.rotation += 6.0 * args.dt;
    }

    fn handle_input(&mut self, input_event: &Input) {
        match *input_event {
            Input::Press(Button::Keyboard(key)) => {
                let new_insert = self.keyboard_state.keys_down.insert(key);
                if(new_insert) {
                    self.keyboard_state.new_keys.insert(key);
                }
            },
            Input::Release(Button::Keyboard(key)) => {
                self.keyboard_state.keys_down.remove(&key);
            },
            Input::Move(Motion::MouseCursor(x, y)) => println!("Mouse is at ({}, {})", x, y),
            _ => println!("Unhandled input {:?}", input_event),
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "top-down",
            [640, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Render(r) => app.render(&r),
            Event::Update(u) => app.update(&u),
            Event::Input(i) => app.handle_input(&i),
            Event::AfterRender(_) => {},
            Event::Idle(_) => {},
        }
    }
}