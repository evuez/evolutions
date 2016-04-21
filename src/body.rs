use uuid::Uuid;
use rand::{XorShiftRng, SeedableRng, Rng};
use rand::distributions::normal::Normal;
use rand::distributions::Sample;
use std::f32::consts::PI;


const MAX_SPEED: u8 = 100;


enum Action {
    FORWARD,
    BACKWARD,
    TURN_LEFT,
    TURN_RIGHT,
}

struct Choice {
    id: u8,
    weight: f64,
    action: Action,
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
    choices: [Choice; 4],
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
            rng: rng,
            choices: [
                Choice { action: Action::FORWARD, id: 1, weight: 1.0 },
                Choice { action: Action::BACKWARD, id: 2, weight: 1.0 },
                Choice { action: Action::TURN_LEFT, id: 3, weight: 1.0 },
                Choice { action: Action::TURN_RIGHT, id: 4, weight: 1.0 },
            ]
        }
    }

    fn memorize(&mut self, action: Action) {
        match self.memory {
            Some(ref mut m) => m.push(action),
            None => self.memory = Some(vec![action])
        };
    }

    fn pick(&mut self) -> Option<f64> {
        let choice = self.rng.choose(&self.choices);
        match choice {
            Some(c) => Some(Normal::new(c.id as f64, c.weight).sample(&mut self.rng)),
            None => None
        }
    }

    pub fn tick(&mut self) {
        let value = match self.pick() {
            Some(c) => c.round() as u8,
            None => return (),
        };

        match value {
            1 => self.forward(),
            2 => self.backward(),
            3 => self.turn_left(),
            4 => self.turn_right(),
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
