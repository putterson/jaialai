//! The simplest possible example that does something.
extern crate ggez;
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics3d;

use ggez::*;
use ggez::graphics::{DrawMode, Point2, Color};
use ggez::event::{Mod, Keycode};
use ggez::timer;

use na::{Vector3, Translation3, Real};
use ncollide::shape::{Ball, Plane};
use nphysics3d::world::World;
use nphysics3d::object::RigidBody;


struct MainState {
    pos_x: f32,
    pos_y:f32,
    physics: Physics,
}


struct Physics {
    world: World<f32>,
}

impl Physics {
    fn new() -> Physics {
        return Physics {
            world: Physics::init(),
        }
    }


    fn init() -> World<f32> {
        /*
        * World
        */
        let mut world = World::new();
        world.set_gravity(Vector3::new(0.0, -9.81, 0.0));

        /*
        * Planes
        */
        let normals = [
            Vector3::new(-1.0, 1.0, -1.0 ),
            Vector3::new(1.0, 1.0, -1.0 ),
            Vector3::new(-1.0, 1.0, 1.0 ),
            Vector3::new(1.0, 1.0, 1.0 )
        ];
        for n in normals.iter() {
            let rb   = RigidBody::new_static(Plane::new(*n), 0.3, 0.6);

            world.add_rigid_body(rb);
        }

        /*
        * Create the balls
        */
        let num     = 1500.0f32.powf(1.0f32 / 3.0) as usize;
        let rad     = 0.5;
        let shift   = 2.5 * rad;
        let centerx = shift * (num as f32) / 2.0;
        let centery = shift * (num as f32) / 2.0;

        for i in 0usize .. num {
            for j in 0usize .. num {
                for k in 0usize .. num {
                    let x = i as f32 * 2.5 * rad - centerx;
                    let y = 10.0 + j as f32 * 2.5 * rad + centery * 2.0;
                    let z = k as f32 * 2.5 * rad - centerx;

                    let mut rb = RigidBody::new_dynamic(Ball::new(rad), 1.0, 0.3, 0.6);

                    rb.append_translation(&Translation3::new(x, y, z));

                    world.add_rigid_body(rb);
                }
            }
        }

        return world;
    }
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y: 380.0, physics: Physics::new() };
        Ok(s)
    }

    fn draw_court(&mut self, ctx: &mut Context) -> GameResult<()> {
        //Bottom
        graphics::set_color(ctx, graphics::Color::new(0.1, 0.4, 0.2, 1.0))?;
        graphics::polygon(ctx, DrawMode::Fill, & vec![
            Point2::new(250.0,590.0),
            Point2::new(50.0,490.0),
            Point2::new(550.0,240.0),
            Point2::new(750.0,340.0),
        ])?;

        //Side
        graphics::set_color(ctx, graphics::Color::new(0.2, 0.3, 0.2, 1.0))?;
        graphics::polygon(ctx, DrawMode::Fill, & vec![
            Point2::new(50.0,490.0-COURT_HEIGHT),
            Point2::new(50.0,490.0),
            Point2::new(550.0,240.0),
            Point2::new(550.0,240.0-COURT_HEIGHT),
        ])?;

        //Back

        //baseboard
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::polygon(ctx, DrawMode::Fill, & vec![
            Point2::new(550.0,240.0-BASEBOARD_HEIGHT),
            Point2::new(550.0,240.0),
            Point2::new(750.0,340.0),
            Point2::new(750.0,340.0-BASEBOARD_HEIGHT),
        ])?;

        graphics::set_color(ctx, graphics::Color::new(0.3, 0.35, 0.3, 1.0))?;
        graphics::polygon(ctx, DrawMode::Fill, & vec![
            Point2::new(550.0,240.0-BASEBOARD_HEIGHT),
            Point2::new(550.0,240.0-COURT_HEIGHT),
            Point2::new(750.0,340.0-COURT_HEIGHT),
            Point2::new(750.0,340.0-BASEBOARD_HEIGHT),
        ])?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let duration = timer::get_delta(_ctx);
        let dt = duration.as_secs() as f32
           + duration.subsec_nanos() as f32 * 1e-9;
        self.physics.world.step(dt);
        self.pos_x = self.pos_x % 800.0 + 2.0;
        self.pos_y = self.pos_y % 600.0 + 1.0;
        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.draw_court(ctx)?;

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

const COURT_HEIGHT: f32 = 230.0;
const BASEBOARD_HEIGHT: f32 = 20.0;

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("jailai", "pl", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
