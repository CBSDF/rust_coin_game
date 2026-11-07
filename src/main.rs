mod ui;
mod game;
mod entities;

use macroquad::prelude::*;
use ui::button;
use game::*;
use entities::*;


#[macroquad::main("Coin Catcher")]
async fn main() {

    let player_tex = Texture2D::from_file_with_format(include_bytes!("../assets/player.png"), None);
    let coin_tex = Texture2D::from_file_with_format(include_bytes!("../assets/coin.png"), None);
    let bomb_tex = Texture2D::from_file_with_format(include_bytes!("../assets/bomb.png"), None);
    let bg_tex = Texture2D::from_file_with_format(include_bytes!("../assets/background.png"), None);

    let mut player = Player {
        x: 200.0,
        y: 400.0,
        hp: MAX_HP,
        size: 50.0,
        speed: 5.0,
    };

    let mut score = 0;

    let mut coins: Vec<Coin> = Vec::new();

    for _ in 0..7 {
        coins.push(Coin {
            x: rand::gen_range(15.0, screen_width() - 15.0),
            y: rand::gen_range(-200.0, 0.0),
            radius: 15.0,
            speed: rand::gen_range(2.0, 6.0),
        });
    }

    let mut bombs: Vec<Bomb> = Vec::new();
    for _ in 0..4 {
        bombs.push(Bomb {
            x: rand::gen_range(15.0, screen_width() - 15.0),
            y: rand::gen_range(-300.0, 0.0),
            radius: 15.0,
            speed: rand::gen_range(3.0, 6.0),
            damage: rand::gen_range(20, 60),
        });
    }

    let mut state = GameState::Menu;

    loop {
        clear_background(BLUE);

        draw_texture_ex(
    &bg_tex,
            0.0,
            0.0,
        WHITE,
    DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
        },
);


        match state {
            GameState::Menu => {
                if button("Play", screen_width() / 2.0 - 100.0, 200.0, 200.0, 50.0) {
                    reset_game(&mut player, &mut score, &mut coins, &mut bombs);
                    state = GameState::Play;
                }
                if button("Exit", screen_width() / 2.0 - 100.0, 250.0, 200.0, 50.0) {
                    break;
                }
            }
            GameState::Play => {
                handle_input(&mut player);

                if player.hp <= 0 {
                    state = GameState::GameOver;
                }

                for coin in &mut coins {
                    coin.y += coin.speed;
                    
                    

                    if coin.y > screen_height() {
                        coin.respawn();
                    }
                
                    if check_collision(&player, coin) {
                        score += 1;
                        coin.respawn();
                    }
                
                    draw_texture(&coin_tex, coin.x - coin.radius, coin.y - coin.radius, WHITE);
                }
            
                for bomb in &mut bombs {
                    bomb.y += bomb.speed;
                
                    if bomb.y > screen_height() {
                        bomb.respawn();
                    }
                
                    if check_collision(&player, bomb) {
                        player.hp -= bomb.damage;
                        bomb.respawn();
                    }
                
                    draw_texture(&bomb_tex, bomb.x - bomb.radius, bomb.y - bomb.radius, WHITE);
                }
            
                draw_texture(&player_tex, player.x, player.y, WHITE);
            
                draw_text(&format!("Score: {}", score), 20.0, 40.0, 30.0, GOLD);
            
                draw_text(&format!("HP: {}", player.hp.max(0)), 20.0, 80.0, 30.0, RED);


            }
            GameState::GameOver => {
                    draw_text(
                        "GAME OVER",
                        screen_width() / 2.0 - 130.0,
                        screen_height() / 2.0,
                        60.0,
                        RED,
                    );
                    draw_text(
                        &format!("Final Score: {}", score),
                        screen_width() / 2.0 - 100.0,
                        screen_height() / 2.0 + 50.0,
                        30.0,
                        WHITE,
                    );
                    draw_text(
                        "Press ENTER to restart",
                        screen_width() / 2.0 - 130.0,
                        screen_height() / 2.0 + 100.0,
                        25.0,
                        LIGHTGRAY,
                    );
                    draw_text(
                        "Press M to return to the menu",
                        screen_width() / 2.0 - 130.0,
                        screen_height() / 2.0 + 130.0,
                        25.0,
                        LIGHTGRAY,
                    );

                    if is_key_pressed(KeyCode::Enter) {
                        reset_game(&mut player, &mut score, &mut coins, &mut bombs);
                        state = GameState::Play;
                    }
                    if is_key_pressed(KeyCode::M) {
                        state = GameState::Menu;
                    }
                }
            
            }
            next_frame().await;
        }
    }

