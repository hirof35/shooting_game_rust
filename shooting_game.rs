use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

// --- 1. データ定義 (Data Structures) ---

enum GameState {
    Title,
    Stage,
    BossBattle,
    GameOver,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    active: bool,
}

struct Enemy {
    pos: Vec2,
    health: i32,
    enemy_type: EnemyType,
}

enum EnemyType { Basic, Boss }

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
}

// --- 2. メイン関数 (Main Loop) ---

#[macroquad::main("Rust STG Full Structure")]
async fn main() {
    // アセットロード
    let snd_shot = load_sound("assets/shot.wav").await.unwrap();
    let snd_expl = load_sound("assets/expl.wav").await.unwrap();

    // ゲーム状態
    let mut state = GameState::Title;
    let mut player_pos = vec2(screen_width() / 2.0, screen_height() - 50.0);
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut score = 0;

    loop {
        clear_background(BLACK);

        match state {
            GameState::Title => {
                draw_text("PRESS SPACE TO START", 150.0, 300.0, 30.0, WHITE);
                if is_key_pressed(KeyCode::Space) { state = GameState::Stage; }
            }
            GameState::Stage | GameState::BossBattle => {
                // --- A. 入力・移動処理 ---
                handle_input(&mut player_pos, &mut bullets, &snd_shot);

                // --- B. 更新処理 (Update) ---
                update_bullets(&mut bullets);
                update_enemies(&mut enemies, &mut state, &score);
                update_particles(&mut particles);

                // --- C. 当たり判定 (Collision) ---
                check_collisions(&mut bullets, &mut enemies, &mut particles, &mut score, &snd_expl);

                // --- D. 描画 (Draw) ---
                draw_player(player_pos);
                for b in &bullets { draw_circle(b.pos.x, b.pos.y, 2.0, YELLOW); }
                for e in &enemies {
                    let color = if let EnemyType::Boss = e.enemy_type { RED } else { WHITE };
                    draw_rectangle_centered(e.pos.x, e.pos.y, 30.0, 30.0, color);
                }
                for p in &particles {
                    draw_circle(p.pos.x, p.pos.y, 4.0 * p.life, Color::new(1.0, 0.5, 0.0, p.life));
                }
                
                draw_text(&format!("Score: {}", score), 10.0, 30.0, 30.0, WHITE);
            }
            GameState::GameOver => {
                draw_text("GAME OVER - PRESS R", 150.0, 300.0, 40.0, RED);
                if is_key_pressed(KeyCode::R) { /* 変数をリセットしてTitleへ */ }
            }
        }
        next_frame().await
    }
}

// --- 3. 各種ヘルパー関数 (Systems) ---

fn handle_input(pos: &mut Vec2, bullets: &mut Vec<Bullet>, snd: &Sound) {
    if is_key_down(KeyCode::Left)  { pos.x -= 5.0; }
    if is_key_down(KeyCode::Right) { pos.x += 5.0; }
    if is_key_pressed(KeyCode::Space) {
        bullets.push(Bullet { pos: *pos, vel: vec2(0.0, -10.0), active: true });
        play_sound(snd, PlaySoundParams { looped: false, volume: 0.3 });
    }
}

fn check_collisions(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>, particles: &mut Vec<Particle>, score: &mut i32, snd: &Sound) {
    for b in bullets.iter_mut() {
        for e in enemies.iter_mut() {
            if (b.pos - e.pos).length() < 20.0 {
                b.active = false;
                e.health -= 1;
                if e.health <= 0 {
                    *score += 100;
                    play_sound(snd, PlaySoundParams { looped: false, volume: 0.5 });
                    // パーティクル生成
                    for _ in 0..10 {
                        particles.push(Particle {
                            pos: e.pos,
                            vel: vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0)),
                            life: 1.0,
                        });
                    }
                }
            }
        }
    }
    bullets.retain(|b| b.active && b.pos.y > 0.0);
    enemies.retain(|e| e.health > 0);
}

// (以下、update_bullets, update_enemies 等の関数が続く...)
fn update_bullets(bullets: &mut Vec<Bullet>) {
    for b in bullets.iter_mut() {
        b.pos += b.vel;
    }
}

fn update_enemies(enemies: &mut Vec<Enemy>, state: &mut GameState, score: &i32) {
    for e in enemies.iter_mut() {
        e.pos.y += 1.0; // ゆっくり降りてくる
    }
}

fn update_particles(particles: &mut Vec<Particle>) {
    particles.retain_mut(|p| {
        p.pos += p.vel;
        p.life -= 0.02;
        p.life > 0.0
    });
}

fn draw_player(pos: Vec2) {
    draw_triangle(
        vec2(pos.x, pos.y - 15.0),
        vec2(pos.x - 15.0, pos.y + 15.0),
        vec2(pos.x + 15.0, pos.y + 15.0),
        WHITE,
    );
}

fn draw_rectangle_centered(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(x - w / 2.0, y - h / 2.0, w, h, color);
}
