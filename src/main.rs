#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use galmetry::geometry::point::Point;

fn main() {
    let p = Point::from2d(2.0, 2.0);
    println!("{}", p);
}
