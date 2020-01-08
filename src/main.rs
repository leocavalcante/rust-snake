extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::thread::sleep;
use std::time::Duration;

const SCALE: u32 = 10;
const SIZE: u32 = 40;

struct Vec {
    x: i32,
    y: i32,
}

impl Vec {
    fn overlap(&self, vec: &Vec) -> bool {
        self.x == vec.x && self.y == vec.y
    }

    fn rand() -> Vec {
        let mut rng = rand::thread_rng();

        Vec {
            x: (rng.gen_range(0, SIZE - SCALE) * SCALE) as i32,
            y: (rng.gen_range(0, SIZE - SCALE) * SCALE) as i32,
        }
    }
}

struct Snake {
    color: Color,
    pos: Vec,
    vel: Vec,
}

impl Snake {
    fn update(&mut self, canvas: &mut WindowCanvas) {
        self.pos.x += self.vel.x * SCALE as i32;
        self.pos.y += self.vel.y * SCALE as i32;

        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, SCALE, SCALE));
    }

    fn eats(&self, food: &Food) -> bool {
        self.pos.overlap(&food.pos)
    }

    fn levelup(&self) {
        println!("Level up");
    }
}

struct Food {
    color: Color,
    pos: Vec,
}

impl Food {
    fn new() -> Food {
        Food {
            color: Color::RGB(255, 255, 0),
            pos: Vec::rand(),
        }
    }

    fn update(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, SCALE, SCALE));
    }

    fn replace(&mut self) {
        self.pos = Vec::rand();
    }
}

fn main() -> Result<(), String> {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video
        .window("Snake", SIZE * SCALE, SIZE * SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();

    let mut snake = Snake {
        color: Color::RGB(255, 0, 0),
        pos: Vec { x: 0, y: 0 },
        vel: Vec { x: 0, y: 1 },
    };

    let mut food = Food::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => snake.vel = Vec { x: 0, y: -1 },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => snake.vel = Vec { x: 1, y: 0 },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => snake.vel = Vec { x: 0, y: 1 },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => snake.vel = Vec { x: -1, y: 0 },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        food.update(&mut canvas);
        snake.update(&mut canvas);

        if snake.eats(&food) {
            food.replace();
            snake.levelup();
        }

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / SCALE));
    }

    Ok(())
}
