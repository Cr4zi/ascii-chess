#[derive(Debug, Clone, PartialEq)]
pub enum Color{
    Black,
    White,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pawn<'a>{
    pub pos: &'a str,
    pub color: Color,
}

pub trait Draw<'a> {
    fn draw(self: &'_ Self) -> Result<String, &str>;
}

impl Draw<'_> for Pawn<'static> {
    fn draw(self: &'_ Self) -> Result<String, &str>{
        let emoji = match self.color {
            Color::Black => Ok(String::from("♙")),
            Color::White => Ok(String::from("♟")),
        };
        emoji
    }
}
