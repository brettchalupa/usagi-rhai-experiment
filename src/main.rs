use rhai::{Engine, EvalAltResult, Scope, ImmutableString};
use macroquad::prelude::*;

#[macroquad::main("Usagi")]
async fn main() -> Result<(), Box<EvalAltResult>> {
    // Create an 'Engine'
    let mut engine = Engine::new();

    engine.register_fn("text", text)
        .register_fn("circle", draw_circle)
        .register_fn("fps", get_fps)
        .register_fn("pressed_up", pressed_up)
        .register_fn("pressed_down", pressed_down)
        .register_fn("pressed_right", pressed_right)
        .register_fn("pressed_left", pressed_left)
        .register_fn("screen_w", screen_width)
        .register_fn("screen_h", screen_height);

    let state = rhai::Map::new();
    let mut scope = Scope::new();                               // create custom scope
    scope.push_constant("WHITE", WHITE)
        .push("state", state);

    loop {
        engine.run_file_with_scope(&mut scope, "main.rhai".into())?;
        next_frame().await
    }
}

fn text(s: ImmutableString, x: f32, y: f32, size: f32, color: Color) {
    draw_text(&s, x, y, size, color);
}

fn pressed_up() -> bool {
    is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)
}

fn pressed_down() -> bool {
    is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)
}

fn pressed_left() -> bool {
    is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)
}

fn pressed_right() -> bool {
    is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)
}

