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

const SCALE: u32 = 12;
const SIZE: u32 = 42;

#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

const MOVE_UP: Point = Point { x: 0, y: -1 };
const MOVE_RIGHT: Point = Point { x: 1, y: 0 };
const MOVE_DOWN: Point = Point { x: 0, y: 1 };
const MOVE_LEFT: Point = Point { x: -1, y: 0 };

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn overlap(&self, vec: &Point) -> bool {
        self.x == vec.x && self.y == vec.y && !::std::ptr::eq(self, vec)
    }

    fn rand() -> Point {
        let mut rng = rand::thread_rng();

        Point {
            x: (rng.gen_range(0, SIZE - SCALE) * SCALE) as i32,
            y: (rng.gen_range(0, SIZE - SCALE) * SCALE) as i32,
        }
    }
}

struct Snake {
    color: Color,
    body: Vec<Point>,
    vel: Point,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            color: Color::RGB(0, 255, 0),
            vel: Point { x: 0, y: 0 },
            body: vec![Point::new()],
        }
    }

    fn update(&mut self, canvas: &mut WindowCanvas) {
        for i in (0..self.body.len()).rev() {
            match i {
                0 => {
                    self.body[i].x += self.vel.x * SCALE as i32;
                    self.body[i].y += self.vel.y * SCALE as i32;
                }
                _ => {
                    self.body[i].x = self.body[i - 1].x;
                    self.body[i].y = self.body[i - 1].y;
                }
            }

            canvas.set_draw_color(self.color);
            canvas.fill_rect(Rect::new(self.body[i].x, self.body[i].y, SCALE, SCALE));
        }
    }

    fn eats(&self, food: &Food) -> bool {
        self.body[0].overlap(&food.pos)
    }

    fn hits_it_self(&self) -> bool {
        for p in &self.body {
            if self.body[0].overlap(&p) {
                return true;
            }
        }

        false
    }

    fn reset(&mut self) {
        self.body = vec![Point::new()];
        self.vel = Point::new();
    }

    fn level_up(&mut self) {
        self.body.push(Point {
            x: self.body[0].x,
            y: self.body[0].y,
        });
    }

    fn move_up(&mut self) {
        match self.vel {
            MOVE_DOWN if self.body.len() > 1 => {}
            _ => self.vel = MOVE_UP,
        }
    }

    fn move_right(&mut self) {
        match self.vel {
            MOVE_LEFT if self.body.len() > 1 => {}
            _ => self.vel = MOVE_RIGHT,
        }
    }

    fn move_down(&mut self) {
        match self.vel {
            MOVE_UP if self.body.len() > 1 => {}
            _ => self.vel = MOVE_DOWN,
        }
    }

    fn move_left(&mut self) {
        match self.vel {
            MOVE_RIGHT if self.body.len() > 1 => {}
            _ => self.vel = MOVE_LEFT,
        }
    }
}

struct Food {
    color: Color,
    pos: Point,
}

impl Food {
    fn new() -> Food {
        Food {
            color: Color::RGB(255, 0, 0),
            pos: Point::rand(),
        }
    }

    fn update(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, SCALE, SCALE));
    }

    fn replace(&mut self) {
        self.pos = Point::rand();
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

    let mut food = Food::new();
    let mut snake = Snake::new();

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
                } => snake.move_up(),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => snake.move_right(),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => snake.move_down(),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => snake.move_left(),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if snake.eats(&food) {
            food.replace();
            snake.level_up();
        }

        food.update(&mut canvas);
        snake.update(&mut canvas);

        if snake.hits_it_self() {
            food.replace();
            snake.reset();
        }

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / SCALE));
    }

    Ok(())
}
