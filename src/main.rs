use std::fmt::Write;
use std::io::Cursor;
use freetype::{Library, face::LoadFlag};
use image::{ImageBuffer, RgbImage, Rgb};
mod sdffont;

// https://freetype.org/freetype2/docs/glyphs/glyphs-3.html

fn main() {

    let lib = Library::init().unwrap();
    let face = lib.new_face("E:/repos/rust-sdf-tool/test_fonts/calibri.ttf",0).unwrap();


    let sdf_chr_info = generate_sdf_info_char('ø', &face, 32);
    sdf_chr_info.img.save("sdf.png");

    let lineheight = face.height() >> 6;
    println!("Line H{:?}", lineheight);

    //let img = create_raster_img_with_baseline('\u{43}', &face, 128);
    //img.save("A.png");


}


fn generate_sdf_info_char(chr: char, face: &freetype::Face, font_size: u32)  -> CharInfo {

    let mut s = "".to_string();

    //std::io::stdin().read_line(&mut s).expect("Rad line");

    let upscale_res = 32;
    // padding on each of the 4 sides
    let padding = (0.125 * upscale_res as f64) as u32;  // this will result in padding of 4 px in 32 px font size on each side

    let spread = upscale_res / 4;

    face.set_pixel_sizes(upscale_res, upscale_res).unwrap();
    face.load_char(chr as usize, LoadFlag::RENDER).unwrap();

    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    let g = glyph.get_glyph().unwrap();

    let rows = bitmap.rows() as u32;
    let width = bitmap.width() as u32;
    let bitmap_buffer = bitmap.buffer();

    println!("Size: {:?}", (width, rows));

    // buffer with padding
    let sdf_h = rows + padding * 2;
    let sdf_w = width + padding * 2;


    let mut sdf_buffer : Vec::<f64> = vec![0.0; (sdf_w * sdf_h)as usize];

    for y in 0..rows {
        for x in 0..width {
            let val = bitmap_buffer[(y * width + x) as usize];
            sdf_buffer[((y + padding) * sdf_w + x +  padding) as usize] = val as f64;
        }
    }

    let scale_x = width as f64 / upscale_res as f64;
    let scale_y = rows as f64 / upscale_res as f64;

    let out_w = (scale_x * font_size as f64).round() as u32;
    let out_h = (scale_y * font_size as f64).round() as u32;

    println!("Output size{:?}",(out_w, out_h));



    let mut tmp_img: RgbImage = ImageBuffer::new(width, rows);

    for y in 0..tmp_img.height() {
        for x in 0..tmp_img.width() {
            let val = bitmap_buffer[(y * width + x) as usize];
            tmp_img.put_pixel(x,y,  Rgb([val, val, val]));
        }
    }

    tmp_img.save("Tmp.png");

    let mut img: RgbImage = ImageBuffer::new(width + padding * 2, rows + padding * 2);

    let mut max_d = 0.0;
    for y in 0..img.height() {
        for x in 0..img.width() {

            let px_v = 0.0 ;

            let px_v = px_value(bitmap_buffer, width as i32, rows as i32, padding as i32, x as i32, y as i32, spread as i32);

            let mut val = (px_v * 255.0) as u8;



            img.put_pixel(x, y, Rgb([val, val, val]));
        }
    }
    //let px_v = px_value(bitmap_buffer, width as i32, rows as i32, padding as i32, 31, 31, spread as i32);


    println!("{:?}", (img.width(), img.height(), padding));

    CharInfo {
        chr: chr as u32,
        advance_x: glyph.advance().x >> 6,
        advance_y: 0, // also not used by text renderer. Is used when align horizontal
        padding_x: padding as i32,
        padding_y: padding as i32,
        img
    }
}


