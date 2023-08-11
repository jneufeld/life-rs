use std::{
    cell::Cell,
    fmt::{self},
    time::Duration,
};

use crate::seed::{Point, Seed};

/// The grid of cells making up the game of life.
pub struct Grid {
    width: usize,
    height: usize,
    steps: u128,
    life: bool,
    cells: Vec<Vec<Cell<bool>>>,
}

impl Grid {
    /// Creates a grid or panics if arguments are invalid. Width and height must
    /// be greater than zero. Seed positions must be on the grid.
    pub fn new(width: usize, height: usize, seeds: Vec<Seed>) -> Grid {
        if width == 0 {
            panic!("Width must be greater than zero (received {width})");
        }

        if height == 0 {
            panic!("Height must be greater than zero (received {height})");
        }

        // Create empty grid
        let mut cells = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell::new(false));
            }
            cells.push(row);
        }

        // Add seeds or panic
        for seed in seeds {
            for Point { x, y } in seed.points() {
                if *x > width - 1 {
                    panic!("Seed cell is outside board. x={x} but width={width}.");
                }
                if *y > height - 1 {
                    panic!("Seed cell is outside board. y={y} but height={height}.");
                }
                cells[*y][*x].set(true);
            }
        }

        Grid {
            width,
            height,
            cells,
            steps: 0,
            life: true,
        }
    }

    /// Plays the game and prints each iteration on the screen.
    pub fn play(&mut self) {
        println!("{self}");

        while self.can_continue() {
            self.tick();
            self.steps += 1;

            println!("{self}");

            let pause_time = Duration::from_millis(100);

            std::thread::sleep(pause_time);
        }
    }

    pub fn steps(&self) -> u128 {
        self.steps
    }

    /// Returns true when there's evidence of life. When cells stop changing
    /// life can't evolve.
    fn can_continue(&self) -> bool {
        self.life
    }

    /// A tick is an iteration of the game. Every iteration, update all cells
    /// according to the rules of the game.
    fn tick(&mut self) {
        let mut changes = Vec::new();

        // Find cells that need to change state. Update them in a later loop so
        // we don't invalidate the current state.
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let is_alive = cell.get();
                let neighbours = self.count_neighbours(x, y);

                if is_alive && !(2..=3).contains(&neighbours) {
                    changes.push((x, y, false));
                } else if !is_alive && neighbours == 3 {
                    changes.push((x, y, true));
                }
            }
        }

        self.life = !changes.is_empty();

        for (x, y, status) in changes {
            self.cells[y][x].set(status);
        }
    }

    /// Counts the number of living cells in the Moore neighbourhood
    fn count_neighbours(&self, cell_x: usize, cell_y: usize) -> u8 {
        let mut result = 0;

        let max_width = self.width - 1;
        let max_height = self.height - 1;

        // Enumerate the row and column indices of potential neighbours. Some
        // spill over the edge of the board. Using `checked_` methods on `usize`
        // returns an `Option` where `Some(_)` returns an actual value while
        // `None` indicates an overflow (i.e., the index is off the board).
        let x_indices = [cell_x.checked_sub(1), cell_x.checked_add(1), Some(cell_x)];
        let y_indices = [cell_y.checked_sub(1), cell_y.checked_add(1), Some(cell_y)];

        for y in y_indices {
            for x in x_indices {
                if y.is_none() || x.is_none() {
                    continue;
                }

                let y = y.unwrap();
                let x = x.unwrap();

                if x > max_width || y > max_height {
                    continue;
                }

                if x == cell_x && y == cell_y {
                    continue;
                }

                if self.cells[y][x].get() {
                    result += 1;
                }
            }
        }

        result
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cell_displays = String::new();

        for row in &self.cells {
            for cell in row {
                let c = if cell.get() { 'x' } else { '.' };
                cell_displays.push(c);
            }

            cell_displays.push('\n');
        }

        writeln!(f, "{}\nStep: {}", cell_displays, self.steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_seed() {
        let seed = vec![];
        let grid = Grid::new(10, 10, seed);

        assert_eq!(0, grid.count_neighbours(0, 0));
    }

    #[test]
    fn glider_one() {
        let seed = Seed::glider_one(1, 1);
        let grid = Grid::new(10, 10, vec![seed]);

        assert_eq!(1, grid.count_neighbours(0, 0));
        assert_eq!(1, grid.count_neighbours(1, 0));
        assert_eq!(1, grid.count_neighbours(2, 0));
        assert_eq!(0, grid.count_neighbours(3, 0));

        assert_eq!(1, grid.count_neighbours(0, 1));
        assert_eq!(1, grid.count_neighbours(1, 1));
        assert_eq!(3, grid.count_neighbours(2, 1));
        assert_eq!(2, grid.count_neighbours(3, 1));

        assert_eq!(2, grid.count_neighbours(0, 2));
        assert_eq!(4, grid.count_neighbours(1, 2));
        assert_eq!(4, grid.count_neighbours(2, 2));
        assert_eq!(2, grid.count_neighbours(3, 2));

        assert_eq!(1, grid.count_neighbours(0, 3));
        assert_eq!(2, grid.count_neighbours(1, 3));
        assert_eq!(3, grid.count_neighbours(2, 3));
        assert_eq!(3, grid.count_neighbours(3, 3));
    }

    #[test]
    fn glider_two() {
        let seed = Seed::glider_two(1, 1);
        let grid = Grid::new(10, 10, vec![seed]);

        assert_eq!(0, grid.count_neighbours(0, 0));
        assert_eq!(0, grid.count_neighbours(1, 0));
        assert_eq!(1, grid.count_neighbours(2, 0));
        assert_eq!(1, grid.count_neighbours(3, 0));

        assert_eq!(1, grid.count_neighbours(0, 1));
        assert_eq!(1, grid.count_neighbours(1, 1));
        assert_eq!(3, grid.count_neighbours(2, 1));
        assert_eq!(1, grid.count_neighbours(3, 1));

        assert_eq!(1, grid.count_neighbours(0, 2));
        assert_eq!(1, grid.count_neighbours(1, 2));
        assert_eq!(5, grid.count_neighbours(2, 2));
        assert_eq!(3, grid.count_neighbours(3, 2));

        assert_eq!(1, grid.count_neighbours(0, 3));
        assert_eq!(2, grid.count_neighbours(1, 3));
        assert_eq!(3, grid.count_neighbours(2, 3));
        assert_eq!(2, grid.count_neighbours(3, 3));
    }
}
