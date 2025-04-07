use ggez::graphics::Canvas;
use ggez::*;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{ContextBuilder, GameResult};
use ggez::mint::{Point2, Vector2};
use ggez::input::keyboard::KeyCode;

struct Player {
    position: Point2<f32>,
    health: u32,
}

struct Enemy {
    position: Point2<f32>,
    health: u32,
}

struct State {
    player: Player,
    enemies: Vec<Enemy>,
    window_size: Point2<f32>,
    world_boundaries: Point2<f32>,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ctx.keyboard.is_key_pressed(KeyCode::Z) {
            // Move up
            println!("Moving up");
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            // Move down
            println!("Moving down");
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Q) {
            // Move left
            println!("Moving left");
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            // Move right
            println!("Moving right");
        }
        Ok(())
    }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, graphics::Color::from_rgb(0, 0, 64));
    let player = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        self.player.position,
        10.0,
        0.1,
        graphics::Color::from_rgb(255, 0, 0),
    )?;
    let enemies: Vec<_> = self.enemies.iter().map(|enemy| {
        graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            enemy.position,
            10.0,
            0.1,
            graphics::Color::from_rgb(0, 255, 0),
        )
    }).collect::<Result<Vec<_>, _>>()?;
    let world_boundaries = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(1.0),
        //since the width is 1 pixel, we need to start at 1.0,1.0
        graphics::Rect::new(1.0,1.0, self.world_boundaries.x, self.world_boundaries.y),
        graphics::Color::from_rgb(255, 255, 255),
    )?;
    canvas.draw(&player, graphics::DrawParam::default());
    for enemy in enemies {
        canvas.draw(&enemy, graphics::DrawParam::default());
    }
    canvas.draw(&world_boundaries, graphics::DrawParam::default());
    canvas.finish(ctx)?;
    Ok(())
  }
}

pub fn main() {
    let player = Player { position : Point2{x:0.0,y:0.0}, health : 100 };
    let enemies = Vec::new();
    let window_size = Point2{x:1024.0,y:768.0};
    // Set world boundaries to be slightly smaller than the window size to avoid drawing outside the window
    let world_boundaries = Point2{x:window_size.x-2.0, y:window_size.y-2.0};
    
    let state = State {
        player,
        enemies,
        window_size,
        world_boundaries
    };

    let c = conf::Conf::new();
    let cb = ContextBuilder::new("my_game", "author")
        .window_setup(WindowSetup::default().title("My Game"))
        .window_mode(WindowMode::default().dimensions(1024.0, 768.0)); // Set width and height here
    let (ctx, event_loop) = cb.build().unwrap();

    event::run(ctx, event_loop, state);
}