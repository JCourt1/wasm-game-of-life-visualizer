extern crate fixedbitset;
use fixedbitset::FixedBitSet;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub fn get_index(width: u32, row: u32, col: u32) -> usize {
    (row * width + col) as usize
}

pub fn apply_func_to_cells<F>(cells: &mut FixedBitSet, func: F) where F: Fn(usize) -> bool {
    for (i, is_alive) in (0..cells.len()).map(func).enumerate() {
        cells.set(i, is_alive);
    }
}