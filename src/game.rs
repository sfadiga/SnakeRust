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

fn check_out_of_bounds(x: f32, y: f32) -> bool {
    if f32::abs(x) > (HALF_WORLD_WIDTH - OFFSET) {
        return true;
    }
    if f32::abs(y) > (HALF_WORLD_HEIGHT - OFFSET) {
        return true;
    }
    return false;
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
        // these ifs avoid the snake to go in reverse
        if f32::abs(_sx) != f32::abs(self.sx) {
            self.sx = _sx;
        }
        if f32::abs(_sy) != f32::abs(self.sy) {
            self.sy = _sy;
        }
    }

    pub fn game_over(&self) -> bool {
        for el in self.tail.iter() {
            if el.0 == self.px && el.1 == self.py {
                return true;
            }
        }
        return check_out_of_bounds(self.px, self.py);
    }

    pub fn update(&mut self) {
        if self.tail.len() == self.size && self.tail.len() > 0 {
            for i in 0..self.tail.len() - 1 {
                self.tail[i] = self.tail[i + 1];
            }
            let last = self.tail.len() - 1;
            self.tail[last] = (self.px, self.py);
        }
        if self.size != self.tail.len() {
            self.tail.push((self.px, self.py));
        }

        self.px = self.px + self.sx * SCALE;
        self.py = self.py + self.sy * SCALE;
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
            Snake::draw_snake(
                self.tail[i].0,
                self.tail[i].1,
                self.dimension,
                self.color,
                self.z,
            );
        }
        Snake::draw_snake(self.px, self.py, self.dimension, self.color, self.z);
    }

    fn draw_snake(x: f32, y: f32, dim: f32, color: Color, z: i32) {
        draw_rect(vec2(x, y), splat(dim - 0.1), color, z + 1);
        draw_rect_outline(vec2(x, y), splat(dim), 0.5, BLACK, z);
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
            color: FUCHSIA,
        }
    }

    pub fn new_pos(&mut self) {
        //TODO check if new pos is inside the snake then, try other pos
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
