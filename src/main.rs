use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("Space Shooter")]
async fn main() {
    const MOVE_SPEED: f32 = 200.0;

    // Seed the random number generator
    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVE_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    let mut gameover = false;

    loop {
        clear_background(DARKPURPLE);

        if !gameover {
            let delta_time = get_frame_time();
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                circle.x += MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                circle.x -= MOVE_SPEED * delta_time
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                circle.y += MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                circle.y -= MOVE_SPEED * delta_time
            }

            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            // Generate a new square
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                });
            }

            // Movement
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Remove shapes outside of screen
            squares.retain(|square| square.y < screen_height() + square.size);
        }

        // Check for collisions
        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        // Draw everything
        draw_circle(circle.x, circle.y, 16.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}
