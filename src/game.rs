use comfy::*;
use rand::Rng;

pub const SCREEN_WIDTH: u32 = 600;
pub const SCREEN_HEIGHT: u32 = 600;

pub const WORLD_WIDTH: f32 = 30.0;
pub const WORLD_HEIGHT: f32 = 30.0;
pub const HALF_WORLD_WIDTH: f32 = WORLD_WIDTH / 2.0;
pub const HALF_WORLD_HEIGHT: f32 = WORLD_HEIGHT / 2.0;

pub const SCALE: f32 = 2.0;
pub const OFFSET: f32 = SCALE / 2.0;

fn constrain(value: f32, min: f32, max: f32) -> f32 {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    return value;
}

fn apply_offset(value: f32) -> f32 {
    if value % SCALE != 0.0 {
        if value > 0.0 {
            return value - OFFSET;
        } else if value < 0.0 {
            return value + OFFSET;
        } else {
            return value - OFFSET;
        }
    } else {
        return value;
    }
}

pub struct Game {
    pub _snake: Snake,
    pub _fruit: Fruit,
}

impl Game {
    pub fn create() -> Self {
        Self {
            _snake: Snake::create(),
            _fruit: Fruit::create(),
        }
    }
}

pub struct Snake {
    pub px: f32,
    pub py: f32,
    sx: f32,
    sy: f32,
    z: i32,
    dimension: f32,
    color: Color,
    pub size: usize,
    pub tail: Vec<(f32, f32)>,
}

impl Snake {
    pub fn create() -> Self {
        Self {
            px: SCALE,
            py: SCALE,
            sx: 0.0,
            sy: 1.0,
            z: 1,
            dimension: SCALE,
            color: GREEN,
            size: 0,
            tail: Vec::new(),
        }
    }

    pub fn set_speed(&mut self, _sx: f32, _sy: f32) {
        self.sx = _sx;
        self.sy = _sy;
    }

    pub fn update(&mut self) {
        if self.tail.len() == self.size && self.tail.len() > 0 {
            for i in 0..self.tail.len()-1 {
                self.tail[i] = self.tail[i + 1];
            }
        }
        if self.size != self.tail.len() {
            self.tail[self.size-1] = (self.px, self.py);
        }
        /*
        // check if new segment will be added
        if self.size != self.tail.len() {
            self.tail.push((self.px, self.py));
        }
        */
        self.px = self.px + self.sx * SCALE;
        self.py = self.py + self.sy * SCALE;

        self.px = constrain(
            self.px,
            -HALF_WORLD_WIDTH + OFFSET,
            HALF_WORLD_WIDTH - OFFSET,
        );
        self.py = constrain(
            self.py,
            -HALF_WORLD_HEIGHT + OFFSET,
            HALF_WORLD_HEIGHT - OFFSET,
        );  
    }

    pub fn check_eat(&mut self, fx: f32, fy: f32) -> bool {
        if self.px == fx && self.py == fy {
            self.size += 1;
            return true;
        }
        return false;
    }

    pub fn draw(&self) {
        for i in 0..self.tail.len() {
            draw_rect(
                vec2(self.tail[i].0, self.tail[i].1),
                splat(self.dimension),
                self.color,
                self.z,
            );
        }

        draw_rect(
            vec2(self.px, self.py),
            splat(self.dimension),
            self.color,
            self.z,
        );
     
    }
}

pub struct Fruit {
    pub px: f32,
    pub py: f32,
    z: i32,
    dimension: f32,
    color: Color,
}

impl Fruit {
    pub fn create() -> Self {
        let mut rng = rand::thread_rng();
        let mut x = f32::floor(rng.gen_range(-HALF_WORLD_WIDTH..=HALF_WORLD_WIDTH));
        let mut y = f32::floor(rng.gen_range(-HALF_WORLD_HEIGHT..=HALF_WORLD_HEIGHT));
        x = apply_offset(x); // correct x position if necessary according with SCALE
        y = apply_offset(y); // correct y position if necessary according with SCALE
        Self {
            px: x,
            py: y,
            z: 0,
            dimension: SCALE,
            color: RED,
        }
    }

    pub fn new_pos(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x = f32::floor(rng.gen_range(-HALF_WORLD_WIDTH..=HALF_WORLD_WIDTH));
        let mut y = f32::floor(rng.gen_range(-HALF_WORLD_HEIGHT..=HALF_WORLD_HEIGHT));
        x = apply_offset(x); // correct x position if necessary according with SCALE
        y = apply_offset(y); // correct y position if necessary according with SCALE
        self.px = x as f32;
        self.py = y as f32;
    }

    pub fn draw(&self) {
        draw_rect(
            vec2(self.px, self.py),
            splat(self.dimension),
            self.color,
            self.z,
        );
    }
}
