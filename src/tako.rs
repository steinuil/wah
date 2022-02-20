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

pub const TAKODACHI_WIDTH: usize = 42;
pub const TAKODACHI_HEIGHT: usize = 16;

const TAKO: [&[u8; TAKODACHI_WIDTH / 2]; TAKODACHI_HEIGHT] = [
    b"__###____###____###__",
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

pub fn draw_takodachi(term: &mut Term, orig: Position) -> io::Result<()> {
    for (i, line) in TAKO.iter().enumerate() {
        term.move_cursor_to(orig.col, orig.row + i)?;
        for char in line.iter() {
            let color = match char {
                0x23 => COLOR_OUTLINE,
                0x2D => COLOR_HIGHLIGHTS,
                0x3D => COLOR_FILL,
                0x2A => COLOR_BLUSH,
                _ => {
                    term.move_cursor_right(2)?;
                    continue;
                }
            };

            write!(term, "\x1b[48;2;{};{};{}m  ", color.r, color.g, color.b)?;
        }
    }

    Ok(())
}
