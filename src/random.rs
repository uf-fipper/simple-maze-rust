use std::ops::Range;

use rand::Rng;

pub trait CanRand {
    fn from_n(n: usize) -> Vec<Self>
    where
        Self: Sized;
}

macro_rules! can_read_impl {
    ($($t:ty)*) => ($(
        impl CanRand for $t {
            #[inline]
            fn from_n(n: usize) -> Vec<Self> {
                Vec::from_iter(0..(n as Self))
            }
        }
    )*)
}

can_read_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub fn randindex<T>(rng: &mut impl Rng, n: usize) -> Vec<T>
where
    T: CanRand,
    Range<T>: Iterator<Item = T>,
{
    let mut res = T::from_n(n);
    for i in 0..n {
        let temp = rng.gen_range(i..n);
        unsafe { std::ptr::swap(&mut res[i], &mut res[temp]) }
    }
    res
}

pub fn randarray<T>(rng: &mut impl Rng, arr: &Vec<T>) -> Vec<T>
where
    T: Default + Clone,
{
    let res_index: Vec<usize> = randindex::<usize>(rng, arr.len());
    let mut res = vec![T::default(); arr.len()];
    for (i, index) in res_index.iter().enumerate() {
        res[i] = arr[*index].clone();
    }
    res
}
