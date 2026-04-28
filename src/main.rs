use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::DrawParam;
use ggez::graphics::FillOptions;
use ggez::graphics::InstanceArray;
use ggez::graphics::Mesh;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use rand::make_rng;
use rand::prelude::*;

struct Predator {
    pos: Vec2,
    direction: f32,
}

impl Predator {
    fn new(pos: Vec2, direction: f32) -> Self {
        Predator {
            pos,
            direction: direction,
        }
    }

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, camera: &Camera) {

    }
}

struct Camera {
    pos: Vec2
}

impl Camera {
    fn new() -> Self {
        Camera {
            pos: Vec2::new(0.0, 0.0)
        }
    }

    fn offset_point(&self, point: Vec2) -> Vec2 {
        Vec2 {
            x: point.x - self.pos.x + (960.0 / 2.0),
            y: point.y - self.pos.y + (540.0 / 2.0)
        }
    }

}



struct MainState {
    dt: std::time::Duration,
    predators: Vec<Predator>,
    predators_instance_array: InstanceArray,
    camera: Camera,
    rng: StdRng
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {

        let mut predators = Vec::new();
        let mut rng: StdRng = make_rng();

        

        for _ in 0..17000 {
            predators.push(Predator::new(Vec2::new(rng.random_range(-500.0 .. 500.0), rng.random_range(-500.0 .. 500.0)), rng.random_range(0.0 .. 2.0 * 3.14159)));
        }

        Ok(MainState {
            dt: std::time::Duration::new(0, 0),
            predators: predators,
            predators_instance_array: InstanceArray::new(_ctx, None),
            camera: Camera::new(),
            rng
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        self.dt = ctx.time.delta();

        let delta: f32 = self.dt.as_secs_f32();

        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.camera.pos.x -= delta * 360.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.camera.pos.x += delta * 360.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.camera.pos.y -= delta * 360.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.camera.pos.y += delta * 360.0;
        }


        for predator in &mut self.predators {
            predator.pos.x += predator.direction.cos() * 145.0 * delta;
            predator.pos.y += predator.direction.sin() * 145.0 * delta;
            predator.direction += self.rng.random_range((-30.0 .. 30.0)) * delta;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
        let mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::Fill(FillOptions::default()),
            Vec2::new(0.0, 0.0),
            15.0,
            0.1,
            Color::GREEN,
        )
        .unwrap();
        self.predators_instance_array.clear();
        for predator in &mut self.predators {
            self.predators_instance_array.push(DrawParam::default().dest(self.camera.offset_point(predator.pos)));
        }
        canvas.draw_instanced_mesh(mesh, &self.predators_instance_array, DrawParam::default());
        

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("creature_fight", "Maxim Kozlov")
        .window_setup(WindowSetup::default().title("Creature Fight"))
        .window_mode(WindowMode::default().dimensions(960.0, 540.0))
        .build()?;

    let my_game = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, my_game)
}
