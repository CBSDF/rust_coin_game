use macroquad::prelude::*;
use crate::entities::*;

pub const MAX_HP: i32 = 100;

pub enum GameState {
    Menu,
    Play,
    GameOver,
}

pub fn handle_input(player: &mut Player) {
    if is_key_down(KeyCode::A) && player.x > 0.0 {
        player.x -= player.speed;
    }
    if is_key_down(KeyCode::D) && player.x < screen_width() - player.size {
        player.x += player.speed;
    }
    if is_key_down(KeyCode::W) && player.y > 0.0 {
        player.y -= player.speed;
    }
    if is_key_down(KeyCode::S) && player.y < screen_height() - player.size {
        player.y += player.speed;
    }
}

pub fn reset_game(player: &mut Player, score: &mut i32, coins: &mut [Coin], bombs: &mut [Bomb]) {
    player.hp = MAX_HP;
    *score = 0;
    player.x = 200.0;
    player.y = 400.0;
    for coin in coins.iter_mut(){
        coin.respawn();
    }
    for bomb in bombs.iter_mut(){
        bomb.respawn();
    }
}