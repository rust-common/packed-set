extern crate int;
extern crate bitrw;
extern crate tbe;
extern crate num_traits;

pub trait UniversalSet {
    type T: int::UInt;
    fn size(self) -> Self::T;
}

///
pub trait OrderedSet {
    type T: int::UInt;
    /// 0 <= value_size()
    fn value_size(&self) -> Self::T;
    /// 0 <= size() <= value_size()
    fn size(&self) -> Self::T;
    /// 0 <= i < size()
    /// 0 <= get(i) < value_size()
    fn get(&self, i: Self::T) -> Self::T;
}

///
pub trait OrderedSetBuilder {
    type T: int::UInt;
    fn add(&mut self, i: Self::T, value: Self::T);
}

pub trait CreateOrderedSet: Copy {
    type T: int::UInt;
    type B: OrderedSetBuilder<T = Self::T>;
    type S: OrderedSet<T = Self::T>;
    fn value_size(self) -> Self::T;
    fn new(self, size: Self::T, f: &mut FnMut(&mut Self::B) -> std::io::Result<()>) -> std::io::Result<Self::S>;
}

struct Range2D<T: int::UInt> {
    pub offset: T,
    pub size: T,
    pub value_offset: T,
    pub value_size: T,
}

impl<T: int::UInt> Range2D<T> {
    fn next_i(&self) -> T {
        self.size >> 1u8
    }
    fn split(self, i: T, value_i: T) -> (Self, Self) {
        let j = i + T::_1;
        let value_j = value_i + T::_1;
        (   Range2D {
                offset: self.offset,
                size: i,
                value_offset: self.value_offset,
                value_size: value_i
            },
            Range2D {
                offset: self.offset + j,
                size: self.size - j,
                value_offset: self.value_offset + value_j,
                value_size: self.value_size - value_j
            }
        )
    }
    fn tbe(&self) -> tbe::TbeStruct<T> {
        use tbe::Tbe;
        (self.value_size - self.size + T::_1).tbe()
    }
    fn new(size: T, value_size: T) -> Self {
        Self { offset: T::_0, size: size, value_offset: T::_0, value_size: value_size }
    }
}

pub trait WriteSet {
    fn ordered_set_write<S: OrderedSet>(self, s: &S) -> std::io::Result<()>;
}

struct WriteFrame<'t, 'b, S: OrderedSet> {
    set: &'t S,
    w: &'t mut bitrw::BitWrite<'b>,
}

impl<S: OrderedSet> WriteFrame<'_, '_, S> {
    fn subset_write(&mut self, range: Range2D<S::T>) -> std::io::Result<()> {
        use int::UInt;
        if S::T::_0 < range.size && range.size < range.value_size {
            use tbe::TbeWrite;
            use num_traits::cast::AsPrimitive;
            let i = range.next_i();
            let value_i = self.set.get(range.offset + i) - range.value_offset;
            self.w.write_tbe(range.tbe(), value_i - i)?;
            let (left, right) = range.split(i, value_i);
            self.subset_write(left)?;
            self.subset_write(right)?;
        }
        Ok(())
    }
}

impl WriteSet for &mut bitrw::BitWrite<'_> {
    fn ordered_set_write<S: OrderedSet>(self, s: &S) -> std::io::Result<()> {
        use tbe::Tbe;
        use tbe::TbeWrite;
        let size = s.size();
        let value_size = s.value_size();
        self.write_tbe(value_size.tbe(), size)?;
        let mut x = WriteFrame { set: s, w: self };
        x.subset_write(Range2D::new(size, value_size))?;
        Ok(())
    }
}

pub trait ReadSet {
    fn ordered_set_read<C: CreateOrderedSet>(&mut self, c: C) -> std::io::Result<C::S>;
}

struct ReadFrame<'t, 'b, B: OrderedSetBuilder> {
    builder: &'t mut B,
    r: &'t mut bitrw::BitRead<'b>,
}

impl<B: OrderedSetBuilder> ReadFrame<'_, '_, B> {
    fn subset_read(&mut self, range: Range2D<B::T>) -> std::io::Result<()> {
        use int::UInt;
        if B::T::_0 < range.size && range.size <= range.value_size {
            use tbe::TbeRead;
            let i = range.next_i();
            let value_i = self.r.read_tbe(range.tbe())? + i;
            self.builder.add(range.offset + i, range.value_offset + value_i);
            let (left, right) = range.split(i, value_i);
            self.subset_read(left)?;
            self.subset_read(right)?;
        }
        Ok(())
    }
}

impl ReadSet for bitrw::BitRead<'_> {
    fn ordered_set_read<C: CreateOrderedSet>(&mut self, c: C) -> std::io::Result<C::S> {
        use tbe::TbeRead;
        use tbe::Tbe;
        let value_size = c.value_size();
        let size = self.read_tbe(value_size.tbe())?;
        c.new(size, &mut |b| {
            ReadFrame { builder: b, r: self }.subset_read(Range2D::new(size, value_size))
        })
    }
}

#[derive(Debug)]
pub struct ByteSet {
    pub data: Vec<u8>
}

impl ByteSet {
    pub fn get_data(&self) -> &Vec<u8> { &self.data }
}

fn byte_set_value_size() -> u16 {
    use int::UInt;
    (u8::MAX_VALUE as u16) + 1
}

impl OrderedSet for ByteSet {
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

impl OrderedSetBuilder for ByteSet {
    type T = u16;
    fn add(&mut self, i: Self::T, value: Self::T) {
        self.data[i as usize] = value as u8;
    }
}

pub struct CreateByteSet {
}

impl Clone for CreateByteSet {
    fn clone(&self) -> Self { CreateByteSet {} }
}

impl Copy for CreateByteSet {}

impl CreateOrderedSet for CreateByteSet {
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
