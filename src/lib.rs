mod utils;

extern crate js_sys;
use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//#[wasm_bindgen]
//extern {
//    fn alert(s: &str);
//}
//
//#[wasm_bindgen]
//pub fn greet(name: &str) {
//    alert(&format!("Hello, {}!", name));
//}


// utils, but still tightly linked to this module so not in utils.rs
pub fn get_index(width: u32, row: u32, col: u32) -> usize {
    (row * width + col) as usize
}

pub fn get_initial_conditions_map_func(
    option : &str,
    width: u32,
    height: u32
) -> Box<Fn(u32) -> Cell> {
    let closure_func : Box<Fn(u32) -> Cell> = match option {
        "copper_head_spaceship" => {
            if !CopperHead::grid_sufficient_size((width, height)) {
                panic!("grid not sufficient size");
            }

            let center = (width / 2, height / 2);
            let dims = CopperHead::min_dimensions();


            let pattern = CopperHead::get_pattern();

            let indices: Vec<usize> = pattern.iter().map(
                |(col, row)| {
                    get_index(width, *row + height / 2, *col + width / 2)
                }
            ).collect();

            Box::new(move |i| {
                let temp = &(i as usize);
                if indices.contains(temp) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
        },

        "random" => {
            Box::new(|i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
        },

        _ => {
            Box::new(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
        }
    };

    closure_func
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width : u32,
    height : u32,
    cells: Vec<Cell>,
}

trait Pattern {
    fn get_pattern() -> Vec<(u32, u32)>;
    fn min_dimensions() -> (u32, u32);
    fn grid_sufficient_size(grid_dimensions: (u32, u32)) -> bool;
}

pub struct CopperHead {}

impl Pattern for CopperHead {
    fn get_pattern() -> Vec<(u32, u32)> {
        vec![
            (2,1), (3,1), (6,1), (7,1), (4,2), (5,2), (4,3), (5,3), (1,4), (3,4), (6,4), (8,4),
            (1,5), (8,5), (1,7), (8,7), (2,8), (3,8), (6,8), (7,8), (3,9), (4,9), (5,9), (6,9),
            (4,11), (5,11), (4,12), (5,12)
        ]
    }

    fn min_dimensions() -> (u32, u32) {
        (10, 14)
    }

    fn grid_sufficient_size(grid_dimensions: (u32, u32)) -> bool {
        let min_dims = Self::min_dimensions();
        if grid_dimensions.0 < min_dims.0 || grid_dimensions.0 < min_dims.0 {
            return false
        }
        true

    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(option : Option<String>) -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let opt = option.as_ref().map_or("", String::as_str);

        let closure_func = get_initial_conditions_map_func(opt, width, height);
        let cells = (0..width * height).map(closure_func).collect();

        Universe {
            width,
            height,
            cells
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        get_index(self.width, row, col)
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // seems a bit weird but is actually this way just to avoid doing special case checks
        // for the edges of the universe. Modulo will ensure if we are at 0 we wrap to the end row or col
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;

                // this is where count is incremented according to the value of neighbouring cells
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        // start boilerplate matrix iteration
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                // end boilerplate matrix iteration
                let live_neighbours = self.live_neighbour_count(row, col);
                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    // in all other cases, the cell just stays the same
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

