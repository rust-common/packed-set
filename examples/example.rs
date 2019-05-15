extern crate set_encoding;
extern crate int;
extern crate bitrw;
extern crate tbe;

fn test(data: Vec<u8>) {
    let set = set_encoding::ByteSet { data: data };
    let v = bitrw::use_bit_write_mem(&mut |w| {
        use set_encoding::WriteSet;
        w.ordered_set_write(&set)
    }).unwrap();
    {
        use bitrw::UseBitRead;
        use set_encoding::ReadSet;
        println!("source: {:02x?}, stream: {:02x?}", set.get_data(), v);
        let mut cursor = std::io::Cursor::new(v);
        let mut r = cursor.use_bit_read();
        let result = r.ordered_set_read(set_encoding::CreateByteSet {}).unwrap();
        assert_eq!(result.get_data(), set.get_data());
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
