use crate::pieces::Draw;

mod pieces;
mod board;

fn main() {
    let black_pawn = pieces::Pawn { pos: "e10", color: pieces::Color::Black };
    let white_pawn = pieces::Pawn { pos: "e10", color: pieces::Color::White};

    println!("black pawn: img {}, pos {}", {&black_pawn.draw().unwrap()}, {&black_pawn.pos});
println!("white pawn: img {}, pos {}", {&white_pawn.draw().unwrap()}, {&white_pawn.pos});
}
