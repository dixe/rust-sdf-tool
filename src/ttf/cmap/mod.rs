use crate::ttf::*;

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct CmapTable {
    pub index: CmapIndex,
    pub sub_tables: Vec<CmapSubTable>,
}

impl Read for CmapTable {
    fn read(stream: &mut Stream) -> Option<Self> {
        stream.base = stream.offset;
        let idx = stream.read::<CmapIndex>()?;
        let base_offset = stream.offset;

        let mut sub_tables = vec![];
        for i in 0..idx.num_subtables as usize {
            stream.offset = base_offset + i * std::mem::size_of::<CmapEncoding>();
            let sub_table = CmapSubTable::read(stream);

            if let Some(st) = sub_table {
                sub_tables.push(st);
            }
        }

        Some(CmapTable {
            index: idx,
            sub_tables,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct CmapIndex {
    version: u16,
    num_subtables: u16,
}

impl FromData for CmapIndex {
    fn parse(data: &[u8]) -> Option<Self> {
        let mut stream = Stream {
            data,
            offset: 0,
            base: 0,
        };

        Some(CmapIndex {
            version: stream.read::<u16>()?,
            num_subtables: stream.read::<u16>()?,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct CmapEncoding {
    platform_id: u16,
    platform_specifier_id: u16,
    offset: u32,
}

impl Read for CmapEncoding {
    fn read(stream: &mut Stream) -> Option<Self> {
        Some(CmapEncoding {
            platform_id: stream.read::<u16>()?,
            platform_specifier_id: stream.read::<u16>()?,
            offset: stream.read::<u32>()?,
        })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct CmapSubTable {
    encoding: CmapEncoding,
    format: Format
}


impl CmapSubTable {

    pub fn get_glyph_id(&self, data: &u8) -> usize {
        0
    }
}

impl Read for CmapSubTable {
    fn read(stream: &mut Stream) -> Option<Self> {
        let encoding = CmapEncoding::read(stream)?;

        // Ignore all but unicode
        if encoding.platform_id != 0 {
            println!(
                "Ignore platformId = {}, specifier={}",
                encoding.platform_id, encoding.platform_specifier_id
            );
            return None;
        }

        // format is after all the encodings, assume that stream base has been set in call to read
        stream.offset = stream.base + encoding.offset as usize;

        let format_id = stream.peek::<u16>()?;

        let format = match format_id {
            4 => {
                let f4 = stream.read::<Format4>()?;
                Format::V4(f4)
            }
            _ => {
                println!("Only format 4 is supported, ignoring this: {:?}", format_id);
                return None;
            }
        };

        println!(
            "{}, {}, {:#?}",
            encoding.platform_id, encoding.platform_specifier_id, format
        );
        Some(CmapSubTable { encoding, format})
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Format {
    N,
    V4(Format4),
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct Format4 {
    format: u16,
    length: u16,
    lang: u16,
    seg_count_x2: u16,
    search_range: u16,
    entry_select: u16,
    range_shifter: u16,
    end_code: u16,
    reserved_pad: u16,
    start_code: u16,
    id_delta: u16,
    id_range_offset: u16,
    glyph_index_array: u16,
}

impl FromData for Format4 {
    fn parse(data: &[u8]) -> Option<Self> {
        let mut stream = Stream {
            data,
            offset: 0,
            base: 0,
        };

        Some(Format4 {
            format: stream.read::<u16>()?,
            length: stream.read::<u16>()?,
            lang: stream.read::<u16>()?,
            seg_count_x2: stream.read::<u16>()?,
            search_range: stream.read::<u16>()?,
            entry_select: stream.read::<u16>()?,
            range_shifter: stream.read::<u16>()?,
            end_code: stream.read::<u16>()?,
            reserved_pad: stream.read::<u16>()?,
            start_code: stream.read::<u16>()?,
            id_delta: stream.read::<u16>()?,
            id_range_offset: stream.read::<u16>()?,
            glyph_index_array: stream.read::<u16>()?,
        })
    }
}
