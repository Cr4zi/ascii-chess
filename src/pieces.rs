enum Color{
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn{
    pos: String,
    color: Color,
}

pub trait Draw {
    fn draw(self: &'_ Self) -> Result<String, &str> {}
}

impl Draw for Pawn {
    fn draw(self: &'_ Self) -> Result<String, &str>{
        let emoji = match self.color {
            Color::Black => Ok(String::from("♟")),
            Color::White => Ok(String::from("♙")),
            _ => Err("You need to define a color")

        };
        emoji
    }
}
