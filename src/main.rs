use std::thread::sleep;
use std::time::Duration;

use crate::mouse::set_mouse_state;
use crate::screen_capture::ScreenCaptureState;

mod screen_capture;
mod mouse;

#[derive(PartialEq, Eq, Debug)]
pub struct Color(u8, u8, u8);

const MIN_X: i32 = 650;
const MAX_X: i32 = 1250;
const MIN_Y: i32 = 0;
const MAX_Y: i32 = 260;
const LIGHT_YELLOW: Color = Color(0xff, 0xff, 0xc0);

fn is_left_arrow(screen: &ScreenCaptureState, x: i32, y: i32) -> Result<bool, String> {
    Ok(
        screen.get_color(x + 4, y - 4)? == LIGHT_YELLOW
            && screen.get_color(x + 3, y - 3)? == LIGHT_YELLOW
            && screen.get_color(x + 2, y - 2)? == LIGHT_YELLOW
            && screen.get_color(x + 1, y - 1)? == LIGHT_YELLOW
            && screen.get_color(x, y)? == LIGHT_YELLOW
            && screen.get_color(x - 1, y)? == LIGHT_YELLOW
            && screen.get_color(x - 2, y)? == LIGHT_YELLOW
            && screen.get_color(x - 3, y)? == LIGHT_YELLOW
            && screen.get_color(x, y + 1)? == LIGHT_YELLOW
            && screen.get_color(x + 1, y + 2)? == LIGHT_YELLOW
            && screen.get_color(x + 2, y + 3)? == LIGHT_YELLOW
            && screen.get_color(x + 3, y + 4)? == LIGHT_YELLOW
            && screen.get_color(x + 4, y + 5)? == LIGHT_YELLOW
            && screen.get_color(x + 3, y)? != LIGHT_YELLOW
    )
}

fn is_right_arrow(screen: &ScreenCaptureState, x: i32, y: i32) -> Result<bool, String> {
    Ok(
        screen.get_color(x - 4, y - 4)? == LIGHT_YELLOW
            && screen.get_color(x - 3, y - 3)? == LIGHT_YELLOW
            && screen.get_color(x - 2, y - 2)? == LIGHT_YELLOW
            && screen.get_color(x - 1, y - 1)? == LIGHT_YELLOW
            && screen.get_color(x, y)? == LIGHT_YELLOW
            && screen.get_color(x + 1, y)? == LIGHT_YELLOW
            && screen.get_color(x + 2, y)? == LIGHT_YELLOW
            && screen.get_color(x + 3, y)? == LIGHT_YELLOW
            && screen.get_color(x, y + 1)? == LIGHT_YELLOW
            && screen.get_color(x - 1, y + 2)? == LIGHT_YELLOW
            && screen.get_color(x - 2, y + 3)? == LIGHT_YELLOW
            && screen.get_color(x - 3, y + 4)? == LIGHT_YELLOW
            && screen.get_color(x - 4, y + 5)? == LIGHT_YELLOW
            && screen.get_color(x - 3, y)? != LIGHT_YELLOW
    )
}

fn is_indicator(screen: &ScreenCaptureState, x: i32, y: i32) -> Result<bool, String> {
    for i in -9..=9 {
        if screen.get_color(x, y + i)? != LIGHT_YELLOW {
            return Ok(false);
        }
    }
    Ok(true)
}

// (x, y, red)
const BUTTON_TEMPLATE: &[(i32, i32, u8)] = &[
    (19, 9, 0x56),
    (22, 9, 0xe4),
    (27, 9, 0x55),
    (9, 23, 0x57),
    (15, 23, 0xe4),
    (22, 23, 0x55),
    (7, 37, 0x56),
    (13, 37, 0xe4),
    (20, 37, 0x57),
    (7, 53, 0x57),
    (13, 53, 0xc7),
    (20, 53, 0x59),
    (36, 30, 0x54),
    (36, 36, 0xa5),
    (36, 46, 0x57),
    (36, 50, 0xa5),
    (36, 59, 0x58),
];
const BUTTON_MIN_X: i32 = 1400;
const BUTTON_MAX_X: i32 = 1920;
const BUTTON_MIN_Y: i32 = 880;
const BUTTON_MAX_Y: i32 = 1080;

