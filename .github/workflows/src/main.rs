use macroquad::prelude::*;

const SQUASH_SIZE: f32 = 20.0;

struct Snake {
    body: Vec<Vec2>,
        dir: Vec2,
            next_dir: Vec2,
            }

            #[macroquad::main("Yilan Oyunu")]
            async fn main() {
                let mut snake = Snake {
                        body: vec![vec2(10.0, 10.0)],
                                dir: vec2(1.0, 0.0),
                                        next_dir: vec2(1.0, 0.0),
                                            };
                                                let mut food = vec2(5.0, 5.0);
                                                    let mut last_update = get_time();

                                                        loop {
                                                                clear_background(BLACK);

                                                                        // --- Girdiler (Android Uyumluluğu İçin) ---
                                                                                if is_key_pressed(KeyCode::Up) { snake.next_dir = vec2(0.0, -1.0); }
                                                                                        if is_key_pressed(KeyCode::Down) { snake.next_dir = vec2(0.0, 1.0); }
                                                                                                if is_key_pressed(KeyCode::Left) { snake.next_dir = vec2(-1.0, 0.0); }
                                                                                                        if is_key_pressed(KeyCode::Right) { snake.next_dir = vec2(1.0, 0.0); }

                                                                                                                // Basit dokunmatik kontrol (Ekranın yarısına göre yön belirleme)
                                                                                                                        if is_mouse_button_pressed(MouseButton::Left) {
                                                                                                                                    let (mx, my) = mouse_position();
                                                                                                                                                let center_x = screen_width() / 2.0;
                                                                                                                                                            let center_y = screen_height() / 2.0;
                                                                                                                                                                        
                                                                                                                                                                                    if (mx - center_x).abs() > (my - center_y).abs() {
                                                                                                                                                                                                    snake.next_dir = vec2((mx - center_x).signum(), 0.0);
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                                snake.next_dir = vec2(0.0, (my - center_y).signum());
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                    }

                                                                                                                                                                                                                                                            // --- Güncelleme Mantığı (Saniyede 10 kez) ---
                                                                                                                                                                                                                                                                    if get_time() - last_update > 0.1 {
                                                                                                                                                                                                                                                                                last_update = get_time();
                                                                                                                                                                                                                                                                                            snake.dir = snake.next_dir;
                                                                                                                                                                                                                                                                                                        let new_head = snake.body[0] + snake.dir;
                                                                                                                                                                                                                                                                                                                    
                                                                                                                                                                                                                                                                                                                                snake.body.insert(0, new_head);
                                                                                                                                                                                                                                                                                                                                            if new_head == food {
                                                                                                                                                                                                                                                                                                                                                            food = vec2((rand::gen_range(0, 20)) as f32, (rand::gen_range(0, 20)) as f32);
                                                                                                                                                                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                                                                                                                                                                                        snake.body.pop();
                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                            }

                                                                                                                                                                                                                                                                                                                                                                                                                    // --- Çizim ---
                                                                                                                                                                                                                                                                                                                                                                                                                            for part in &snake.body {
                                                                                                                                                                                                                                                                                                                                                                                                                                        draw_rectangle(part.x * SQUASH_SIZE, part.y * SQUASH_SIZE, SQUASH_SIZE - 1.0, SQUASH_SIZE - 1.0, GREEN);
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        draw_rectangle(food.x * SQUASH_SIZE, food.y * SQUASH_SIZE, SQUASH_SIZE - 1.0, SQUASH_SIZE - 1.0, RED);

                                                                                                                                                                                                                                                                                                                                                                                                                                                                next_frame().await
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    