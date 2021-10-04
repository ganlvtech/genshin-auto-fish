use crate::Color;
use crate::windows::WindowCapture;

const LIGHT_YELLOW: Color = Color(0xff, 0xff, 0xc0);

pub fn find_indicator(capture: &WindowCapture) -> Option<(usize, usize)> {
    let width = capture.get_width();
    let height = capture.get_height();
    let indicator_half_height = (0.01 * (height as f32)) as usize;
    for y in indicator_half_height..=((0.33 * (height as f32)) as usize) {
        for x in ((0.35 * (width as f32)) as usize)..=((0.64 * (width as f32)) as usize) {
            let mut found = true;
            unsafe {
                if capture.get_color_unchecked(x as usize, y) != LIGHT_YELLOW {
                    found = false;
                } else {
                    for i in 1..=indicator_half_height {
                        if capture.get_color_unchecked(x as usize, (y - i) as usize) != LIGHT_YELLOW {
                            found = false;
                            break;
                        }
                        if capture.get_color_unchecked(x as usize, (y + i) as usize) != LIGHT_YELLOW {
                            found = false;
                            break;
                        }
                    }
                }
            }
            if found {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn find_left_arrow(capture: &WindowCapture, indicator_y: usize) -> Option<(usize, usize)> {
    let width = capture.get_width();
    let height = capture.get_height();
    let arrow_half_height = (0.007 * (height as f32)) as usize;
    for y in (indicator_y - arrow_half_height)..=(indicator_y + arrow_half_height) {
        for x in ((0.35 * (width as f32)) as usize)..=((0.64 * (width as f32)) as usize) {
            let mut found = true;
            unsafe {
                if capture.get_color_unchecked(x as usize, y) != LIGHT_YELLOW {
                    found = false;
                } else {
                    for i in 1..=arrow_half_height {
                        if capture.get_color_unchecked((x + i) as usize, (y - i) as usize) != LIGHT_YELLOW {
                            found = false;
                            break;
                        }
                        if capture.get_color_unchecked((x + i) as usize, (y + i) as usize) != LIGHT_YELLOW {
                            found = false;
                            break;
                        }
                    }
                }
            }
            if found {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn find_right_arrow(capture: &WindowCapture, indicator_y: usize) -> Option<(usize, usize)> {
    let width = capture.get_width();
    let height = capture.get_height();
    let arrow_half_height = (0.007 * (height as f32)) as usize;
    for y in (indicator_y - arrow_half_height)..=(indicator_y + arrow_half_height) {
        for x in ((0.35 * (width as f32)) as usize)..=((0.64 * (width as f32)) as usize) {
            let mut found = true;
            unsafe {
                if capture.get_color_unchecked(x as usize, y) != LIGHT_YELLOW {
                    found = false;
                } else {
                    for i in 1..=arrow_half_height {
                        if capture.get_color_unchecked((x - i) as usize, (y - i) as usize) != LIGHT_YELLOW {
                            found = false;
                            break;
                        }
                        if capture.get_color_unchecked((x - i) as usize, (y + i) as usize) != LIGHT_YELLOW {
                            found = false;
                            break;
                        }
                    }
                }
            }
            if found {
                return Some((x, y));
            }
        }
    }
    None
}

const BUTTON_SIZE: (usize, usize) = (76, 76);
const BUTTON_TEMPLATE_WINDOW_SIZE: (usize, usize) = (1920, 1080);
// (x, y, red)
const BUTTON_TEMPLATE: &[(usize, usize, u8)] = &[
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

pub fn find_fish_button(capture: &WindowCapture) -> Option<(usize, usize)> {
    let width = capture.get_width();
    let height = capture.get_height();
    let scale = width as f32 / BUTTON_TEMPLATE_WINDOW_SIZE.0 as f32;
    let button_width = (BUTTON_SIZE.0 as f32 * scale) as usize;
    let button_height = (BUTTON_SIZE.1 as f32 * scale) as usize;
    for x in ((0.8 * (width as f32)) as usize)..(width - button_width) {
        for y in ((0.85 * (height as f32)) as usize)..(height - button_height) {
            let mut max_diff = i32::MIN;
            let mut min_diff = i32::MAX;
            let mut sum_diff = 0;
            for &(x2, y2, r2) in BUTTON_TEMPLATE.iter() {
                let color = unsafe { capture.get_color_unchecked(x + (x2 as f32 * scale) as usize, y + (y2 as f32 * scale) as usize) };
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
            let found = (max_diff - min_diff) < 100 && (sum_diff.abs() / BUTTON_TEMPLATE.len() as i32) < 50;
            if found {
                return Some((x, y));
            }
        }
    }
    None
}
