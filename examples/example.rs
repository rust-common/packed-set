extern crate set_encoding;
extern crate int;
extern crate bitrw;
extern crate tbe;

#[derive(Debug)]
struct ByteSet {
    data: Vec<u8>
}

fn byte_set_value_size() -> u16 {
    use int::UInt;
    (u8::MAX_VALUE as u16) + 1
}

impl set_encoding::OrderedSet for ByteSet {
    type T = u16;
    /// 0 <= value_size()
    fn value_size(&self) -> Self::T { byte_set_value_size() }
    /// 0 <= size() <= value_size()
    fn size(&self) -> Self::T {
        self.data.len() as u16
    }
    /// 0 <= i < size()
    /// 0 <= get(i) < value_size()
    fn get(&self, i: Self::T) -> Self::T {
        self.data[i as usize] as u16
    }
}

impl set_encoding::OrderedSetBuilder for ByteSet {
    type T = u16;
    fn add(&mut self, i: Self::T, value: Self::T) {
        self.data[i as usize] = value as u8;
    }
}

struct CreateByteSet {
}

impl Clone for CreateByteSet {
    fn clone(&self) -> Self { CreateByteSet {} }
}

impl Copy for CreateByteSet {}

impl set_encoding::CreateOrderedSet for CreateByteSet {
    type T = u16;
    type B = ByteSet;
    type S = ByteSet;
    fn value_size(self) -> Self::T { byte_set_value_size() }
    fn new(self, size: Self::T, f: &mut FnMut(&mut Self::B) -> std::io::Result<()>) -> std::io::Result<Self::S> {
        let mut result = ByteSet { data: vec![0; size as usize] };
        f(&mut result)?;
        Ok(result)
    }
}

fn test(data: Vec<u8>) {
    let set = ByteSet { data: data };
    let v = bitrw::use_bit_write_mem(&mut |w| {
        use set_encoding::WriteSet;
        w.ordered_set_write(&set)
    }).unwrap();
    {
        use bitrw::UseBitRead;
        use set_encoding::ReadSet;
        println!("source: {:02x?}, stream: {:02x?}", set.data, v);
        let mut cursor = std::io::Cursor::new(v);
        let mut r = cursor.use_bit_read();
        let result = r.ordered_set_read(CreateByteSet {}).unwrap();
        assert_eq!(result.data, set.data);
    }
}

fn main() -> () {
    test(vec![]);
    test(vec![5]);
    test(vec![0,255]);
    test(vec![0, 128, 130]);
    test(vec![32, 64, 128, 192]);
    test(vec![0, 1]);
    test(vec![0, 1, 2]);
    test(vec![0, 1, 2, 3]);
    test(vec![0, 1, 2, 3, 255]);
    test(vec![0, 1, 2, 3, 4]);
    test(vec![0, 1, 2, 3, 4, 5]);
    test(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    test(vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112]);
    test(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
}
