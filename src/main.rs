use std::thread::sleep;
use std::time::Duration;

use genshin_auto_fish::genshin::{find_fish_button, find_indicator, find_left_arrow, find_right_arrow};
use genshin_auto_fish::windows::{set_dpi_aware, set_mouse_state, WindowCapture};

fn main() {
    set_dpi_aware();
    let mut capture = WindowCapture::new("UnityWndClass".to_string(), "原神".to_string());
    let mut is_prev_down = false;
    loop {
        if let Ok(()) = capture.capture() {
            let is_down = if let Some((indicator_x, indicator_y)) = find_indicator(&capture) {
                let left_arrow_x = if let Some((left_arrow_x, _)) = find_left_arrow(&capture, indicator_y) {
                    left_arrow_x
                } else {
                    indicator_x
                };
                let right_arrow_x = if let Some((right_arrow_x, _)) = find_right_arrow(&capture, indicator_y) {
                    right_arrow_x
                } else {
                    indicator_x
                };
                let width = capture.get_width();
                let range_left_x = (0.35 * width as f32) as usize;
                let range_width = (0.29 * width as f32) as usize;

                const N: usize = 40;
                let mut s = vec![b' '; N];
                let indicator_i = (indicator_x - range_left_x) * N / range_width;
                let left_arrow_i = (left_arrow_x - range_left_x) * N / range_width;
                let right_arrow_i = (right_arrow_x - range_left_x) * N / range_width;
                s[left_arrow_i] = b'<';
                s[right_arrow_i] = b'>';
                s[indicator_i] = b'|';
                let s = String::from_utf8(s).unwrap();
                let is_down = indicator_x < left_arrow_x + (right_arrow_x - left_arrow_x) / 3;
                println!("[{}] {}", s, is_down);
                is_down
            } else {
                if let Some((x, y)) = find_fish_button(&capture) {
                    println!("Found fish button: ({}, {})", x, y);
                    true
                } else {
                    false
                }
            };
            if is_down != is_prev_down {
                set_mouse_state(is_down);
                is_prev_down = is_down;
            }
        }
        sleep(Duration::from_millis(50));
    }
}