#![allow(unused)]

use crate::prelude::*;
use crate::controllers::AppController;

mod error;
mod prelude;
mod utils;
mod models;
mod views;
mod controllers;
mod gui;
mod search;
mod sort;
mod pathfinder;
mod tree_traversal;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppController::new();
    app.run().await
}