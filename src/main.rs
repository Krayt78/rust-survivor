use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::mint::{Point2, Vector2};
use ggez::*;
use ggez::{ContextBuilder, GameResult};

struct Player {
    position: Point2<f32>,
    health: u32,
    movement_speed: f32,
}

struct Enemy {
    position: Point2<f32>,
    health: u32,
    movement_speed: f32,
    damage: u32,
    cooldown: f32,
}

struct State {
    player: Player,
    enemies: Vec<Enemy>,
    window_size: Point2<f32>,
    world_boundaries: Point2<f32>,
}

impl State {
    fn handle_player_input(&mut self, ctx: &mut Context) {
        // Get the time delta once to avoid multiple calls.
        let delta = ctx.time.delta().as_secs_f32();

        if ctx.keyboard.is_key_pressed(KeyCode::Z) {
            // Move player up
            self.player.position.y -= self.player.movement_speed * delta;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            // Move player down
            self.player.position.y += self.player.movement_speed * delta;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Q) {
            // Move player left
            self.player.position.x -= self.player.movement_speed * delta;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            // Move player right
            self.player.position.x += self.player.movement_speed * delta;
        }
    }
    fn handle_ai_movement(&mut self, ctx: &mut Context) {
        // Get the time delta once to avoid multiple calls.
        let delta = ctx.time.delta().as_secs_f32();

        for enemy in &mut self.enemies {
            if enemy.position.x < self.player.position.x {
                enemy.position.x += enemy.movement_speed * delta;
            } else {
                enemy.position.x -= enemy.movement_speed * delta;
            }

            if enemy.position.y < self.player.position.y {
                enemy.position.y += enemy.movement_speed * delta;
            } else {
                enemy.position.y -= enemy.movement_speed * delta;
            }
        }

        // Check for collisions with the player
        for enemy in &mut self.enemies {
            //Here i first reduce the cooldown of the enemy and then check if the enemy is in range of the player
            // because if he is still on cooldown he should not attack

            if enemy.cooldown <= 0.0 {
                if (enemy.position.x - self.player.position.x).abs() < 10.0
                    && (enemy.position.y - self.player.position.y).abs() < 10.0
                {
                    // Enemy attacks player
                    self.player.health = self.player.health.saturating_sub(enemy.damage);
                    enemy.cooldown = 1.0; // Reset cooldown //TODO: Make this a variable and not a magic number
                }
            } else if enemy.cooldown > 0.0 {
                enemy.cooldown -= delta; // Decrease cooldown
            }
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.handle_player_input(ctx);
        self.handle_ai_movement(ctx);
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
        let enemies: Vec<_> = self
            .enemies
            .iter()
            .map(|enemy| {
                graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    enemy.position,
                    10.0,
                    0.1,
                    graphics::Color::from_rgb(0, 255, 0),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let world_boundaries = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            //since the width is 1 pixel, we need to start at 1.0,1.0
            graphics::Rect::new(1.0, 1.0, self.world_boundaries.x, self.world_boundaries.y),
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
    let window_size = Point2 {
        x: 1024.0,
        y: 768.0,
    };
    // Set world boundaries to be slightly smaller than the window size to avoid drawing outside the window
    let world_boundaries = Point2 {
        x: window_size.x - 2.0,
        y: window_size.y - 2.0,
    };

    let player = Player {
        position: Point2 {
            x: world_boundaries.x / 2.0,
            y: world_boundaries.y / 2.0,
        },
        health: 100,
        movement_speed: 100.0,
    };
    let mut enemies = Vec::new();

    // Create a few enemies with random positions
    for _i in 0..5 {
        let enemy = Enemy {
            position: Point2 {
                x: rand::random::<f32>() * world_boundaries.x,
                y: rand::random::<f32>() * world_boundaries.y,
            },
            health: 100,
            movement_speed: 50.0,
            damage: 5,
            cooldown: 0.0,
        };
        enemies.push(enemy);
    }

    let state = State {
        player,
        enemies,
        window_size,
        world_boundaries,
    };

    let cb = ContextBuilder::new("my_game", "author")
        .window_setup(WindowSetup::default().title("My Game"))
        .window_mode(WindowMode::default().dimensions(1024.0, 768.0)); // Set width and height here
    let (ctx, event_loop) = cb.build().unwrap();

    event::run(ctx, event_loop, state);
}
