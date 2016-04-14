use uuid::Uuid;
use rand::{XorShiftRng, SeedableRng, Rng};
use std::f32::consts::PI;


const MAX_SPEED: u8 = 100;


struct Action {
    name: String,
}

struct Color {
    h: u8,
    s: u8,
    v: u8,
}

struct Pose {
    x: f32,
    y: f32,
    d: f32,
}

struct Body {
    name: Uuid,
    memory: Option<Vec<Action>>,
    pose: Pose,
    color: Color,
    rng: XorShiftRng,
}

impl Body {
    fn new() -> Body {
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

    fn memorize(&mut self, action_name: &str) {
        let action = Action { name: action_name.to_string() };

        match self.memory {
            Some(ref mut m) => m.push(action),
            None => self.memory = Some(vec![action])
        };
    }

    fn speed(&mut self) -> u8 {
       self.rng.gen::<u8>() & MAX_SPEED
    }

    fn forward(&mut self) {
        let speed: f32 = self.speed() as f32;

        self.pose.x += speed.cos();
        self.pose.y += speed.sin();

        self.memorize("forward")
    }

    fn backward(&mut self) {
        let speed: f32 = self.speed() as f32;

        self.pose.x -= speed.cos();
        self.pose.y -= speed.sin();
    }

    fn turn_left(&mut self) {
        self.pose.d -= (self.speed() & (2 * PI as u8)) as f32;
        while self.pose.d < 0.0 { self.pose.d += 2.0 * PI }
    }

    fn turn_right(&mut self) {
        self.pose.d += (self.speed() & (2 * PI as u8)) as f32;
        while self.pose.d > 2.0 * PI { self.pose.d -= 2.0 * PI }
    }

    fn change_color(&mut self) {
        self.color.h = self.color.h & self.speed();
        self.color.s = self.color.v & self.speed();
        self.color.v = self.color.s & self.speed();
    }
}
