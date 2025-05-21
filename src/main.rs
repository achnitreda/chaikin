use macroquad::prelude::*;

#[macroquad::main("Chaikin")]
async fn main() {
    let mut points = vec![];

    loop {
        // clear_background(BLACK); ??
        draw_text("Left-click to add points", 20.0, 20.0, 20.0, WHITE);
        draw_text("Press Enter to start animation", 20.0, 40.0, 20.0, WHITE);
        draw_text("Press Escape to quit", 20.0, 60.0, 20.0, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x,y) = mouse_position();
            points.push(vec2(x,y));
        }

        if is_key_pressed(KeyCode::Enter) && points.len() > 1 {
            let mut current_points = points.clone();
            println!("-> {:?}",current_points);
            for i in 0..current_points.len()-1 {
                let p0 = current_points[i];
                let p1 = current_points[i+1];

                println!("{} {}",p0,p1);
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for point in &points {
            draw_circle(point.x,point.y,5.0, WHITE);
        }

        next_frame().await;
    }
}   