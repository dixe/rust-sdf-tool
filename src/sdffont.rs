

struct FontInfo {
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

struct Page {
    id: i32,
    file: String,

     // DATA:
    chars: Vec::<FontCharInfo>,
    kernings: Vec::<KerningInfo>,
}

struct FontCharInfo {
    id: i32,
    x: i32, // position in png
    y: i32, // position in png
    width: i32,
    height: i32,
    xoffset: i32,
    yoffset: i32,
    xadvance: i32,
    page: i32,
    chnl: i32,
}


struct KerningInfo {
    first: i32,
    second: i32,
    amount: i32,
}
