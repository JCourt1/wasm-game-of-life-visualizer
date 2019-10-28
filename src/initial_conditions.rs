#[path = "utils.rs"]
mod utils;

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

pub fn get_initial_conditions_map_func(option : &str, width: u32, height: u32) -> Box<Fn(usize) -> bool> {
    let closure_func : Box<Fn(usize) -> bool> = match option {

        "test_space_ship" => {

            let pattern = &[(1,2), (2,3), (3,1), (3,2), (3,3)];
            let indices: Vec<usize> = pattern.iter().map(
                |(row, col)| {
                    utils::get_index(width, *row + height / 2, *col + width / 2)
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
                    utils::get_index(width, *row + height / 2, *col + width / 2)
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