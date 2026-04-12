use macroquad::prelude::*;
use std::f32::consts::TAU;

// --- データ構造の定義 ---

#[derive(PartialEq)]
enum GameState { Title, Stage, GameClear, GameOver }

struct Bullet { pos: Vec2, vel: Vec2, active: bool }
struct Enemy { pos: Vec2, health: i32, is_boss: bool }
struct EnemyBullet { pos: Vec2, vel: Vec2, active: bool }
struct Particle { pos: Vec2, vel: Vec2, life: f32, color: Color }
struct Star { pos: Vec2, speed: f32, size: f32 }

// --- メイン関数 ---

#[macroquad::main("Rust Alpha Shooter")]
async fn main() {
    // 1. 各種変数の初期化
    let mut state = GameState::Title;
    let mut score: i32 = 0;
    let mut frame_count: u64 = 0;

    let mut player_pos = vec2(screen_width() / 2.0, screen_height() - 80.0);
    let mut player_lives = 3;
    let mut invincible_timer = 0.0;

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut enemy_bullets: Vec<EnemyBullet> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut stars: Vec<Star> = Vec::new();

    let mut boss_spawned = false;
    let mut warning_timer = 0.0;

    // 星空の初期化
    for _ in 0..100 {
        stars.push(Star {
            pos: vec2(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())),
            speed: rand::gen_range(1.0, 4.0),
            size: rand::gen_range(0.5, 2.0),
        });
    }

    // 2. ゲームループ
    loop {
        clear_background(BLACK);
        let delta = get_frame_time();

        // 背景の星の描画と更新
        for s in stars.iter_mut() {
            s.pos.y += s.speed;
            if s.pos.y > screen_height() { s.pos.y = 0.0; s.pos.x = rand::gen_range(0.0, screen_width()); }
            let b = s.speed / 4.0;
            draw_circle(s.pos.x, s.pos.y, s.size, Color::new(b, b, b, 1.0));
        }

        match state {
            GameState::Title => {
                draw_text("RUST ALPHA SHOOTER", screen_width()/2.0 - 160.0, 250.0, 40.0, BLUE);
                draw_text("PRESS SPACE TO START", screen_width()/2.0 - 130.0, 350.0, 25.0, WHITE);
                if is_key_pressed(KeyCode::Space) { state = GameState::Stage; }
            }

            GameState::Stage => {
                frame_count += 1;
                if invincible_timer > 0.0 { invincible_timer -= delta; }

                // --- プレイヤー操作 ---
                if is_key_down(KeyCode::Left) && player_pos.x > 20.0 { player_pos.x -= 7.0; }
                if is_key_down(KeyCode::Right) && player_pos.x < screen_width() - 20.0 { player_pos.x += 7.0; }
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Bullet { pos: player_pos, vel: vec2(0.0, -12.0), active: true });
                }

                // --- 敵の生成ロジック ---
                if score < 1000 {
                    if frame_count % 40 == 0 {
                        enemies.push(Enemy { pos: vec2(rand::gen_range(50.0, screen_width()-50.0), -20.0), health: 1, is_boss: false });
                    }
                } else if !boss_spawned {
                    enemies.push(Enemy { pos: vec2(screen_width()/2.0, -100.0), health: 10, is_boss: true });
                    boss_spawned = true;
                    warning_timer = 3.0;
                }

                // --- 更新処理 ---
                for b in bullets.iter_mut() { b.pos += b.vel; }
                for eb in enemy_bullets.iter_mut() { eb.pos += eb.vel; }
                
                for e in enemies.iter_mut() {
                    if e.is_boss {
                        if e.pos.y < 120.0 { e.pos.y += 1.0; }
                        else { 
                            e.pos.x += (frame_count as f32 * 0.03).sin() * 3.0;
                            // ボスの全方位弾
                            if frame_count % 80 == 0 {
                                for i in 0..12 {
                                    let angle = (i as f32 / 12.0) * TAU + (frame_count as f32 * 0.1);
                                    enemy_bullets.push(EnemyBullet { pos: e.pos, vel: vec2(angle.cos()*4.0, angle.sin()*4.0), active: true });
                                }
                            }
                        }
                    } else { e.pos.y += 3.0; }
                }

                // パーティクル更新
                for p in particles.iter_mut() {
                    p.pos += p.vel;
                    p.life -= 0.02;
                }

                // --- 当たり判定 ---
                for b in bullets.iter_mut() {
                    for e in enemies.iter_mut() {
                        let dist = if e.is_boss { 50.0 } else { 25.0 };
                        if b.active && (b.pos - e.pos).length() < dist {
                            b.active = false;
                            e.health -= 1;
                            // 被弾パーティクル
                            particles.push(Particle { pos: b.pos, vel: vec2(0.0, 2.0), life: 0.5, color: YELLOW });
                            if e.health <= 0 {
                                score += if e.is_boss { 5000 } else { 100 };
                                if e.is_boss { state = GameState::GameClear; }
                                // 撃破爆発
                                for _ in 0..20 {
                                    particles.push(Particle { pos: e.pos, vel: vec2(rand::gen_range(-4.0, 4.0), rand::gen_range(-4.0, 4.0)), life: 1.0, color: RED });
                                }
                            }
                        }
                    }
                }

                // プレイヤーへの被弾判定
                if invincible_timer <= 0.0 {
                    for eb in enemy_bullets.iter_mut() {
                        if eb.active && (eb.pos - player_pos).length() < 15.0 {
                            eb.active = false;
                            player_lives -= 1;
                            invincible_timer = 2.0;
                            if player_lives <= 0 { state = GameState::GameOver; }
                        }
                    }
                }

                // クリーンアップ
                bullets.retain(|b| b.active && b.pos.y > -10.0);
                enemies.retain(|e| e.health > 0 && e.pos.y < screen_height() + 50.0);
                enemy_bullets.retain(|eb| eb.active && eb.pos.y < screen_height() && eb.pos.y > 0.0);
                particles.retain(|p| p.life > 0.0);

                // --- 描画 ---
                // プレイヤー（無敵時の点滅）
                let p_color = if invincible_timer > 0.0 && (invincible_timer * 10.0) as i32 % 2 == 0 { Color::new(1.,1.,1.,0.3) } else { WHITE };
                draw_triangle(vec2(player_pos.x, player_pos.y-15.0), vec2(player_pos.x-15.0, player_pos.y+15.0), vec2(player_pos.x+15.0, player_pos.y+15.0), p_color);

                for e in &enemies {
                    if e.is_boss {
                        draw_rectangle(e.pos.x-40.0, e.pos.y-40.0, 80.0, 80.0, PURPLE);
                        draw_rectangle(e.pos.x-40.0, e.pos.y-55.0, (e.health as f32 / 100.0)*80.0, 8.0, GREEN);
                    } else { draw_rectangle(e.pos.x-15.0, e.pos.y-15.0, 30.0, 30.0, RED); }
                }
                for b in &bullets { draw_circle(b.pos.x, b.pos.y, 4.0, YELLOW); }
                for eb in &enemy_bullets { draw_circle(eb.pos.x, eb.pos.y, 3.0, ORANGE); }
                for p in &particles { draw_circle(p.pos.x, p.pos.y, 2.0 * p.life, p.color); }

                // UI
                draw_text(&format!("SCORE: {:07}", score), 20.0, 40.0, 30.0, WHITE);
                draw_text(&format!("LIVES: {}", player_lives), 20.0, 70.0, 30.0, RED);

                // WARNING表示
                if warning_timer > 0.0 {
                    warning_timer -= delta;
                    let alpha = (warning_timer * 8.0).sin().abs();
                    draw_text("WARNING!", screen_width()/2.0-100.0, screen_height()/2.0, 50.0, Color::new(1., 0., 0., alpha));
                }
            }

            GameState::GameClear | GameState::GameOver => {
                let msg = if state == GameState::GameClear { "MISSION COMPLETE" } else { "GAME OVER" };
                let col = if state == GameState::GameClear { GOLD } else { RED };
                draw_text(msg, screen_width()/2.0-140.0, screen_height()/2.0-20.0, 40.0, col);
                draw_text(&format!("FINAL SCORE: {}", score), screen_width()/2.0-100.0, screen_height()/2.0+40.0, 25.0, WHITE);
                draw_text("PRESS R TO TITLE", screen_width()/2.0-80.0, screen_height()/2.0+80.0, 20.0, GRAY);
                
                if is_key_pressed(KeyCode::R) {
                    // 全変数をリセット
                    score = 0; player_lives = 3; boss_spawned = false;
                    enemies.clear(); bullets.clear(); enemy_bullets.clear();
                    player_pos = vec2(screen_width()/2.0, screen_height()-80.0);
                    state = GameState::Title;
                }
            }
        }
        next_frame().await
    }
}
