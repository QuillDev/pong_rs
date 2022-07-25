extern crate core;

mod entity;

use tetra::{Context, ContextBuilder, Event, graphics, input, State, TetraError, window};
use tetra::graphics::{Color, DrawParams, Texture};
use tetra::input::Key;
use tetra::math::{Vec2, Vec3};
use crate::entity::entity::Entity;

// engine constants
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

struct GameState {
    player_one: Entity,
    player_two: Entity,
    ball: Entity,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        // player one
        let player_one_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player_one_position =
            Vec2::new(16.0, (WINDOW_HEIGHT - player_one_texture.height() as f32) / 2.0);

        // player two
        let player_two_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player_two_position =
            Vec2::new(WINDOW_WIDTH - 16.0 - player_two_texture.width() as f32,
                      (WINDOW_HEIGHT - player_two_texture.height() as f32) / 2.0,
            );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position =
            Vec2::new((WINDOW_WIDTH - ball_texture.width() as f32) / 2.0,
                      (WINDOW_HEIGHT - ball_texture.height() as f32) / 2.0,
            );

        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        return Ok(GameState {
            player_one: Entity::new(player_one_texture, player_one_position),
            player_two: Entity::new(player_two_texture, player_two_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity)
        });
    }
}

impl State for GameState {
    // game physics / logic processing done here
    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {

        // player one controls
        // move paddle up
        if input::is_key_down(ctx, Key::W) {
            self.player_one.position.y -= PADDLE_SPEED;
        }

        // move paddle down
        if input::is_key_down(ctx, Key::S) {
            self.player_one.position.y += PADDLE_SPEED;
        }

        // player two controls
        // move paddle up
        if input::is_key_down(ctx, Key::I) {
            self.player_two.position.y -= PADDLE_SPEED;
        }

        // move paddle down
        if input::is_key_down(ctx, Key::K) {
            self.player_two.position.y += PADDLE_SPEED;
        }

        self.ball.update();

        // do physics
        let player_one_bounds = self.player_one.bounds();
        let player_two_bounds = self.player_two.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player_one_bounds) {
            Some(&self.player_one)
        }
        else if ball_bounds.intersects(&player_two_bounds) {
            Some(&self.player_two)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
            let offset = (paddle.center().y - self.ball.center().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // detect winner
        if self.ball.position.x < 0.0 {
            window::quit(ctx);
            println!("Player 2 wins!");
        }

        if self.ball.position.x > WINDOW_WIDTH {
            window::quit(ctx);
            println!("Player 1 wins!");
        }
        Ok(())
    }

    // rendering done here
    fn draw(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.player_one.draw(ctx);
        self.player_two.draw(ctx);
        self.ball.draw(ctx);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
