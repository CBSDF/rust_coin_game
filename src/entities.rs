use macroquad::prelude::*;

pub trait CircleCollider {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_radius(&self) -> f32;
}
 


pub struct Coin {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub size: f32,
    pub speed: f32,
}

pub struct Bomb {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
    pub damage: i32,
}

impl Coin {
    pub fn respawn(&mut self) {
        self.y = rand::gen_range(-200.0, 0.0);
        self.x = rand::gen_range(self.radius, screen_width() - self.radius);
        self.speed = rand::gen_range(2.0, 6.0);
    }
}

impl Bomb {
    pub fn respawn(&mut self) {
        self.y = rand::gen_range(-300.0, 0.0);
        self.x = rand::gen_range(self.radius, screen_width() - self.radius);
        self.speed = rand::gen_range(3.0, 6.0);
        self.damage = rand::gen_range(20, 60);
    }
}

impl CircleCollider for Coin {
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl CircleCollider for Bomb {
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn get_radius(&self) -> f32 {
        self.radius
    }
}

pub fn check_collision(player: &Player, object: &impl CircleCollider) -> bool {
    let c_x = object.get_x();
    let c_y = object.get_y();
    let r = object.get_radius();

    let closest_x = c_x.clamp(player.x, player.x + player.size);
    let closest_y = c_y.clamp(player.y, player.y + player.size);

    let distance_x = c_x - closest_x;
    let distance_y = c_y - closest_y;
    let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

    distance_squared < (r * r)
}


