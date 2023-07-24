use std::{error::Error, fs::File, io::Write};

/// Pack color as 4-byte u32.
fn pack_color(r: u8, g: u8, b: u8, a: Option<u8>) -> u32 {
    let a: u8 = a.unwrap_or(255);
    // rgba is stored in a u32
    (r as u32) + ((g as u32) << 8) + ((b as u32) << 16) + ((a as u32) << 24)
}

fn unpack_color(color: &u32, r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
    // Modify the r channel.
    // Extract the color at each channel by:
    // * Right-shift bits by 1 byte chunks to get color channel as u8.
    // * Update rgb values.
    *r = (*color & 255) as u8;
    *g = ((*color >> 8) & 255) as u8;
    *b = ((*color >> 16) & 255) as u8;
    *a = ((*color >> 24) & 255) as u8;
}

/// Write PPM file.
/// https://netpbm.sourceforge.net/doc/ppm.html
fn drop_ppm_image(
    fname: &str,
    image: &[u32],
    w: usize,
    h: usize,
) -> Result<(), Box<dyn Error>> {
    // Check images is correct size as given width and height.
    assert_eq!(image.len(), w * h);
    let mut fh = File::create(fname)?;
    // Write magic number identifying file type, w, h, max color value. All delimited by newline.
    write!(&mut fh, "P6\n{w} {h}\n255\n")?;

    for px in image.iter().take(h * w) {
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
        // Update rgba
        unpack_color(px, &mut r, &mut g, &mut b, &mut a);
        write!(&mut fh, "{r}{g}{b}")?;
    }

    Ok(())
}

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    // Store image in 1D array.
    // Access elems by specify w + (h * WIDTH)
    let mut framebuffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Iterate through window pixels and fill with color gradient.
    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            // Vary the red channel between 0-255 as h sweeps vertical.
            let r: u8 = (255 * h / HEIGHT) as u8;
            // Vary the green channel between 0-255 as w sweeps horizontal.
            let g: u8 = (255 * w / WIDTH) as u8;
            let b: u8 = 0;
            // Access index of one-dim array.
            framebuffer[w + h * WIDTH] = pack_color(r, g, b, None)
        }
    }
    drop_ppm_image("./out.ppm", &framebuffer, WIDTH, HEIGHT).unwrap();
}
