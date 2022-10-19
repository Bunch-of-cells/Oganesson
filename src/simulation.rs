use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub struct MainState {
    pos_x: f32,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 500.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0],
            100.0,
            0.1,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, ([self.pos_x, 380.0],))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
