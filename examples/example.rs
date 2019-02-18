extern crate set_encoding;
extern crate int;
extern crate bitrw;

struct Set {
    data: Vec<u8>
}

impl set_encoding::OrderedSet for Set {
    type T = u16;
    /// 0 <= value_size()
    fn value_size(&self) -> Self::T {
        use int::UInt;
        (u8::MAX_VALUE as u16) + 1
    }
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

fn main() -> () {
    let v = bitrw::use_bit_write_mem(&mut |w| {
        w.write(0_u8, 0)?; //  0
        w.write(1_u16, 1)?; //  1
        w.write(2_u32, 2)?; //  3
        w.write(3_u64, 3)?; //  6
        w.write(4_u128, 4)?; // 10
        w.write(5_usize, 5)?; // 15
        w.write(6_u8, 6)?; // 21
        w.write(0xFFFF_u16, 12)?; // 33
        Ok(())
    });
}
