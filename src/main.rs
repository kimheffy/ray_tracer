mod color;
mod tuple;
mod utils;

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
    let mut p = Projectile::from(
        Tuple::to_point(0.0, 1.0, 0.0),
        Tuple::to_vector(1.0, 1.0, 0.0).normalize(),
    );

    let e = Environment::from(
        Tuple::to_vector(0.0, -0.1, 0.0),
        Tuple::to_vector(-0.01, 0.0, 0.0),
    );

    println!("p position is {}", p.position.y);

    while p.position.y >= 0.0 {
        p = tick(&e, &p);

        println!("Projectile's position: {:?}", p);
    }
}
