use std::io;
use std::io::Write;

use console::Term;

use crate::Position;

const COLORS: [(u8, u8, u8); 4] = [
    (45, 42, 59),
    (145, 126, 156),
    (125, 111, 134),
    (209, 147, 150),
];

const TAKO: [&[u8; 21]; 17] = [
    b"__111___11111___111__",
    b"_12331_1333331_13331_",
    b"_1233313333333133331_",
    b"__12333333333333331__",
    b"___133333333333331___",
    b"___133333333333331___",
    b"___133311333113331___",
    b"__12311333333311331__",
    b"__12333333333333331__",
    b"__12343133133134331__",
    b"__12333311311333331__",
    b"__12333333333333331__",
    b"_1233333333333333331_",
    b"_1233333333333333331_",
    b"123333333333333333331",
    b"123311333111333113331",
    b"_111__111___111__111_",
];

pub fn draw_tako(term: &mut Term, orig: Position) -> io::Result<()> {
    for (i, line) in TAKO.iter().enumerate() {
        term.move_cursor_to(orig.col, orig.row + i)?;
        for char in line.iter() {
            let color = match char {
                0x31 => COLORS[0],
                0x32 => COLORS[1],
                0x33 => COLORS[2],
                0x34 => COLORS[3],
                _ => {
                    term.move_cursor_right(2)?;
                    continue;
                }
            };

            write!(term, "\x1b[48;2;{};{};{}m  ", color.0, color.1, color.2)?;
        }
    }

    Ok(())
}
