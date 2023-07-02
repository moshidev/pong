mod pong;
use crate::pong::{Pong, RacketMov};

use crossterm::event::{poll, read, Event, KeyEventKind, KeyCode};
use std::time::{Instant, Duration};

fn main() {
    let mut game = Pong::build(26, 16);
    crossterm::terminal::enable_raw_mode().unwrap();

    'game_loop: loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", game);
        println!("Press q for quitting!");
        
        let mut racket_a_mov : Option<RacketMov> = None;
        let mut racket_b_mov : Option<RacketMov> = None;

        let start = Instant::now();
        while Instant::now().duration_since(start).as_millis() < 200 {
            if poll(Duration::from_millis(200)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Char('q') => { break 'game_loop; },
                                KeyCode::Char('a') => { racket_a_mov = Some(RacketMov::Up); },
                                KeyCode::Char('z') => { racket_a_mov = Some(RacketMov::Down); },
                                KeyCode::Char('k') => { racket_b_mov = Some(RacketMov::Up); },
                                KeyCode::Char('m') => { racket_b_mov = Some(RacketMov::Down); },
                                _ => {},
                            }
                        }
                    },
                    _ => {},
                }
            }
        }

        game.tick(racket_a_mov, racket_b_mov);
    }

    crossterm::terminal::disable_raw_mode().unwrap();
}

impl std::fmt::Display for Pong {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.columns {
                let mut c = '.';

                if j == self.racket_a.x && i >= self.racket_a.y && i < self.racket_a.y + self.racket_a.size {
                    c = 'R';
                }
                else if j == self.racket_b.x && i >= self.racket_b.y && i < self.racket_b.y + self.racket_b.size {
                    c = 'R';
                }
                else if j >= self.ball.x && j < self.ball.x + self.ball.size &&
                        i >= self.ball.y && i < self.ball.y + self.ball.size {
                    c = 'B';
                }

                match write!(f, " {} ", c) {
                    Err(a) => return Err(a),
                    _ => {},
                }
            }
            match write!(f, "\n") {
                Err(a) => return Err(a),
                _ => {},
            }
        }
        write!(f, "SCORED A: {} ||| SCORED B: {}", self.score_a, self.score_b)
    }
}