fn is_button(screen: &ScreenCaptureState, x: i32, y: i32) -> Result<bool, String> {
    let mut max_diff = i32::MIN;
    let mut min_diff = i32::MAX;
    let mut sum_diff = 0;
    for &(x2, y2, r2) in BUTTON_TEMPLATE.iter() {
        let color = screen.get_color(x + x2, y + y2)?;
        let r = color.0;
        let diff: i32 = r as i32 - r2 as i32;
        if diff > max_diff {
            max_diff = diff;
        }
        if diff < min_diff {
            min_diff = diff;
        }
        sum_diff += diff;
    }
    let matched = (max_diff - min_diff) < 100 && (sum_diff.abs() / BUTTON_TEMPLATE.len() as i32) < 50;
    Ok(matched)
}

fn main() {
    let mut screen = ScreenCaptureState::new(MIN_X, MIN_Y, MAX_X + 1, MAX_Y + 1).unwrap();
    let mut screen2 = ScreenCaptureState::new(BUTTON_MIN_X, BUTTON_MIN_Y, BUTTON_MAX_X, BUTTON_MAX_Y).unwrap();
    let mut prev_mouse_down = false;
    loop {
        screen.capture().unwrap();
        let mut indicator_y = -1;
        let mut indicator_x = -1;
        let mut range_min_x = -1;
        let mut range_min_y = -1;
        let mut range_max_x = -1;

        'outer1: for y in MIN_Y..=MAX_Y {
            for x in MIN_X..=MAX_X {
                if is_indicator(&screen, x, y) == Ok(true) {
                    indicator_x = x;
                    indicator_y = y;
                    break 'outer1;
                }
            }
        }
        if indicator_x >= 0 {
            let min_y = indicator_y - 10;
            let max_y = indicator_y + 10;
            'outer2: for y in min_y..=max_y {
                for x in MIN_X..=MAX_X {
                    if is_left_arrow(&screen, x, y) == Ok(true) {
                        range_min_x = x;
                        range_min_y = y;
                        break 'outer2;
                    }
                }
            }
            if range_min_x < 0 {
                range_min_x = indicator_x - 10;
            }
        }
        if indicator_x >= 0 {
            let min_y = indicator_y - 10;
            let max_y = indicator_y + 10;
            'outer3: for y in min_y..=max_y {
                for x in MIN_X..=MAX_X {
                    if is_right_arrow(&screen, x, y) == Ok(true) {
                        range_max_x = x;
                        break 'outer3;
                    }
                }
            }
            if range_max_x < 0 {
                range_max_x = indicator_x + 10;
            }
        }

        let mouse_down = if indicator_x >= 0 {
            let mouse_down = indicator_x < (range_min_x + (range_max_x - range_min_x) / 3);
            println!("range: [{}, {}] indicator: {} mouse_down: {:#?} (y: {})", range_min_x, range_max_x, indicator_x, mouse_down, range_min_y);
            mouse_down
        } else {
            screen2.capture().unwrap();
            'outer4: for y in BUTTON_MIN_Y..BUTTON_MAX_Y {
                for x in BUTTON_MIN_X..BUTTON_MAX_X {
                    if is_button(&screen2, x, y) == Ok(true) {
                        println!("found button: ({}, {})", x + 32, y + 32);
                        set_mouse_state(true).unwrap();
                        set_mouse_state(false).unwrap();
                        break 'outer4;
                    }
                }
            }
            false
        };
        if mouse_down != prev_mouse_down {
            set_mouse_state(mouse_down).unwrap();
            prev_mouse_down = mouse_down;
        }

        sleep(Duration::from_millis(50));
    }
}