// TODO: Make an input module that will allow code based input configuration
// Should have options like whileDown(Keyboard::Key(W), MoveForward)
// onPress(Mouse::Button(1), FireWeapon)
extern crate piston;
extern crate vecmath;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate float;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::{ HashMap, HashSet };
use float::Radians;

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

struct PlayerState {
    velocity: vecmath::Vector2<f64>,
    location: vecmath::Vector2<f64>,
    max_speed: f64,
    angle: f64
}

struct ControlState {
    velocity: vecmath::Vector2<f64>,
    angle: f64,
    aiming_point: vecmath::Vector2<f64>
}

pub struct App {
    gl: GlGraphics,
    player_state: PlayerState,
    control_state: ControlState,
    keyboard_state: KeyboardState, // TODO: This is input state
    move_mapping: HashMap<Key, vecmath::Vector2<f64>> // TODO: this is input state/configuration?
}

fn vec2_is_zero(vec:vecmath::Vector2<f64>) -> bool {
    vec[0] == 0.0 && vec[1] == 0.0
}

fn vec2_rotate(vec: vecmath::Vector2<f64>, rotation: f64) -> vecmath::Vector2<f64> {
    let rotation_matrix =  [
        [rotation.cos(), -rotation.sin(), 0.0],
        [rotation.sin(), rotation.cos(), 0.0]
    ];


    vecmath::row_mat2x3_transform_vec2(rotation_matrix, vec)
}

fn get_rotation(pos1: vecmath::Vector2<f64>, pos2: vecmath::Vector2<f64>) -> f64 {
    (pos2[1] - pos1[1]).atan2(pos2[0] - pos1[0])
}

impl App {
    fn new(opengl: OpenGL, (width, height): (f64, f64)) -> App {
        let mut move_mapping = HashMap::new();
        move_mapping.insert(Key::W, [0.0, -1.0]);
        move_mapping.insert(Key::S, [0.0, 1.0]);
        move_mapping.insert(Key::A, [-1.0, 0.0]);
        move_mapping.insert(Key::D, [1.0, 0.0]);

        return App {
            gl: GlGraphics::new(opengl),
            control_state: ControlState {
                velocity: [0.0, 0.0],
                angle: 0.0,
                aiming_point: [0.0, 0.0]
            },
            player_state: PlayerState {
                velocity: [0.0, 0.0],
                max_speed: 5.0,
                location: [(width / 2.0) as f64, (height / 2.0) as f64],
                angle: 0.0
            },
            keyboard_state: KeyboardState::new(),
            move_mapping: move_mapping
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RECT_SIZE : f64 = 100.0;

        let square = rectangle::square(0.0, 0.0, RECT_SIZE);
        let player = &self.player_state;
        let rotation = player.angle;
        let x = player.location[0];
        let y = player.location[1];

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
        let mut target_velocity = [0.0, 0.0];

        for key in self.keyboard_state.keys_down.iter() {
            match self.move_mapping.get(&key) {
                Some(x) => target_velocity = vecmath::vec2_add(target_velocity, *x),
                None => {}
            }
        }

        if(!vec2_is_zero(target_velocity)) {
            target_velocity = vecmath::vec2_normalized(target_velocity);
        }
        target_velocity = vecmath::vec2_scale(target_velocity, self.player_state.max_speed);

        let new_velocity = if(vec2_is_zero(target_velocity)) {
            if(!vec2_is_zero(self.player_state.velocity)){
                let reverse = vec2_rotate(self.player_state.velocity, (180.0).deg_to_rad());

                let scaled = vecmath::vec2_scale(
                    vecmath::vec2_normalized(reverse),
                    0.1
                );
                vecmath::vec2_add(self.player_state.velocity, scaled)
            } else {
                [0.0, 0.0]
            }
        } else {
            let acceleration = vecmath::vec2_scale(
                vecmath::vec2_normalized(
                    target_velocity
                ),
                0.5
            );

            let mut new_velocity = vecmath::vec2_add(self.player_state.velocity, acceleration);

            if(vecmath::vec2_len(new_velocity) > self.player_state.max_speed) {
                vecmath::vec2_scale(
                    vecmath::vec2_normalized(new_velocity),
                    self.player_state.max_speed
                )
            } else {
                new_velocity
            }
        };

        self.player_state.velocity = new_velocity;
        self.player_state.location = vecmath::vec2_add(self.player_state.location, self.player_state.velocity);
        self.keyboard_state.new_keys.clear();
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
            Input::Text(_) => {},
            Input::Move(Motion::MouseCursor(x, y)) => self.control_state.aiming_point = [x, y],
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
    let mut app = App::new(opengl, (640.0, 480.0));

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