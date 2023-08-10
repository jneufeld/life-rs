mod life;

use crate::life::Life;

fn main() {
    let seed = glider_two(1, 1);
    let mut life = Life::new(50, 20, seed);

    life.play();

    println!("Life ended after {} generations", life.steps);
}

fn glider_one(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y),
        (x, y + 2),
        (x + 1, y + 1),
        (x + 1, y + 2),
        (x + 2, y + 1),
    ]
}

fn glider_two(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y + 1),
        (x + 1, y + 2),
        (x + 2, y),
        (x + 2, y + 1),
        (x + 2, y + 2),
    ]
}
