extern crate int;
extern crate bitrw;
extern crate tbe;

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
pub trait SetBuilder<T: int::UInt> {
    fn add(&mut self, value: T);
}

pub trait CreateSet<T: int::UInt, B: SetBuilder<T>> {
    fn parent_size(&self) -> T;
    fn new(&self, size: T) -> B;
}

pub trait WriteSet {
    fn ordered_set_write<S: OrderedSet>(&mut self, s: &S) -> std::io::Result<()>;
}

struct WriteFrame<'t, 'b, S: OrderedSet> {
    set: &'t S,
    w: &'t mut bitrw::BitWrite<'b>,
}

impl<S: OrderedSet> WriteFrame<'_, '_, S> {
    fn subset_write(&mut self, offset: S::T, size: S::T, value_offset: S::T, value_size: S::T) -> std::io::Result<()> {
        use int::UInt;
        if S::T::_0 < size && size < value_size {
            use tbe::Tbe;
            use tbe::TbeWrite;
            let i = size >> 1;
            let value_i = self.set.get(offset + i) - value_offset;
            self.w.write_tbe(value_size.tbe(), value_i)?;
            self.subset_write(offset, i, value_offset, value_i)?;
            {
                let j = i + S::T::_1;
                let value_j = value_i + S::T::_1;
                self.subset_write(offset + j, size - j, value_offset + value_j, value_size - value_j)?;
            }
        }
        Ok(())
    }
}

impl WriteSet for bitrw::BitWrite<'_> {
    fn ordered_set_write<S: OrderedSet>(&mut self, s: &S) -> std::io::Result<()> {
        use tbe::Tbe;
        use tbe::TbeWrite;
        use int::UInt;
        let size = s.size();
        let value_size = s.value_size();
        self.write_tbe(value_size.tbe(), size)?;
        let mut x = WriteFrame { set: s, w: self };
        x.subset_write(S::T::_0, size, S::T::_0, value_size)?;
        Ok(())
    }
}
