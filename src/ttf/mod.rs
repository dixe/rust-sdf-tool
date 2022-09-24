/// A true type Loading

mod head;
mod cmap;
mod data_types;
use data_types::*;

#[derive(Debug, Clone)]
pub struct FontInfo {
    pub offset_table: OffsetTable,
    pub head_table: head::Table,
    pub cmap_table: cmap::CmapTable
}



pub fn parse_font(data: &[u8]) -> Option<FontInfo> {
    // parse offset_table

    let offset_table = OffsetTable::parse(data)?;


    let mut head_table = None;
    let mut cmap_table = None;

    let base_offset = OffsetTable::SIZE;


    let mut stream = Stream { data, offset: 0, base: 0 };

    for i  in 0..(offset_table.num_tables as usize) {
        let offset = base_offset + i * TableDir::SIZE;
        stream.offset = offset;
        let table_dir = stream.read::<TableDir>()?;

        stream.offset = table_dir.offset as usize;
        match &table_dir.tag.to_be_bytes() {
            b"head"  => {
                head_table = stream.read::<head::Table>();
            },

            b"cmap"  => {
                cmap_table = cmap::CmapTable::read(&mut stream);
            },
            unknown => {
                println!("Unknown header: '{:?}' ignored", String::from_utf8( unknown.to_vec()));
            }
        }

    }

    Some(FontInfo {
        offset_table,
        head_table: head_table?,
        cmap_table: cmap_table?,
    })
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TableDir {
    tag: u32,
    checksum: u32,
    offset: u32,
    length: u32,
}

impl FromData for TableDir {
    fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 16 {
            return None;
        }

        Some(TableDir {
            tag: u32::parse(&data[0..4])?,
            checksum: u32::parse(&data[4..8])?,
            offset: u32::parse(&data[8..12])?,
            length: u32::parse(&data[12..16])?,
        })
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OffsetTable {
    scaler_type: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

impl FromData for OffsetTable {
    fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 12 {
            return None;
        }

        Some(OffsetTable {
            scaler_type: u32::parse(&data[0..4])?,
            num_tables: u16::parse(&data[4..6])?,
            search_range: u16::parse(&data[6..8])?,
            entry_selector: u16::parse(&data[8..10])?,
            range_shift: u16::parse(&data[10..12])?,
        })
    }
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn parse_table_some() {
        let data: [u8; 12] = [0, 0, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10];
        let table = OffsetTable::parse(&data);
        assert_ne!(None, table);

        let t = table.unwrap();

        assert_eq!(10, t.scaler_type);
        assert_eq!(10, t.num_tables);
        assert_eq!(10, t.search_range);
        assert_eq!(10, t.entry_selector);
        assert_eq!(10, t.range_shift);
    }

    #[test]
    fn parse_table_none() {
        let data: [u8; 9] = [0, 0, 0, 10, 0, 10, 0, 10, 0];
        let table = OffsetTable::parse(&data);

        assert_eq!(None, table);
    }


}
