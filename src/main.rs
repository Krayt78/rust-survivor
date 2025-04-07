use ggez::*;
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
    Ok(())
  }
}

pub fn main() {
    let player = Player { position : Point2{x:0.0,y:0.0}, health : 100 };
    let enemies = Vec::new();
    let world_boundaries = Point2{x:100.0,y:100.0};
    
    let state = State {
        player,
        enemies,
        world_boundaries
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}