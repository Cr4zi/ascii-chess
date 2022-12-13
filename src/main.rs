use crate::pieces::Draw;

mod pieces;
mod board;

fn main() {
    let black_pawn = pieces::Pawn { pos: "e10", color: pieces::Color::Black };
    let white_pawn = pieces::Pawn { pos: "e10", color: pieces::Color::White};

    println!("black pawn: {:#?}", {&black_pawn.draw().unwrap()});
    println!("white pawn: {:#?}", {&white_pawn.draw().unwrap()});
}
