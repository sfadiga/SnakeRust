use comfy::*;

pub const SCREEN_WIDTH: u32 = 600; 
pub const SCREEN_HEIGHT: u32 = 600;

pub const WORLD_WIDTH: u32 = 30; 
pub const WORLD_HEIGHT: u32 = 30;
pub const HALF_WORLD_WIDTH: f32 = WORLD_WIDTH as f32/2.0; 
pub const HALF_WORLD_HEIGHT: f32 = WORLD_HEIGHT as f32/2.0;

pub const SCALE: f32 = 1.0;

fn constrain(value:f32, min:f32, max:f32) -> f32 {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    return value;
}

pub struct Game {
    pub _snake : Snake,
    pub _fruit : Fruit,
 }

 impl Game {
    pub fn create() -> Self {
       Self {
          _snake: Snake::create(), 
          _fruit: Fruit::create(),
       }
    }
 }

pub struct Snake 
{
     px: f32,
     py: f32,
     sx: f32,
     sy: f32,
     z: i32, 
     size: f32,
     color: Color,
}

impl Snake {

    pub fn create() -> Self {
        Self {
            px: 0.0,
            py: 0.0,
            sx: 0.0,
            sy: 1.0,
            z: 0,
            size: 1.0,
            color: GREEN,
        }
    }

    pub fn set_speed(&mut self, _sx:f32, _sy:f32) {
        self.sx = _sx;
        self.sy = _sy;
    }

    pub fn update(&mut self) {
        self.px = self.px + self.sx * SCALE;
        self.py = self.py + self.sy * SCALE;

       self.px = constrain(self.px, -(HALF_WORLD_WIDTH - self.size), HALF_WORLD_WIDTH - self.size);
       self.py = constrain(self.py, -(HALF_WORLD_HEIGHT - self.size), HALF_WORLD_HEIGHT - self.size);
    }

    pub fn draw(&self) {
        draw_rect(vec2(self.px, self.py), splat(self.size), self.color, self.z);
        draw_text(format!("px:{}, py:{}", self.px, self.py).as_str(), vec2(12.0,12.0), WHITE, TextAlign::Center);
    }
}


pub struct Fruit {
    px: f32,
    py: f32,
    z: i32, 
    size: f32,
    color: Color,
}

impl Fruit {

    pub fn create() -> Self {
        let col : f32 = f32::floor(WORLD_WIDTH as f32/SCALE);
        let row: f32 = f32::floor(WORLD_HEIGHT as f32/SCALE);
        let x : f32 = f32::floor(random()*col);
        let y : f32 = f32::floor(random()*row);
        Self {
            px: x,
            py: y,
            z: 0,
            size: 1.0,
            color: RED,
        }
    }

    pub fn draw(&self) {
        draw_rect(vec2(self.px, self.py), splat(self.size), self.color, self.z);
        draw_text(format!("px:{}, py:{}", self.px, self.py).as_str(), vec2(12.0,12.0), WHITE, TextAlign::Center);
    }
}