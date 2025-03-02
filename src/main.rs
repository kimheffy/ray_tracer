mod canvas;
mod color;
mod matrix;
mod tuple;
mod utils;

use std::{fmt::format, fs, io::Write};

use canvas::Canvas;
use color::Color;
use tuple::Tuple;

type Vector = Tuple;
type Point = Tuple;

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn from(pos: Tuple, vel: Tuple) -> Self {
        let position = Tuple::from(pos.x, pos.y, pos.z, pos.w);
        let velocity = Tuple::from(vel.x, vel.y, vel.z, vel.w);

        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn from(grav: Vector, wind: Vector) -> Self {
        Self {
            gravity: grav,
            wind,
        }
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile::from(position, velocity)
}

fn main() {
    let f: &str = "0.0";
    f.parse::<f64>().unwrap();

    let start = Tuple::to_point(0.0, 1.0, 0.0);
    let velocity = Tuple::to_vector(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut p = Projectile::from(start, velocity);

    let e = Environment::from(
        Tuple::to_vector(0.0, -0.1, 0.0),
        Tuple::to_vector(-0.01, 0.0, 0.0),
    );

    let canvas_width = 900;
    let canvas_height = 500;
    let mut canvas = Canvas::new(canvas_width, canvas_height);

    println!("starting projectile? ({}, {}) ", p.position.x, p.position.y);

    while (p.position.x as usize) < canvas_width {
        let p_x = p.position.x as usize;
        let p_y = canvas_height - p.position.y as usize;

        if p_y < canvas_height {
            println!("pos: ({}, {})", p_x, p_y);

            canvas.write_pixel(p_x, p_y, Color::new(1.0, 0.0, 0.0));

            p = tick(&e, &p);
        } else {
            break;
        }
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("sim.ppm")
        .unwrap();

    file.write_fmt(format_args!("{}", canvas.to_ppm())).unwrap();
}
