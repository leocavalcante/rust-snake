extern crate ggez;
extern crate rand;

use std::time::{Duration, Instant};

use ggez::graphics;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use rand::Rng;

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
    color: graphics::Color,
    body: Vec<Point>,
    vel: Point,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            color: graphics::Color::from_rgb(0, 255, 0),
            vel: Point { x: 0, y: 0 },
            body: vec![Point::new()],
        }
    }

    fn update(&mut self) {
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
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) {
        for p in &self.body {
            let sqr = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, SCALE as f32, SCALE as f32),
                self.color,
            ).unwrap();

            graphics::draw(ctx, &sqr, (na::Point2::new(p.x as f32, p.y as f32), ));
        }
    }

    fn eats(&self, food: &Food) -> bool {
        self.head().overlap(&food.pos)
    }

    fn hits_it_self(&self) -> bool {
        for p in &self.body {
            if self.head().overlap(&p) {
                return true;
            }
        }

        false
    }

    fn of_bounds(&self) -> bool {
        self.head().x < 0
            || self.head().y < 0
            || self.head().x > (SIZE * SCALE) as i32
            || self.head().y > (SIZE * SCALE) as i32
    }

    fn reset(&mut self) {
        self.body = vec![Point::new()];
        self.vel = Point::new();
    }

    fn level_up(&mut self) {
        self.body.push(Point {
            x: self.head().x,
            y: self.head().y,
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

    fn head(&self) -> &Point {
        &self.body[0]
    }
}

struct Food {
    color: graphics::Color,
    pos: Point,
}

impl Food {
    fn new() -> Food {
        Food {
            color: graphics::Color::from_rgb(255, 0, 0),
            pos: Point::rand(),
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) {
        let sqr = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, SCALE as f32, SCALE as f32),
            self.color,
        ).unwrap();

        graphics::draw(ctx, &sqr, (na::Point2::new(self.pos.x as f32, self.pos.y as f32), ));
    }

    fn replace(&mut self) {
        self.pos = Point::rand();
    }
}

struct Game {
    food: Food,
    snake: Snake,
    last_update: Instant,
}

impl Game {
    fn new() -> ggez::GameResult<Game> {
        let g = Game {
            food: Food::new(),
            snake: Snake::new(),
            last_update: Instant::now(),
        };

        Ok(g)
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if !(Instant::now() - self.last_update >= Duration::from_millis((1.0 / (SCALE as f32) * 1000.0) as u64)) {
            return Ok(());
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.snake.move_right();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.snake.move_down();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.snake.move_left();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.snake.move_up();
        }

        if self.snake.eats(&self.food) {
            self.food.replace();
            self.snake.level_up();
        }

        self.snake.update();

        if self.snake.hits_it_self() {
            self.food.replace();
            self.snake.reset();
        }

        if self.snake.of_bounds() {
            self.food.replace();
            self.snake.reset();
        }

        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        self.food.draw(ctx);
        self.snake.draw(ctx);

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("snake", "leocavalcante")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions((SIZE * SCALE) as f32, (SIZE * SCALE) as f32));

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut Game::new()?;

    ggez::event::run(ctx, event_loop, state)
}
