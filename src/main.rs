use ggez::*;
use ggez::mint::{Point2, Vector2};

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
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    Ok(())
  }
}

pub fn main() {
    let main_state = State {
        dt: std::time::Duration::new(0, 0),
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

        event::run(ctx, event_loop, state);
}