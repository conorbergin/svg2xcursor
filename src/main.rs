mod lib;

use std::io;
use crate::lib::{write_theme,XCursor};

fn main() -> io::Result<()> {
    // Read the SVG file
    let text = include_str!("svg/text.svg");
    let default = include_str!("svg/default.svg");

    write_theme(
        "ExampleTheme",
        "Adwaita",
        vec![
            XCursor {
                name: "text",
                cursor: text,
                symlinks: vec!["xterm"]
            },
            XCursor {
                name: "default",
                cursor: default,
                symlinks: vec!["left_ptr"]
            }]
    )?;
    
    Ok(())
}
