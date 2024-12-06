use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T = char>(Vec<Vec<T>>);

impl Grid {
    pub fn parse(input: &str) -> Self {
        Self(input.lines().map(|x| x.chars().collect()).collect())
    }
}

impl<T> Grid<T> {
    pub fn contains(&self, v: Vector2) -> bool {
        (0..self.0.len()).contains(&v.1) && (0..self.0[v.1].len()).contains(&v.0)
    }

    pub fn get(&self, v: Vector2) -> Option<&T> {
        self.0.get(v.1).and_then(|l| l.get(v.0))
    }

    pub fn get_mut(&mut self, v: Vector2) -> Option<&mut T> {
        self.0.get_mut(v.1).and_then(|l| l.get_mut(v.0))
    }

    pub fn row<'a>(&'a self, y: usize) -> impl use<'a, T> + Iterator<Item = &'a T> {
        assert!(self.contains(Vector2(0, y)));
        self.0[y].iter()
    }

    pub fn col<'a>(&'a self, x: usize) -> impl use<'a, T> + Iterator<Item = &'a T> {
        assert!(self.contains(Vector2(x, 0)));
        self.0.iter().map(move |r| &r[x])
    }

    pub fn row_mut<'a>(&'a mut self, y: usize) -> impl use<'a, T> + Iterator<Item = &'a mut T> {
        assert!(self.contains(Vector2(0, y)));
        self.0[y].iter_mut()
    }

    pub fn col_mut<'a>(&'a mut self, x: usize) -> impl use<'a, T> + Iterator<Item = &'a mut T> {
        assert!(self.contains(Vector2(x, 0)));
        self.0.iter_mut().map(move |r| &mut r[x])
    }

    pub fn enumerate<'a>(&'a self) -> impl use<'a, T> + Iterator<Item = (Vector2, &'a T)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, t)| (Vector2(x, y), t)))
    }

    pub fn enumerate_mut<'a>(
        &'a mut self,
    ) -> impl use<'a, T> + Iterator<Item = (Vector2, &'a mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(y, r)| {
            r.iter_mut()
                .enumerate()
                .map(move |(x, t)| (Vector2(x, y), t))
        })
    }

    pub fn domain(&self) -> impl use<'_, T> + Iterator<Item = Vector2> {
        (0..self.0.len()).flat_map(|y| (0..self.0[y].len()).map(move |x| Vector2(x, y)))
    }

    pub fn iter<'a>(&'a self) -> impl use<'a, T> + Iterator<Item = &'a T> {
        self.0.iter().flat_map(|r| r.iter())
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl use<'a, T> + Iterator<Item = &'a mut T> {
        self.0.iter_mut().flat_map(|r| r.iter_mut())
    }
}

impl<T> Index<Vector2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vector2) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Index {index} out of range."))
    }
}

impl<T> IndexMut<Vector2> for Grid<T> {
    fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Index {index} out of range."))
    }
}

/// Ords are for BTreeMaps, not mathematical comparison
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2<T = usize>(pub T, pub T);

impl<T: Debug> Debug for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.0, self.1)
    }
}

impl<T: Add<T, Output = T>> Add<Self> for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<T, Output = T>> Sub<Self> for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<T, Output = T> + Clone> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2(self.0 * rhs.clone(), self.1 * rhs)
    }
}

impl<T: AddAssign<T>> AddAssign<Self> for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: SubAssign<T>> SubAssign<Self> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: MulAssign<T> + Clone> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs.clone();
        self.1 *= rhs.clone();
    }
}

impl Vector2 {
    pub const ZERO: Self = Self(0, 0);
    pub const ONE: Self = Self(1, 1);

    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0), self.1.wrapping_add(rhs.1))
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0), self.1.wrapping_sub(rhs.1))
    }

    pub fn wrapping_add_i(self, rhs: Vector2<isize>) -> Self {
        self.i().wrapping_add(rhs).u()
    }

    pub fn wrapping_sub_i(self, rhs: Vector2<isize>) -> Self {
        self.i().wrapping_sub(rhs).u()
    }

    pub fn i(self) -> Vector2<isize> {
        Vector2(self.0 as isize, self.1 as isize)
    }
}

impl Vector2<isize> {
    pub const ZERO: Self = Self(0, 0);
    pub const ONE: Self = Self(1, 1);

    pub const UP: Self = Self(0, -1);
    pub const LEFT: Self = Self(-1, 0);
    pub const DOWN: Self = Self(0, 1);
    pub const RIGHT: Self = Self(1, 0);

    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0), self.1.wrapping_add(rhs.1))
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0), self.1.wrapping_sub(rhs.1))
    }

    pub fn u(self) -> Vector2<usize> {
        Vector2(self.0 as usize, self.1 as usize)
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<T: Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
