use macroquad::prelude::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut original_points = vec![];
    let mut points = vec![];
    let mut count = 0;
    let mut is_animating = false;
    let mut last_step_time = 0.0;
    let delay = 0.5;

    loop {
        clear_background(BLACK);
        draw_text("Left-click to add points", 20.0, 20.0, 20.0, WHITE);
        draw_text("Press Enter to start animation", 20.0, 40.0, 20.0, WHITE);
        draw_text("Press Space to reset", 20.0, 60.0, 20.0, WHITE);
        draw_text("Press Escape to quit", 20.0, 80.0, 20.0, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) && !is_animating {
            let (x,y) = mouse_position();
            original_points.push(vec2(x,y));
            points.push(vec2(x,y));
        }

        if is_key_pressed(KeyCode::Enter) && points.len() > 2 && !is_animating {
            count = 0;
            is_animating = true;
            last_step_time = get_time();
        }

        if is_animating && get_time() - last_step_time > delay {
            let mut new_points = vec![];
            for i in 0..points.len().saturating_sub(1) {
                let p0 = points[i];
                let p1 = points[i+1];

                let first_cut = p0 + 0.25 * (p1-p0);
                let second_cut = p0 + 0.75 * (p1-p0);

                new_points.push(first_cut);
                new_points.push(second_cut);
            }

            points = new_points;
            count+=1;
            if count == 7 {
                count = 0;
                points = original_points.clone();
            }
            last_step_time = get_time();
        }

        if is_animating {
            draw_text(
                format!("{:02}/07 steps", count + 1).as_str(),
                20.0,
                100.0,
                20.0,
                WHITE,
            );
        }

        for i in 0..points.len().saturating_sub(1) {
            draw_line(
                points[i].x,
                points[i].y,
                points[i + 1].x,
                points[i + 1].y,
                2.0,
                RED,
            );
        }

        for point in &points {
            draw_circle(point.x,point.y, 4.0, WHITE);
        }

         if is_key_pressed(KeyCode::Space) {
            original_points.clear();
            points.clear();
            is_animating = false;
            count = 0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}   