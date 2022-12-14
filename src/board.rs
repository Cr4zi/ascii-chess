#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub pos: String,
    pub empty: bool,
}

fn draw_raw(first_raw: bool) {
    if first_raw {
        for _ in 1..(8*2){
            print!("-");
        }
        println!("");
    }
    for _ in 1..9 {
        print!("| ");
    }
    println!("");
    for _ in 1..(8*2){
        print!("-");
    }
    println!("");

}

pub fn draw_board() {
    draw_raw(true);
    for i in 0..7 {
        draw_raw(false);
    }
}
