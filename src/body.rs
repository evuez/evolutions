use uuid::Uuid;
use rand::{XorShiftRng, SeedableRng, Rng};
use std::f32::consts::PI;


const MAX_SPEED: u8 = 100;


enum Action {
    FORWARD,
    BACKWARD,
    TURN_LEFT,
    TURN_RIGHT,
}

struct Color {
    h: u8,
    s: u8,
    v: u8,
}

pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub d: f32,
}

pub struct Body {
    name: Uuid,
    memory: Option<Vec<Action>>,
    pub pose: Pose,
    color: Color,
    rng: XorShiftRng,
}

impl Body {
    pub fn new() -> Body {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);

        Body {
            name: Uuid::new_v4(),
            memory: None,
            pose: Pose {
                x: rng.gen::<f32>(),
                y: rng.gen::<f32>(),
                d: rng.gen::<f32>()
            },
            color: Color {
                h: rng.gen::<u8>(),
                s: rng.gen::<u8>(),
                v: rng.gen::<u8>()
            },
            rng: rng
        }
    }

    fn memorize(&mut self, action: Action) {
        match self.memory {
            Some(ref mut m) => m.push(action),
            None => self.memory = Some(vec![action])
        };
    }

    pub fn run(&mut self) {
        match self.rng.gen::<u8>() {
            10 ... 20 => self.forward(),
            20 ... 30 => self.backward(),
            30 ... 40 => self.turn_left(),
            40 ... 50 => self.turn_right(),
            _ => (),
        }
    }

    fn speed(&mut self) -> u8 {
       self.rng.gen::<u8>() & MAX_SPEED
    }

    fn forward(&mut self) {
        let speed: f32 = self.speed() as f32;

        self.pose.x += speed.cos();
        self.pose.y += speed.sin();

        self.memorize(Action::FORWARD)
    }

    fn backward(&mut self) {
        let speed: f32 = self.speed() as f32;

        self.pose.x -= speed.cos();
        self.pose.y -= speed.sin();

        self.memorize(Action::BACKWARD)
    }

    fn turn_left(&mut self) {
        self.pose.d -= (self.speed() & (2 * PI as u8)) as f32;
        while self.pose.d < 0.0 { self.pose.d += 2.0 * PI }

        self.memorize(Action::TURN_LEFT)
    }

    fn turn_right(&mut self) {
        self.pose.d += (self.speed() & (2 * PI as u8)) as f32;
        while self.pose.d > 2.0 * PI { self.pose.d -= 2.0 * PI }

        self.memorize(Action::TURN_LEFT)
    }

    fn change_color(&mut self) {
        self.color.h = self.color.h & self.speed();
        self.color.s = self.color.v & self.speed();
        self.color.v = self.color.s & self.speed();
    }
}
