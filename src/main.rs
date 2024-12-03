use macroquad::prelude::*;

#[macroquad::main("Space Shooter")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
