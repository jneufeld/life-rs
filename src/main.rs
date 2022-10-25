mod life;

use crate::life::Life;

fn main() {
    let seed = glider(50, 10);

    let mut life = Life::new(100, 20, seed);

    life.play();

    println!("Life ended after {} generations", life.steps);
}

fn glider(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x + 1, y + 2),
        (x + 2, y + 3),
        (x + 3, y + 1),
        (x + 3, y + 2),
        (x + 3, y + 3),
    ]
}
