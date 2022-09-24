use crate::ttf::*;

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct Table {
    version: Fixed,
    font_revision: Fixed,
    check_sum_adj: u32,
    magic_num: u32,
    flags: u16,
    unit_pr_em: u16,
    created: LongDateTime,
    modified: LongDateTime,
    x_min: FWord,
    y_min: FWord,
    x_max: FWord,
    y_max: FWord,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_dir_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16
}




impl FromData for Table {
    fn parse(data: &[u8]) -> Option<Self> {

        let mut stream = Stream {
            data,
            offset: 0,
            base: 0
        };

        Some(Table {
            version: stream.read::<Fixed>()?,
            font_revision: stream.read::<Fixed>()?,
            check_sum_adj: stream.read::<u32>()?,
            magic_num: stream.read::<u32>()?,
            flags: stream.read::<u16>()?,
            unit_pr_em: stream.read::<u16>()?,
            created: stream.read::<LongDateTime>()?,
            modified: stream.read::<LongDateTime>()?,
            x_min: stream.read::<FWord>()?,
            y_min: stream.read::<FWord>()?,
            x_max: stream.read::<FWord>()?,
            y_max: stream.read::<FWord>()?,
            mac_style: stream.read::<u16>()?,
            lowest_rec_ppem: stream.read::<u16>()?,
            font_dir_hint: stream.read::<i16>()?,
            index_to_loc_format: stream.read::<i16>()?,
            glyph_data_format: stream.read::<i16>()?
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse() {
        let data :[u8;64]= [0x1, 0x1, 0x1, 0x1, 0x1, 0x6, 0x3a, 0xe1, 0x86, 0x2, 0xeb, 0x54, 0x5f, 0xf, 0x3c, 0xf5, 0x1, 0x19, 0x8, 0x1, 0x1, 0x1, 0x1, 0x1, 0xbb, 0xeb, 0x7c, 0xcc, 0x1, 0x10, 0x1, 0x1, 0xd7, 0x49, 0x77, 0x4f, 0xfb, 0xfa, 0xfd, 0x80, 0x9, 0xec, 0x8, 0x36, 0x1, 0x1, 0x1, 0x9, 0x1, 0x2, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x6, 0x1, 0xfe, 0x1];

        let table = Table::parse(&data).unwrap();

        assert_eq!(0x5F0F3CF5, table.magic_num);


    }
}