// find min dist to a border pixel
// return in range 0.0..1.0 where 0.5 border
fn px_value(buffer: &[u8], buf_w: i32, buf_h: i32, padding: i32, sdf_x: i32, sdf_y: i32, spread: i32) -> f64 {

    let outside = sdf_x < padding || sdf_x >= buf_w + padding || sdf_y < padding || sdf_y >= buf_h + padding;

    // set state (inside or out)
    let mut state = 0;
    let state_idx = ((sdf_y - padding ) * buf_w + sdf_x - padding) as usize;
    if !outside {

        if buffer[state_idx] > 0 {
            state = 1;
        };
        assert!(state == 1 || state == 0);
    }


    let x_start = sdf_x - spread;
    let x_end = sdf_x + spread;

    let y_start = sdf_y - spread;
    let y_end = sdf_y + spread;

    let mut min_dist_squared = spread * spread;

    let max_dist = f64::sqrt(min_dist_squared as f64);
    // iterate only over pixels in buffer, since all the others outside will be further away

    // x and y is index into the final sdf image
    for y in y_start..y_end {
        for x in x_start..x_end {

            // map to x and y in buffer. and if outside skip
            let buf_x = x - padding;
            let buf_y = y - padding;

            if buf_x < 0 || buf_x >= buf_w || buf_y < 0 || buf_y >= buf_h {
                continue;
            }

            let buffer_v = buffer[(buf_y * buf_w + buf_x) as usize].max(0).min(1);

            if buffer_v != state {
                let x_diff = sdf_x - x;
                let y_diff = sdf_y - y;

                let before = min_dist_squared;
                min_dist_squared = i32::min(min_dist_squared, x_diff * x_diff + y_diff * y_diff);

                if min_dist_squared < before {
                    //println!("{:?}", (x_diff, y_diff, min_dist_squared));
                                      //, sdf_y, buffer_v, state, buf_x, buf_y, padding, buffer[(buf_y * buf_w + buf_x) as usize], (buf_y * buf_w + buf_x) as usize));
                }
            }

        }
    }

    let min_dist = f64::sqrt(min_dist_squared as f64);

    assert!(min_dist <= max_dist);

    // outside become negative inside positive
    let mut mul = 1.0;
    if state == 0 {
        mul = -1.0
    }


    let scaled = (min_dist / max_dist) * mul;

    // map from [-1.0..1.0] to [-0.5..0.5] and + 0.5 to be in [0.0..1.0]
    let mut res = scaled / 2.0 + 0.5;
    assert!((state == 1 && res > 0.5) || (state == 0 && res <= 0.5));
    //println!("{:?}", res);
    res

}





#[derive(Clone, Copy, Debug)]
struct FaceInfo {
    pixel_h: u32,
    pixel_w: u32
}

#[derive(Clone, Debug)]
struct CharInfo {
    chr: u32,
    advance_x: i32,
    advance_y: i32,
    padding_x: i32,
    padding_y: i32,
    img: RgbImage
}

fn create_raster_img_with_baseline(chr: char, face: &freetype::Face, font_size: u32) -> RgbImage {

    face.set_pixel_sizes(font_size, font_size).unwrap();
    println!("{:?}", chr);
    face.load_char(chr as usize, LoadFlag::RENDER).unwrap();

    let glyph = face.glyph();

    let metrics = face.size_metrics().unwrap();
    let max_descent = metrics.descender >> 6;
    let max_ascent = metrics.ascender >> 6;

    let face_info = FaceInfo {
        pixel_h: ( max_ascent + i32::abs(max_descent)) as  u32,
        pixel_w: font_size
    };

    let bitmap = glyph.bitmap();

    let g = glyph.get_glyph().unwrap();
    let g_metrics = glyph.metrics();
    println!("{:#?}", g_metrics);

    let rows = bitmap.rows();
    let width = bitmap.width();
    let buffer = bitmap.buffer();

    let mut img: RgbImage = ImageBuffer::new(face_info.pixel_w, face_info.pixel_h);

    let rows = bitmap.rows();
    let width = bitmap.width();
    let buffer = bitmap.buffer();


    // >> 6 to divide by 64. since they are in 26.6 fractional format

    let left = g_metrics.horiAdvance >> 6;
    let g_width = g_metrics.width >> 6;


    let advance_x = glyph.advance().x >> 6;

    let advance_y = glyph.advance().y >> 6;

    let x_offset = (face_info.pixel_w as i32 - advance_x) / 2;


    let bearing_y = g_metrics.horiBearingY >> 6;
    let y_offset =  (face_info.pixel_h as i32 - bearing_y) /2;

    println!("{:?}", (x_offset, y_offset));

    println!("H  = {:?}",g_metrics.height >> 6);

    // Draw baseline, relative to char
    let baseline_h = (y_offset + bearing_y) as u32;
    for y in 0..face_info.pixel_h {
        for x in 0..face_info.pixel_w {
            if y == baseline_h {
                img.put_pixel(x as u32, y as u32, Rgb([255, 0, 255]));
            }
        }
    }

    for y in 0..rows {
        for x in 0..width {

            let val = buffer[(y * width + x) as usize];
            let y_index = y_offset + y as i32;
            let x_index = x_offset + x;

            if val > 0 {
                img.put_pixel(x_index as u32, y_index as u32, Rgb([val, val, val]));
                //img.put_pixel(x_index as u32, y as u32, Rgb([val, val, val]));
            }
        }
    }

    img
}


fn create_sdf(img: RgbImage, info: CharInfo ){

}
fn create_font(face: freetype::Face) {



}





#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_px_value() {

        let mut buffer = [1; 8*8];

        for i in 0..8 {
            buffer[i] = 0;
            buffer[i+8] = 0;
        }

        let padding = 4;
        let spread = 8;

        let x = 4;
        let y = 4;
        let v = px_value(&buffer, 8, 8, padding, padding + x, padding + y, spread);

        println!("{:?}", v);


        assert!(false);



    }

}
