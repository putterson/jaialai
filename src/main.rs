//! The simplest possible example that does something.
extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point2, Color};
use ggez::event::{Mod, Keycode};

struct MainState {
    pos_x: f32,
    pos_y:f32
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y: 380.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        self.pos_x = self.pos_x % 800.0 + 2.0;
        self.pos_y = self.pos_y % 600.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::Color::new(0.1, 0.4, 0.2, 1.0))?;

        graphics::polygon(ctx, DrawMode::Fill, & vec![
            Point2::new(250.0,590.0),
            Point2::new(50.0,490.0),
            Point2::new(750.0,340.0),
        ])?;

        graphics::polygon(ctx, DrawMode::Fill, & vec![
            Point2::new(550.0,240.0),
            Point2::new(50.0,490.0),
            Point2::new(750.0,340.0),
        ])?;

        graphics::set_color(ctx, Color::new(1.0,1.0,1.0,0.8));

        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, self.pos_y),
                         3.0,
                         0.1)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self, 
        ctx: &mut Context, 
        keycode: Keycode, 
        _keymod: Mod, 
        _repeat: bool
    ) { 
        self.pos_y = self.pos_y - 10.0;
    }

    fn key_up_event(
        &mut self, 
        _ctx: &mut Context,
        _keycode: Keycode,
        _keymod: Mod,
        _repeat: bool) {
        self.pos_x = self.pos_x + 10.00;
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("jailai", "pl", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
