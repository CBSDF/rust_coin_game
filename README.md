# Coin Catcher

Простая 2D-игра на Rust и [macroquad](https://macroquad.rs/).
Ловите монетки и уворачивайтесь от бомб.

## Как играть

- **W / A / S / D** — движение
- Собирайте жёлтые монетки (+1 очко)
- Избегайте бомб: каждая отнимает часть HP
- **Enter** — начать заново после проигрыша
- **M** — вернуться в меню

## Запуск

Скачайте готовую версию во вкладке [Releases](../../releases)
или соберите сами:

    git clone https://github.com/CBSDF/rust_coin_game.git
    cd rust_coin_game
    cargo run --release

Нужен установленный [Rust](https://rustup.rs/).

## Что внутри

- Состояния игры через `enum` (меню, игра, game over)
- Трейт `CircleCollider` и коллизии круга с прямоугольником
- Текстуры вшиты в exe через `include_bytes!`

## Автор

CBSDF