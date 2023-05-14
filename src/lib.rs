#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "gui")]
pub mod app;

pub mod graph;

pub mod prelude {
    use super::*;

    pub use graph::generate_graph;

    #[cfg(feature = "gui")]
    pub use graph::show_graph;
}
