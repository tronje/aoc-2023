use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    pub fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn with_red(mut self, red: u32) -> Self {
        self.red = red;
        self
    }

    pub fn with_green(mut self, green: u32) -> Self {
        self.green = green;
        self
    }

    pub fn with_blue(mut self, blue: u32) -> Self {
        self.blue = blue;
        self
    }

    fn total(&self) -> u32 {
        self.red + self.green + self.blue
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    revealed_subsets: Vec<CubeSet>,
}

impl Game {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_valid(&self, config: &CubeSet) -> bool {
        for subset in self.revealed_subsets.iter() {
            if subset.total() > config.total() {
                return false;
            }

            if subset.red > config.red {
                return false;
            }

            if subset.green > config.green {
                return false;
            }

            if subset.blue > config.blue {
                return false;
            }
        }

        true
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use parser::*;

        let mut buf = String::with_capacity(s.len());

        let mut game = Self {
            id: 0,
            revealed_subsets: Vec::new(),
        };

        let mut last_char_was_digit = false;
        let mut last_token = Token::Start;
        let mut current_set = CubeSet::empty();

        for c in s.chars() {
            // if inside a number, keep adding digits
            if c.is_ascii_digit() {
                last_char_was_digit = true;
                buf.push(c);
                continue;
            }

            if !last_char_was_digit && !c.is_whitespace() {
                buf.push(c);
            }

            last_char_was_digit = false;

            if let Ok(token) = parser::Token::from_str(&buf) {
                match (last_token, token) {
                    (Token::Game, Token::Number(n)) => {
                        game.id = n;
                    }

                    (Token::Number(n), Token::Color(c)) => match c {
                        Color::Red => current_set.red = n,
                        Color::Green => current_set.green = n,
                        Color::Blue => current_set.blue = n,
                    },

                    (_, Token::Semicolon) => {
                        game.revealed_subsets.push(current_set);
                        current_set = CubeSet::empty();
                    }

                    (_, _) => {}
                }

                last_token = token;
                buf.clear();
            }
        }

        game.revealed_subsets.push(current_set);

        Ok(game)
    }
}

mod parser {
    use std::str::FromStr;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Token {
        Start,
        Game,
        Colon,
        Comma,
        Semicolon,
        Color(Color),
        Number(u32),
    }

    impl FromStr for Token {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Game" => Ok(Self::Game),
                ":" => Ok(Self::Colon),
                "," => Ok(Self::Comma),
                ";" => Ok(Self::Semicolon),
                "red" => Ok(Self::Color(Color::Red)),
                "green" => Ok(Self::Color(Color::Green)),
                "blue" => Ok(Self::Color(Color::Blue)),
                other => {
                    let n = u32::from_str(other)?;
                    Ok(Self::Number(n))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_tests_part_one() {
        let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_str(test).unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.revealed_subsets.len(), 3);

        assert_eq!(
            game.revealed_subsets[0],
            CubeSet {
                red: 4,
                green: 0,
                blue: 3
            }
        );

        assert_eq!(
            game.revealed_subsets[1],
            CubeSet {
                red: 1,
                green: 2,
                blue: 6,
            }
        );

        assert_eq!(
            game.revealed_subsets[2],
            CubeSet {
                red: 0,
                green: 2,
                blue: 0,
            }
        );
    }
}
