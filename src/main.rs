mod pong;
use crate::pong::{Pong, RacketMov};

use console::Term;

fn main() {
    let stdout = Term::buffered_stdout();
    let mut game = Pong::build(24, 12);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", game);

        let mut racket_a_mov : Option<RacketMov> = None;
        if let Ok(c) = stdout.read_char() {
            match c {
                'j' => racket_a_mov = Some(RacketMov::Up),
                'k' => racket_a_mov = Some(RacketMov::Down),
                'q' => break,
                _ => (),
            }
        }
        stdout.clear_line().unwrap();

        game.tick(racket_a_mov, None);
    }
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
