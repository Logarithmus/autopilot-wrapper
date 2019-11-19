mod res;

use crate::res::RES_PATH;
use autopilot::bitmap::*;
use autopilot::geometry::*;
use autopilot::key::{Character, Flag};
use autopilot::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::thread::sleep;
use std::time::Duration;

#[macro_use(lazy_static)]
extern crate lazy_static;

pub fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

fn stupid_screenshot() -> autopilot::bitmap::Bitmap {
    let screen = autopilot::bitmap::capture_screen().expect("Failed to capture screen");
    screen
        .image
        .save(res::RES_PATH.join("screenshot.png"))
        .unwrap();
    gray_image!("screenshot.png")
}

pub fn paste_string(ctx: &mut ClipboardContext, s: &str) {
    ctx.set_contents(s.to_string()).unwrap();
    tap_with_mod(&Character('v'), &[Flag::Control], 100);
}

pub fn get_color(x: i32, y: i32) -> Option<[u8; 4]> {
    let point = Point::new(x as f64, y as f64).scaled(1.0 / screen::scale());
    match autopilot::screen::get_color(point) {
        Ok(color) => Some(color.0), // color is a tuple struct with one member
        Err(_) => None,
    }
}

pub fn shifted_point(point: &Point, (dx, dy): (i32, i32)) -> Point {
    Point::new(point.x + dx as f64, point.y + dy as f64)
}

pub fn move_to(x: i32, y: i32) {
    let point = Point::new(x as f64, y as f64).scaled(1.0 / screen::scale());
    mouse::move_to(point).unwrap();
}

pub fn tap_with_mod<T: key::KeyCodeConvertible + Copy>(
    key: &T,
    flags: &[key::Flag],
    delay_ms: u64,
) {
    key::toggle(key, true, flags, delay_ms);
    sleep_ms(50);
    key::toggle(key, false, flags, delay_ms);
}

pub fn click(x: i32, y: i32) {
    move_to(x, y);
    mouse::click(mouse::Button::Left, None);
}

pub fn double_click(x: i32, y: i32) {
    move_to(x, y);
    mouse::click(mouse::Button::Left, None);
    sleep_ms(100);
    mouse::click(mouse::Button::Left, None);
}

pub fn wait_to_appear(
    needle: &Bitmap,
    rect: Option<Rect>,
    tries: u32,
    tolerance: f64,
) -> Option<Point> {
    let mut tries = tries;
    let mut opt = None;
    while (tries > 0) && (opt.is_none()) {
        sleep_ms(1000);
        opt = stupid_screenshot().find_bitmap(needle, Some(tolerance), rect, None);
        tries -= 1;
    }
    opt
}

pub fn wait_to_disappear(
    needle: &Bitmap,
    rect: Option<Rect>,
    tries: u32,
    tolerance: f64,
) -> Result<(), Point> {
    let mut tries = tries;
    let mut opt = Some(Point::new(0., 0.));
    while (tries > 0) && (opt.is_some()) {
        sleep_ms(1000);
        opt = stupid_screenshot().find_bitmap(needle, Some(tolerance), rect, None);
        tries -= 1;
    }
    match opt {
        Some(point) => Err(point),
        None => Ok(()),
    }
}
