fn main() {
    println!("Hello, world!");
}

struct Pieze {
    size : i32,
    x : i32,
    y : i32,
}

struct Pong {
    ball : Pieze,
    add_x : i32,
    add_y : i32,
}

impl Pong {
    fn build(columns : i32, rows : i32) -> Pong {
        Pong {
            ball : Pieze { size: 2, x: (columns-1)/2, y: (rows-1)/2 },
            add_x : -1,
            add_y :  1,
        }
    }

    fn tick(&mut self) {
        if self.ball.x + self.add_x < 0 || self.ball.x + self.ball.size-1 + self.add_x >= 4 {
            self.add_x = -self.add_x;
        }
        if self.ball.y + self.add_y < 0 || self.ball.y + self.ball.size-1 + self.add_y >= 4 {
            self.add_y = -self.add_y;
        }

        self.ball.x += self.add_x;
        self.ball.y += self.add_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounces() {
        let mut game : Pong = Pong::build(4, 4);
        assert!(game.ball.size == 2);
        assert!(game.ball.x == 1);
        assert!(game.ball.y == 1);

        game.tick();
        assert!(game.ball.x == 0);
        assert!(game.ball.y == 2);

        game.tick();
        assert!(game.ball.x == 1);
        assert!(game.ball.y == 1);

        game.tick();
        assert!(game.ball.x == 2);
        assert!(game.ball.y == 0);

        game.tick();
        assert!(game.ball.x == 1);
        assert!(game.ball.y == 1);
    }
}