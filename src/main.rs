//! The simplest possible example that does something.
extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point2};
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

        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, self.pos_y),
                         100.0,
                         2.0)?;
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
