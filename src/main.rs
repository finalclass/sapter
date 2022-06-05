use ruscii::app::{App, Config, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

fn main() {
    let mut app = App::config(Config::new().fps(20));
    let mut game_state = sapter::GameState::new(10, 10, 10);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::Right) => game_state.move_player_right(),
                KeyEvent::Pressed(Key::Left) => game_state.move_player_left(),
                KeyEvent::Pressed(Key::Up) => game_state.move_player_up(),
                KeyEvent::Pressed(Key::Down) => game_state.move_player_down(),
                KeyEvent::Pressed(Key::Space) => game_state.reveal().unwrap(),
                KeyEvent::Pressed(Key::Backspace) => game_state.flag().unwrap(),
                KeyEvent::Pressed(Key::R) => game_state.resurect(),
                KeyEvent::Pressed(Key::N) => game_state = sapter::GameState::new(10, 10, 10),
                _ => (),
            }
        }

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.set_foreground(Color::Grey);

        for x in 0..game_state.width {
            for y in 0..game_state.height {
                let pos = Vec2::xy(x as i32 * 5, y as i32 * 3 + 8);

                let mut text = String::from("");
                let mut color = Color::Grey;

                if game_state.is_flagged(x, y) {
                    color = Color::Xterm(56);
                    text = String::from("!")
                } else if game_state.is_opened(x, y) {
                    if game_state.has_mine(x, y) {
                        color = Color::Red;
                        text = String::from("*");
                    } else {
                        color = Color::White;
                        let nof_mines = game_state.count_mines_around(x, y);
                        if nof_mines > 0 {
                            text = format!("{}", nof_mines);
                        }

                        if nof_mines == 1 {
                            color = Color::Green;
                        }

                        if nof_mines == 2 {
                            color = Color::Yellow;
                        }

                        if nof_mines == 3 {
                            color = Color::Xterm(208);
                        }

                        if nof_mines > 3 {
                            color = Color::Magenta;
                        }
                    }
                } else {
                    text = String::from("?");
                }

                pencil.set_foreground(color);

                if game_state.is_player(x, y) {
                    pencil.draw_rect(&RectCharset::double_lines(), pos, Vec2::xy(5, 3));
                } else {
                    pencil.draw_rect(&RectCharset::simple_lines(), pos, Vec2::xy(5, 3));
                }

                pencil.draw_text(&text, pos + Vec2::xy(2, 1));
                pencil.set_foreground(Color::Grey);
            }
        }

        pencil
            .draw_text("ARROWS - Move", Vec2::xy(0, 0))
            .draw_text("SPACE - reveal field", Vec2::xy(0, 1))
            .draw_text("BACKSPACE - Flag field", Vec2::xy(0, 2))
            .draw_text("Q or ESC - quit", Vec2::xy(0, 3))
            .draw_text("R - resurect", Vec2::xy(0, 4))
            .draw_text("N - new game", Vec2::xy(0, 5));

        let t = format!("Mines left: {}", game_state.mines_left());
        pencil.draw_text(&t, Vec2::xy(0, 7));

        if game_state.check_has_won() {
            pencil.set_foreground(Color::Green);
            pencil.draw_text("Victory!", Vec2::xy(0, 8));
            pencil.set_foreground(Color::Grey);
        }
    });
}
