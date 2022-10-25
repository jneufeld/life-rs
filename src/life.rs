use std::{
    cell::Cell,
    fmt::{self},
    time::Duration,
};

/// The board on which to play the game of life
pub struct Life {
    width: usize,
    height: usize,
    pub steps: u128,
    cells: Vec<Vec<Cell<bool>>>,
}

impl Life {
    /// Create a new game of life
    pub fn new(width: usize, height: usize, seed: Vec<(usize, usize)>) -> Life {
        if width == 0 {
            panic!("Width must be greater than zero (received {})", width);
        }

        if height == 0 {
            panic!("Height must be greater than zero (received {})", height);
        }

        let mut cells = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();

            for _ in 0..width {
                row.push(Cell::new(false));
            }

            cells.push(row);
        }

        for (x, y) in seed {
            if x > width - 1 {
                panic!("Seed cell is outside board. x={} but width={}.", x, width);
            }
            if y > height - 1 {
                panic!("Seed cell is outside board. y={} but width={}.", y, height);
            }

            cells[y][x].set(true);
        }

        Life {
            width,
            height,
            steps: 0,
            cells,
        }
    }

    /// Play the game of life
    pub fn play(&mut self) {
        println!("{}", self);

        while self.can_continue() {
            self.tick();
            self.steps += 1;

            println!("{}", self);

            let pause_time = Duration::from_secs(2);

            std::thread::sleep(pause_time);
        }
    }

    /// Continue if there are any living cells
    fn can_continue(&self) -> bool {
        self.cells.iter().flatten().any(|cell| cell.get())
    }

    /// A tick is an iteration of the game. Every iteration, update all cells according to the
    /// rules of the game.
    fn tick(&mut self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let is_alive = cell.get();
                let neighbours = self.count_neighbours(x, y);

                // Kill living cells with too many or too few neighbours
                if is_alive && (neighbours > 3 || neighbours < 2) {
                    cell.set(false);
                }

                // Dead cells with the right neighbours come to life
                if !is_alive && neighbours == 3 {
                    cell.set(true);
                }
            }
        }
    }

    /// Counts the number of living cells in the Moore neighbourhood
    fn count_neighbours(&self, cell_x: usize, cell_y: usize) -> u8 {
        // The implementation is fussy over types and edge cases. The code is deliberately verbose
        // to help make sense of the details.
        let max_width = self.width - 1;
        let max_height = self.height - 1;

        // Enumerate the row and column indices of potential neighbours. Some spill over the edge of
        // the board. Using `checked_` methods on `usize` returns an `Option` where `Some(_)` returns
        // an actual value while `None` indicates an overflow (i.e., the index is off the board).
        let x_indices = [cell_x.checked_sub(1), cell_x.checked_add(1), Some(cell_x)];
        let y_indices = [cell_y.checked_sub(1), cell_y.checked_add(1), Some(cell_y)];

        let mut result = 0;

        for y in y_indices {
            // Skip indices of the `None` variant since these indicate an index off the board (i.e., it is)
            // it is too small)
            if y.is_none() {
                continue;
            }

            // Since the variant is not `None` it is safe unwrap the value
            let y = y.unwrap();

            // Ensure the value is on the board in the other direction (i.e., it isn't too big)
            if y > max_height {
                continue;
            }

            // The logic for each column index is the same as the code above. If all conditions pass, the
            // tuple of indices indexes a cell on the board.
            for x in x_indices {
                if x.is_none() {
                    continue;
                }

                let x = x.unwrap();

                if x > max_width {
                    continue;
                }

                // Lastly, skip if this is `(i, j)` since that is the cell for which we are counting the
                // neighbours
                if x == cell_x && y == cell_y {
                    continue;
                }

                // If the cell is dead then continue
                if !self.cells[y][x].get() {
                    continue;
                }

                // Otherwise count the neighbour as live
                result += 1;
            }
        }

        result
    }
}

impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cell_displays = String::new();

        for row in &self.cells {
            for cell in row {
                let value = if cell.get() { "x" } else { "." };
                cell_displays.push_str(value);
            }

            cell_displays.push('\n');
        }

        writeln!(f, "{}\nStep: {}", cell_displays, self.steps)
    }
}
