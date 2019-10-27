mod utils;

extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
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

pub fn get_initial_conditions_map_func(option : &str, width: u32, height: u32) -> Box<Fn(usize) -> bool> {
    let closure_func : Box<Fn(usize) -> bool> = match option {

        "test_space_ship" => {

            let pattern = &[(1,2), (2,3), (3,1), (3,2), (3,3)];
            let indices: Vec<usize> = pattern.iter().map(
                |(row, col)| {
                    get_index(width, *row + height / 2, *col + width / 2)
                }
            ).collect();

            Box::new(move |i| {
                if indices.contains(&i) {
                    true
                } else {
                    false
                }
            })

        },

        "copper_head_spaceship" => {
            if !CopperHead::grid_sufficient_size((width, height)) {
                panic!("grid not sufficient size");
            }

            let pattern = CopperHead::get_pattern();
            let indices: Vec<usize> = pattern.iter().map(
                |(col, row)| {
                    get_index(width, *row + height / 2, *col + width / 2)
                }
            ).collect();

            Box::new(move |i| {
                if indices.contains(&i) {
                    true
                } else {
                    false
                }
            })
        },

        "random" => {
            Box::new(|_i| {
                if js_sys::Math::random() < 0.5 {
                    true
                } else {
                    false
                }
            })
        },

        _ => {
            Box::new(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    true
                } else {
                    false
                }
            })
        }
    };

    closure_func
}

fn apply_func_to_cells<F>(cells: &mut FixedBitSet, func: F) where F: Fn(usize) -> bool {
    for (i, is_alive) in (0..cells.len()).map(func).enumerate() {
        cells.set(i, is_alive);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}


#[wasm_bindgen]
pub struct Universe {
    width : u32,
    height : u32,
    cells: FixedBitSet,
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

// methods not exposed to Javascript
impl Universe {
    fn apply_func_to_cells<F>(&mut self, func: F) where F: Fn(usize) -> bool {
        apply_func_to_cells(&mut self.cells, func);
    }

    fn reset_cells_to_dead(&mut self) {
        self.apply_func_to_cells(|_i| false);
    }

    pub fn get_cells(&self) -> &[u32] {
        self.cells.as_slice()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        let mut next = self.cells.clone();

        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            next.set(idx, true);
        }

        self.cells = next;
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
}

// These methods are to be exposed to Javascript, so marked with wasm_bindgen
#[wasm_bindgen]
impl Universe {
    pub fn new(option : Option<String>) -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        let opt = option.as_ref().map_or("", String::as_str);

        let closure_func = get_initial_conditions_map_func(opt, width, height);
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        apply_func_to_cells(&mut cells, closure_func);

        Universe {
            width,
            height,
            cells
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_cells_to_dead();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_cells_to_dead();
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

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        get_index(self.width, row, col)
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

                next.set(idx, match (cell, live_neighbours) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    // in all other cases, the cell just stays the same
                    (otherwise, _) => otherwise,
                })


            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

