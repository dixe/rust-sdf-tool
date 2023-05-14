use crate::*;
use std::io::prelude::*;
use std::io::Write;
use image::imageops;

pub fn write_font_files(face: &freetype::Face, gen_info: GenInfo, chars: Vec::<(CharInfo, RgbaImage)> ) {

    let page_size = 512;

    // layout chars into pages and page images. Return

    let scale = gen_info.upscale_res as f32 / (face.em_size() >> 6 ) as f32;

    let lineheight = ((face.height() >> 6 ) as f32 * scale) as u32 ;

    let pages = layout_chars(face, chars, page_size, lineheight, gen_info.upscale_res);

    for p in &pages {
        let mut out_img = p.image.clone();
        //out_img = imageops::flip_vertical(&p.image);
        out_img.save(p.file.clone());
    }

    let info_line = info_string(&face, gen_info);

    let common_line = common_line_string(lineheight, pages.len());

    let pages_lines = pages_string(&pages);

    let output = info_line + &common_line + &pages_lines;

    let mut file = std::fs::File::create(format!("{}_{}.fnt", face.family_name().unwrap(), gen_info.upscale_res)).unwrap();
    file.write_all(output.as_bytes()).unwrap();
    file.flush();
}


fn layout_chars(face: &freetype::Face, chars: Vec::<(CharInfo, RgbaImage)>, page_size: u32, lineheight: u32, pixel_size: u32) -> Vec::<Page> {

    let mut page_id = 0;
    let mut res =  vec![];
    let page_file_name = format!("{}_{}_{}.png", face.family_name().unwrap(), page_id, pixel_size);
    let mut char_infos = vec![];

    let mut res_img: RgbaImage = ImageBuffer::new(page_size, page_size);

    // try to layout all chars into current images alternative figure out image size and then use that
    let mut x = 0;
    let mut y = 0;
    for (chr_info, img) in chars {
        let w = img.width();
        let h = img.height();

        // check if we should go to new line
        if x + w > page_size {
            x = 0;
            y += lineheight;
        }


        let (x_new, y_new) = insert_chr_img(x, y, &mut res_img, &img);

        // insert char info to page chars including info about x and y
        char_infos.push( FontCharInfo {
            id: chr_info.chr,
            x, // with padding
            y, // with padding
            width: chr_info.width,
            height: chr_info.height,
            xoffset: chr_info.offset_x,
            yoffset: - chr_info.offset_y + lineheight as i32,
            xadvance: chr_info.advance_x,
            page: page_id,
            chnl: 0,
        });

        x = x_new;
        y = y_new;
    }



    let page = Page {
        id: page_id,
        file: page_file_name,
        chars: char_infos,
        kernings: vec![],
        image: res_img
    };


    res.push(page);
    res
}


fn insert_chr_img(x: u32, y: u32, res_img: &mut RgbaImage, img: &RgbaImage) -> (u32, u32) {
    // assume that x + img.width < res_img.width, same with y and height


    for img_y in 0..img.height() {
        for img_x in 0..img.width() {
            res_img.put_pixel(x + img_x, y + img_y, *img.get_pixel(img_x, img_y));
        }
    }

    (img.width() + x + 4, y)


}


fn info_string(face: &freetype::Face, gen_info: GenInfo) -> String {
    let mut res = "info ".to_string();

    res += &format!("face=\"{:?}\" size={} bold=0 italic=0 charset=\"\" unicode=0 stretchH=100 smooth=1 aa=1 ",
                    face.family_name().unwrap(), gen_info.upscale_res);


    let p = gen_info.padding;
    res += &format!("padding={p},{p},{p},{p}");

    // don't think it is used in the text renderer
    res += &format!("spacing=-8,-8\n");

    res
}





fn common_line_string(lineheight: u32, pages: usize) -> String {
    let mut res = "common ".to_string();
    let base = 300; // Don't use it when rendering so set to 30 for no

    res += &format!("lineHeight={:?} base={} scaleW=512 scaleH=512 pages={} packed=0\n",
                    lineheight, base, pages);
    res
}


fn pages_string(pages: &Vec::<Page>) -> String {

    let mut res = "".to_string();
    for p in pages {
        res += &format!("page id={} file=\"{}\"\n", p.id, p.file);
        res += &format!("chars count={}\n", p.chars.len());

        res += &chars_string(&p.chars);

        res += &kernings_string(&p.kernings);

    }

    res
}

fn chars_string(chars: &Vec::<FontCharInfo>) -> String {

    let mut res = "".to_string();

    for c in chars {
        res += &format!("char id={}    x={}  y={}  width={}  height={}  xoffset={}  yoffset={}  xadvance={} page={} chnl={}\n", c.id, c.x, c.y, c.width, c.height, c.xoffset, c.yoffset, c.xadvance, c.page, c.chnl);
    }

    res
}

fn kernings_string(kernings: &Vec::<KerningInfo>) -> String {

    let mut res = format!("kernings count={}\n", kernings.len());

    for k in kernings  {

    }
    res
}



pub struct FontInfo {
    // INFO
    face: String,
    size: i32,
    bold: bool,
    italic: bool,
    charset: String,
    unicode: i32,
    stretchH: i32,
    smooth: i32,
    padding: [i32;4],
    spacing: [i32;2],

    // COMMON
    line_height: i32,
    base: i32,
    scaleW: i32,
    scaleH: i32,
    packed: i32,


    // DATA
    pages: Vec::<Page>,
}

pub struct Page {
    id: i32,
    file: String,

     // DATA:
    chars: Vec::<FontCharInfo>,
    kernings: Vec::<KerningInfo>,
    image: RgbaImage
}

pub struct FontCharInfo {
    id: u32,
    x: u32, // position in png
    y: u32, // position in png
    width: u32,
    height: u32,
    xoffset: i32,
    yoffset: i32,
    xadvance: i32,
    page: i32,
    chnl: i32,
}


pub struct KerningInfo {
    first: i32,
    second: i32,
    amount: i32,
}
