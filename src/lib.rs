// lib.rs

// External crate dependencies, if any
// For example, if you're using any external libraries across multiple modules.

// Module declarations
mod cpi_data;
mod calculator;
mod utils;

// Re-exporting necessary functions or structs
// This allows other parts of your program (like CLI and GUI) to access these functions/structs directly through the library crate
pub use cpi_data::fetch_cpi_data;
pub use calculator::calculate_inflation;

// Any shared structs, enums, or constants that are used across multiple modules
// For example, error types or common data structures
