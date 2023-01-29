use derive_more::*;

#[derive(Add, Sub, BitAnd, BitOr, Shl, Shr, Sum, Not, Into, PartialEq, PartialOrd, Eq)]
pub struct NBitNumber<const N: usize> {
    pub value: u16,
}

pub trait NumberOperations<const N: usize> {
    fn get_max() -> Self;
    fn as_u16(&self) -> u16;
    fn as_usize(&self) -> usize;
    fn get(&self) -> u16;
    fn new() -> Self;

}


impl<const N: usize> NBitNumber<N> {
    pub fn new(value: u16) -> Self {
        NBitNumber { value: value & ((1 << N) - 1)}
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}

impl<const N: usize> NumberOperations<N> for NBitNumber<N> {
    fn get_max() -> Self {
        NBitNumber::<N>::new((1 << N) - 1)
    }

    fn as_u16(&self) -> u16 {
        self.value
    }

    fn as_usize(&self) -> usize {
        self.value as usize
    }

    fn get(&self) -> u16 {
        self.value
    }

    fn new() -> Self {
        NBitNumber::<N>::new(0)
    }
}

impl<const N: usize> Clone for NBitNumber<N> {
    fn clone(&self) -> Self {
        NBitNumber::<N>::new(self.value)
    }
}


impl<const N: usize> Copy for NBitNumber<N> {}


pub type u12 = NBitNumber<12>;
pub type u7 = NBitNumber<7>;
pub type u5 = NBitNumber<5>;
pub type u9 = NBitNumber<9>;
pub type u3 = NBitNumber<3>;
pub type u2 = NBitNumber<2>;