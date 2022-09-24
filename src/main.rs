use std::fmt::Write;
use std::io::Cursor;
use freetype::{Library, face::LoadFlag};
use image::{ImageBuffer, RgbImage, Rgb};
mod sdffont;

// https://freetype.org/freetype2/docs/glyphs/glyphs-3.html

fn main() {

    let lib = Library::init().unwrap();

    let face = lib.new_face("E:/repos/rust-sdf-tool/test_fonts/calibri.ttf",0).unwrap();

    //face.set_char_size(40 * 64, 0, 50, 0).unwrap();

    let pixel_w = 32;
    let pixel_h = 32;
    face.set_pixel_sizes(pixel_w, pixel_h).unwrap();

    face.load_char('+' as usize, LoadFlag::RENDER).unwrap();

    let glyph = face.glyph();


    let metrics = face.size_metrics().unwrap();
    println!("{:#?}", metrics);

    let x_scale = metrics.x_scale as f64 / (65536.0);
    let y_scale = metrics.x_scale as f64 / (65536.0);

    println!("scale: {:?}", (x_scale, y_scale));

    let x_scale = metrics.x_scale as f64 / (65536.0);
    let y_scale = metrics.x_scale as f64 / (65536.0);

    println!("scale: {:?}", (x_scale, y_scale));


    println!("MAV {:?}", metrics.max_advance >> 6);

    let max_descent = metrics.descender >> 6;
    let max_ascent = metrics.ascender >> 6;

    let face_info = FaceInfo {
        pixel_h: ( max_ascent + i32::abs(max_descent)) as  u32,
        pixel_w
    };


    let img = create_raster_img(glyph, face_info);

    img.save("./A.png");

}


#[derive(Clone, Copy, Debug)]
struct FaceInfo {
    pixel_h: u32,
    pixel_w: u32
}

#[derive(Clone, Copy, Debug)]
struct CharInfo {
    chr: u32,
    advance_x: i32,
    advance_y: i32,
    padding_x: i32,
    padding_y: i32

}

fn create_raster_img(glyph: &freetype::GlyphSlot, face_info: FaceInfo) -> RgbImage {

    let bitmap = glyph.bitmap();

    let g = glyph.get_glyph().unwrap();
    let g_metrics = glyph.metrics();
    println!("{:#?}", g_metrics);

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
