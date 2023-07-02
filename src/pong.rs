use std::cmp;

pub enum RacketMov {
    Up,
    Down,
}

#[derive(Clone)]
pub struct Pieze {
    pub size : i32,
    pub x : i32,
    pub y : i32,
}

pub struct Pong {
    pub rows : i32,
    pub columns : i32,
    pub score_a : i32,
    pub score_b : i32,
    pub racket_a : Pieze,
    pub racket_b : Pieze,
    pub ball : Pieze,
    pub ball_dir_x : i32,
    pub ball_dir_y : i32,
}

impl Pong {
    pub fn build(columns : i32, rows : i32) -> Pong {
        Pong {
            rows : rows,
            columns : columns,
            score_a : 0,
            score_b : 0,
            racket_a : Pieze { size: 2, x: 0, y: (rows-1)/2 },
            racket_b : Pieze { size: 2, x: columns-1, y: (rows-1)/2 },
            ball : Pieze { size: 2, x: (columns-1)/2, y: (rows-1)/2 },
            ball_dir_x : -1,
            ball_dir_y :  1,
        }
    }

    pub fn move_racket(&self, mut racket : Pieze, mov : Option<RacketMov>) -> Pieze {
        racket.y = match mov {
            Some(RacketMov::Up) => cmp::max(racket.y - 1, 0),
            Some(RacketMov::Down) => cmp::min(racket.y + 1, self.rows - racket.size),
            None => racket.y,
        };

        racket
    }

    pub fn tick(&mut self, racket_a_mov : Option<RacketMov>, racket_b_mov : Option<RacketMov>) {
        self.racket_a = self.move_racket(self.racket_a.clone(), racket_a_mov); 
        self.racket_b = self.move_racket(self.racket_b.clone(), racket_b_mov); 

        let ball = &self.ball;

        let hits_vertical_wall : bool = ball.y + self.ball_dir_y < 0 || ball.y + ball.size-1 + self.ball_dir_y >= self.rows;
        if hits_vertical_wall {
            self.ball_dir_y = -self.ball_dir_y;
        }

        /* Generalize by 2D collision detection? */
        let racket_a = &self.racket_a;
        let racket_b = &self.racket_b;
        let hits_racket_a : bool = ball.x + self.ball_dir_x <= racket_a.x && ball.y + ball.size-1 + self.ball_dir_y >= racket_a.y && ball.y + self.ball_dir_y <= racket_a.y + racket_a.size-1;
        let hits_racket_b : bool = ball.x + ball.size-1 + self.ball_dir_x >= self.racket_b.x && ball.y + ball.size-1 + self.ball_dir_y >= racket_b.y && ball.y + self.ball_dir_y <= racket_b.y + racket_b.size-1;
        let hits_wall_a : bool = ball.x + self.ball_dir_x < 0;
        let hits_wall_b : bool = ball.x + ball.size-1 + self.ball_dir_x >= self.columns;
        if  hits_racket_a || hits_racket_b || hits_wall_a || hits_wall_b {
            self.ball_dir_x = -self.ball_dir_x;
        }
        
        if hits_wall_a || hits_wall_b {
            if hits_wall_a {
                self.score_b += 1;
            }
            if hits_wall_b {
                self.score_a += 1;
            }
            self.ball.x = (self.columns-1)/2;
            self.ball.y = (self.rows-1)/2;
        }
        else {
            self.ball.x += self.ball_dir_x;
            self.ball.y += self.ball_dir_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounces() {
        let mut game : Pong = Pong::build(8, 6);
        assert!(game.ball.size == 2);
        assert!(game.ball.x == 3);
        assert!(game.ball.y == 2);

        assert!(game.racket_a.size == 2);
        assert!(game.racket_a.x == 0);
        assert!(game.racket_a.y == 2);

        assert!(game.racket_b.size == 2);
        assert!(game.racket_b.x == 7);
        assert!(game.racket_b.y == 2);

        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        assert!(game.ball.x == 2);
        assert!(game.ball.y == 3);

        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        assert!(game.ball.x == 4);
        assert!(game.ball.y == 1);
    }

    #[test]
    fn scores() {
        let mut game : Pong = Pong::build(10, 8);
        assert!(game.ball.size == 2);
        assert!(game.ball.x == 4);
        assert!(game.ball.y == 3);

        assert!(game.racket_a.size == 2);
        assert!(game.racket_a.x == 0);
        assert!(game.racket_a.y == 3);

        assert!(game.racket_b.size == 2);
        assert!(game.racket_b.x == 9);
        assert!(game.racket_b.y == 3);

        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        assert!(game.ball.x == 0);
        assert!(game.ball.y == 5);
        game.tick(None, None);
        assert!(game.score_a == 0);
        assert!(game.score_b == 1);
        assert!(game.ball.x == 4);
        assert!(game.ball.y == 3);
        
        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        game.tick(None, None);
        assert!(game.ball.x == 8);
        assert!(game.ball.y == 1);
        game.tick(None, None);
        assert!(game.score_a == 1);
        assert!(game.score_b == 1);
        assert!(game.ball.x == 4);
        assert!(game.ball.y == 3);
    }

    #[test]
    fn moves_rackets() {
        let mut game : Pong = Pong::build(10, 8);
        assert!(game.racket_a.size == 2);
        assert!(game.racket_a.x == 0);
        assert!(game.racket_a.y == 3);

        assert!(game.racket_b.size == 2);
        assert!(game.racket_b.x == 9);
        assert!(game.racket_b.y == 3);

        game.tick(Some(RacketMov::Up), None);
        assert!(game.racket_a.x == 0);
        assert!(game.racket_a.y == 2);
        assert!(game.racket_b.x == 9);
        assert!(game.racket_b.y == 3);

        game.tick(Some(RacketMov::Up), Some(RacketMov::Down));
        assert!(game.racket_a.x == 0);
        assert!(game.racket_a.y == 1);
        assert!(game.racket_b.x == 9);
        assert!(game.racket_b.y == 4);

        game.tick(Some(RacketMov::Up), Some(RacketMov::Down));
        game.tick(Some(RacketMov::Up), Some(RacketMov::Down));
        game.tick(Some(RacketMov::Up), Some(RacketMov::Down));
        game.tick(Some(RacketMov::Up), Some(RacketMov::Down));
        assert!(game.racket_a.x == 0);
        assert!(game.racket_a.y == 0);
        assert!(game.racket_b.x == 9);
        assert!(game.racket_b.y == 6);
    }
}