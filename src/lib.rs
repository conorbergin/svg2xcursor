use resvg::{tiny_skia, usvg};


use std::{io,env,fs};

pub struct XCursor {
    pub name: &'static str,
    pub cursor: &'static str,
    pub symlinks: Vec<&'static str>
}





pub fn write_theme(name: &str, inherits: &str, cursors: Vec<XCursor>) -> io::Result<()> {

    fs::create_dir("output")?;
    env::set_current_dir("output")?;
    fs::create_dir(name)?;
    env::set_current_dir(name)?;
    fs::write("index.theme", format!("[icon theme]\nName={}\nInherits={}\n",name,inherits))?;
    fs::create_dir("cursors")?;
    env::set_current_dir("cursors")?;

    for c in cursors {
        fs::write(c.name, generate_xcursor_binary(c.cursor)?)?;
        for s in c.symlinks {
            std::os::unix::fs::symlink(c.name, s)?;
        }
    }
   
   Ok(())
}

pub fn generate_xcursor_binary(svg: &str) -> std::io::Result<Vec<u8>> {
    const MAGIC: &[u8] = b"Xcur";
    const XCURSOR_HEADER_SIZE: u32 = 4 * 4;
    const XCURSOR_FILE_VERSION: u32 = 65536;

    const CURSOR_SIZES: &[u32] = &[16, 24, 32];
    const N_ENTRIES: u32 = CURSOR_SIZES.len() as u32;
    const TOC_SIZE: u32 = N_ENTRIES * 3 * 4;

    let mut buffer = <Vec<u8>>::new();

    // read svg
    let mut tree = {
        let mut opt = usvg::Options::default();
        let mut fontdb = usvg::fontdb::Database::new();
        fontdb.load_system_fonts();

        usvg::Tree::from_data(&svg.as_bytes(), &opt).unwrap()
    };
    tree.calculate_bounding_boxes();
    let bb = tree.root.bounding_box.unwrap();
    let x_offset = bb.left().abs() / bb.width();
    let y_offset = bb.top().abs() / bb.height();

    // Write header
    buffer.extend(MAGIC); // magic bytes
    buffer.extend(
        [XCURSOR_HEADER_SIZE, XCURSOR_FILE_VERSION, N_ENTRIES]
            .iter()
            .flat_map(|x| x.to_le_bytes()),
    );

    // Write toc
    {
        let mut acc = 0;
        for &s in CURSOR_SIZES {
            let toc = [
                0xfffd0002_u32, // type descriptor for cursor (comment also possible)
                s,              // nominal size, we use the actual size
                XCURSOR_HEADER_SIZE + TOC_SIZE + acc, // position in the file
            ];

            acc += s * s * 4 + 36; // size of all images so far

            buffer.extend(toc.iter().flat_map(|x| x.to_le_bytes()));
        }
    }

    // Write cursor images

    for &s in CURSOR_SIZES {
        // println!("{:?}", tree.root);

        let mut pixmap = tiny_skia::Pixmap::new(s, s).unwrap();

        let scale_x = s as f32 / tree.size.width();
        let scale_y = s as f32 / tree.size.height();

        resvg::render(
            &tree,
            usvg::Transform::from_scale(scale_x, scale_y),
            &mut pixmap.as_mut(),
        );

        let hot_x = (x_offset * s as f32) as u32;
        let hot_y = (y_offset * s as f32) as u32;

        let xcursor_image_header: [u32; 9] = [
            36,         // header size
            0xfffd0002, // image type
            s,          // nominal size
            1,          // version
            s,          // image width
            s,          // image height
            hot_x,      // x origin
            hot_y,      // y origin
            0,          // delay
        ];

        buffer.extend(xcursor_image_header.iter().flat_map(|x| x.to_le_bytes()));

        let argb_data = pixmap
            .pixels()
            .iter()
            .flat_map(|p| {
                // little endian argb
                [p.blue(), p.green(), p.red(), p.alpha()]
            })
            .collect::<Vec<u8>>();

        buffer.extend(&argb_data);
        // Write cursor
    }

    Ok(buffer)
}
