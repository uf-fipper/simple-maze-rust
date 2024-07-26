use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<T = i32>(pub T, pub T);

impl<T> Default for Point<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default(), T::default())
    }
}

impl<T> Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub trait CanPointIndex {
    fn one() -> Self;
    fn to_usize(&self) -> usize;
}

macro_rules! can_point_index_impl {
    ($($t:ty)*) => ($(
        impl CanPointIndex for $t {
            #[inline]
            fn one() -> Self {
                1
            }

            #[inline]
            fn to_usize(&self) -> usize {
                *self as usize
            }
        }
    )*)
}

can_point_index_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + CanPointIndex,
{
    pub fn get_range_vec(&self) -> Vec<Self> {
        vec![
            Point(self.0 - T::one(), self.1),
            Point(self.0 + T::one(), self.1),
            Point(self.0, self.1 - T::one()),
            Point(self.0, self.1 + T::one()),
        ]
    }

    pub fn get_range_tuple(&self) -> (Self, Self, Self, Self) {
        (
            Point(self.0 - T::one(), self.1),
            Point(self.0 + T::one(), self.1),
            Point(self.0, self.1 - T::one()),
            Point(self.0, self.1 + T::one()),
        )
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Add<(T, T)> for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Sub<(T, T)> for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T, U> Index<Point<U>> for Vec<Vec<T>>
where
    U: CanPointIndex,
{
    type Output = T;

    fn index(&self, index: Point<U>) -> &Self::Output {
        &self[index.0.to_usize()][index.1.to_usize()]
    }
}

impl<T, U> IndexMut<Point<U>> for Vec<Vec<T>>
where
    U: CanPointIndex,
{
    fn index_mut(&mut self, index: Point<U>) -> &mut Self::Output {
        &mut self[index.0.to_usize()][index.1.to_usize()]
    }
}
