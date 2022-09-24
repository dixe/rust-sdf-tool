pub type FWord = u16;
pub type LongDateTime = u64;


pub trait FromData: Sized {
    const SIZE : usize = std::mem::size_of::<Self>();
    fn parse(data: &[u8]) -> Option<Self>;
}

pub trait Read : Sized {
    fn read(stream: &mut Stream) -> Option<Self>;
}

pub struct Stream<'a> {
    pub offset: usize,
    pub data: &'a[u8],
    pub base: usize,

}

impl<'a> Stream<'a> {

    pub fn read<T>(&mut self) -> Option<T> where T: FromData {
        let res = T::parse(&self.data[self.offset..self.offset + T::SIZE]);
        self.offset += T::SIZE;
        res
    }

     pub fn peek<T>(&mut self) -> Option<T> where T: FromData {
        T::parse(&self.data[self.offset..self.offset + T::SIZE])
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fixed {
    whole: i16,
    frac: u16
}

impl FromData for Fixed {
    fn parse(data: &[u8]) -> Option<Self> {

        let mut stream = Stream {data, offset: 0, base: 0 };
        Some(Fixed {
            whole: stream.read::<i16>()?,
            frac:  stream.read::<u16>()?,
        })
    }
}



impl FromData for u16 {
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u16::from_be_bytes)
    }
}

impl FromData for i16 {
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(i16::from_be_bytes)
    }
}

impl FromData for u32 {
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u32::from_be_bytes)
    }
}


impl FromData for u64 {
    fn parse(data: &[u8]) -> Option<Self> {
        data.try_into().ok().map(u64::from_be_bytes)
    }
}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_u16() {
        let data: [u8; 12] = [0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10];

        let num = u16::parse(&data[0..2]);
        assert_ne!(None, num);

        assert_eq!(10, num.unwrap());
    }

    #[test]
    fn parse_u32() {
        let data: [u8; 12] = [0, 0, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10];

        let num = u32::parse(&data[0..4]);
        assert_ne!(None, num);

        assert_eq!(10, num.unwrap());
    }



}
