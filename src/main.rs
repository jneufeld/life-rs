mod life;
mod seed;

use crate::{life::Grid, seed::Seed};

fn main() {
    let seeds = vec![
        Seed::glider_one(1, 1),
        Seed::glider_one(10, 10),
        Seed::glider_two(20, 10),
        Seed::glider_two(30, 5),
    ];

    let mut life = Grid::new(50, 20, seeds);
    life.play();

    println!("Life ended after {} generations", life.steps());
}
