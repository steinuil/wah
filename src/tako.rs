use std::io::{self, Write};

use console::Term;

use crate::Position;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

const COLOR_OUTLINE: Color = Color {
    r: 45,
    g: 42,
    b: 59,
};

const COLOR_HIGHLIGHTS: Color = Color {
    r: 145,
    g: 126,
    b: 156,
};

const COLOR_FILL: Color = Color {
    r: 125,
    g: 111,
    b: 134,
};

const COLOR_BLUSH: Color = Color {
    r: 209,
    g: 147,
    b: 150,
};

const COLOR_HALO: Color = Color {
    r: 230,
    g: 169,
    b: 115,
};

pub const TAKODACHI_WIDTH: usize = 42;
pub const TAKODACHI_HEIGHT: usize = 18;

const TAKO: [&[u8; TAKODACHI_WIDTH / 2]; TAKODACHI_HEIGHT] = [
    b"_______+++++++_______",
    b"______+_______+______",
    b"__###__++###++__###__",
    b"_#-==#_##===##_#===#_",
    b"_#-===#=======#====#_",
    b"__#-====-=========#__",
    b"___#-==-=========#___",
    b"___#-============#___",
    b"___#===##===##===#___",
    b"__#-=##=======##==#__",
    b"__#-==============#__",
    b"__#-=*=#==#==#=*==#__",
    b"_#-=====##=##======#_",
    b"_#-================#_",
    b"#-==================#",
    b"#-==================#",
    b"_#==##===###===##==#_",
    b"__##__###___###__##__",
];

fn char_to_color(char: u8) -> Option<Color> {
    match char {
        b'#' => Some(COLOR_OUTLINE),
        b'-' => Some(COLOR_HIGHLIGHTS),
        b'=' => Some(COLOR_FILL),
        b'*' => Some(COLOR_BLUSH),
        b'+' => Some(COLOR_HALO),
        _ => None,
    }
}

pub fn draw_takodachi(term: &mut Term, orig: Position) -> io::Result<()> {
    for (i, line) in TAKO.iter().enumerate() {
        term.move_cursor_to(orig.col, orig.row + i)?;
        for char in line.iter() {
            match char_to_color(*char) {
                Some(color) => {
                    write!(term, "\x1b[48;2;{};{};{}m  ", color.r, color.g, color.b)?;
                }
                None => {
                    term.move_cursor_right(2)?;
                }
            }
        }
    }

    Ok(())
}

pub fn print_takodachi() {
    for line in TAKO {
        let line = line
            .iter()
            .map(|char| {
                char_to_color(*char).map_or_else(
                    || "\x1b[0m  ".to_string(),
                    |color| format!("\x1b[48;2;{};{};{}m  ", color.r, color.g, color.b),
                )
            })
            .collect::<String>();
        println!("{}\x1b[0m", line);
    }
}
