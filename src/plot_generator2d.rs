use crate::expression::Expression;
use honestintervals::IntervalSet;
use std::collections::VecDeque;

pub struct Rectangle {
    pub x_start: f64,
    pub y_start: f64,
    pub x_end: f64,
    pub y_end: f64,
}

pub fn generate_2dplot_implicit(expression: &Expression<IntervalSet<f64>>, display_info: Rectangle, resolution: u32) -> Vec<Rectangle> {
    let smallest_quadrant = (display_info.x_end - display_info.x_start) / resolution as f64;
    let max_rectangles = resolution*resolution/100;
    let mut queue: VecDeque<Rectangle> = VecDeque::new();
    queue.push_back(display_info);
    while let Some(rect) = queue.pop_front() {
        if rect.y_end - rect.y_start < smallest_quadrant || 
            rect.x_end - rect.x_start < smallest_quadrant ||
            queue.len() > max_rectangles as usize {
            break;
        }
        queue.append(&mut generate_subquadrants(expression, rect));
    }
    queue.into()
}

fn generate_subquadrants(expression: &Expression<IntervalSet<f64>>, rect: Rectangle) -> VecDeque<Rectangle> {
    let x_half = (rect.x_start + rect.x_end) / 2.0;
    let y_half = (rect.y_start + rect.y_end) / 2.0;

    let mut subquadrants = VecDeque::new();

    let mut add_if_has_zero = |quadrant: Rectangle| {
        let x_interval = IntervalSet::new(quadrant.x_start, quadrant.x_end);
        let y_interval = IntervalSet::new(quadrant.y_start, quadrant.y_end);
        let eval = expression.eval_3d(x_interval, y_interval);
        if eval.has_zero() {
            subquadrants.push_back(quadrant)
        }
    };

    let quadrant = Rectangle {
        x_start: x_half,
        y_start: y_half,
        x_end: rect.x_end,
        y_end: rect.y_end
    };
    add_if_has_zero(quadrant);

    let quadrant = Rectangle {
        x_start: rect.x_start,
        y_start: y_half,
        x_end: x_half,
        y_end: rect.y_end
    };
    add_if_has_zero(quadrant);

    let quadrant = Rectangle {
        x_start: rect.x_start,
        y_start: rect.y_start,
        x_end: x_half,
        y_end: y_half
    };
    add_if_has_zero(quadrant);

    let quadrant = Rectangle {
        x_start: x_half,
        y_start: rect.y_start,
        x_end: rect.x_end,
        y_end: y_half
    };
    add_if_has_zero(quadrant);

    subquadrants
}

// Given the DisplayInfo, it returns an approximation of the plot
// consistings as a list of rectangles that should be displayed
pub fn generate_2dplot(expression: &Expression<IntervalSet<f64>>, display_info: Rectangle, resolution: u32) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    let step = (display_info.x_end - display_info.x_start) / resolution as f64;
    let mut x_0 = display_info.x_start;
    while x_0 < display_info.x_end {

        let x_1 = x_0 + step;
        let x_interval = IntervalSet::new(x_0, x_1);
        let y_intervals: Vec<(f64, f64)> = expression.eval_2d(x_interval).into();

        for interval in y_intervals {
            if (interval.0 > display_info.y_end && interval.1 > display_info.y_end) ||
                (interval.0 < display_info.y_start && interval.1 < display_info.y_start) {
                    continue;
            }

            rectangles.push(Rectangle {
                x_start: x_0,
                y_start: interval.0.max(display_info.y_start).min(display_info.y_end),
                x_end: x_1,
                y_end: interval.1.max(display_info.y_start).min(display_info.y_end)
            });
        }

        x_0 += step;
    }

    return rectangles
}
