# SVG 2 XCursor

This is a utility for building XCursor themes from SVG strings, the themes will be place in a folder named output.

Clone the repo and `cargo run` to build the example theme. You can then define you own themes either directly in rust or by using the `include_str!` macro.

A theme is defined as a list of XCursors, a theme name, and a them that you inherit from. The theme that you inherit from will act as a fallback for any XCursors you don't define, i.e. 'Adwaita'.

Each XCursor definition has a name, an SVG, and a list of dependencies. These dependencies are other cursors which use the same image, i.e. you will probably want the `default` and `left_ptr` cursor to point to the same image, this is done by making a symlink to the original XCursor file.

The names of cursors are not well documented, the best thing to do ids to find some existing XCursor themes and see if they work well on your desktop, then see what XCursor file names they use and which symlinks point to them.
