use crate::{
  MulTrunc, WrappingAdd, WrappingDiv, WrappingFrom, WrappingInto, WrappingMul, WrappingSub,
};
use core::{
  cmp::Ordering,
  fmt,
  marker::PhantomData,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[repr(transparent)]
pub struct Measure<T, S>(pub T, PhantomData<S>);
impl<T, S> Measure<T, S> {
  #[inline]
  pub const fn new(x: T) -> Self {
    Self(x, PhantomData)
  }

  #[inline]
  pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Measure<U, S> {
    Measure::new(f(self.0))
  }

  #[inline]
  pub fn with_sys<S2>(self) -> Measure<T, S2> {
    Measure::new(self.0)
  }
}

impl<T: Default, S> Default for Measure<T, S> {
  #[inline]
  fn default() -> Self {
    Self::new(T::default())
  }
}

impl<T: Clone, S> Clone for Measure<T, S> {
  #[inline]
  fn clone(&self) -> Self {
    Self::new(self.0.clone())
  }
}
impl<T: Copy, S> Copy for Measure<T, S> {}

impl<T: PartialEq, S> PartialEq for Measure<T, S> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
impl<T: PartialEq, S> PartialEq<T> for Measure<T, S> {
  #[inline]
  fn eq(&self, other: &T) -> bool {
    self.0 == *other
  }
}
impl<T: Eq, S> Eq for Measure<T, S> {}

impl<T: PartialOrd, S> PartialOrd for Measure<T, S> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}
impl<T: PartialOrd, S> PartialOrd<T> for Measure<T, S> {
  #[inline]
  fn partial_cmp(&self, other: &T) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
impl<T: Ord, S> Ord for Measure<T, S> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl<T: fmt::Debug, S> fmt::Debug for Measure<T, S> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}
impl<T: fmt::Display, S> fmt::Display for Measure<T, S> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl<T: WrappingFrom<U>, U, S> WrappingFrom<Measure<U, S>> for Measure<T, S> {
  #[inline]
  fn wfrom(x: Measure<U, S>) -> Self {
    Self::new(x.0.winto())
  }
}

impl<T: Neg, S> Neg for Measure<T, S> {
  type Output = Measure<T::Output, S>;
  #[inline]
  fn neg(self) -> Self::Output {
    Measure::new(-self.0)
  }
}

macro_rules! impl_op {
  ($op:ident<$other_ty:ty>, $fn:ident $(, .$field_rhs:tt)?) => {
    impl<T: $op<U>, U, S> $op<$other_ty> for Measure<T,S> {
      type Output = Measure<T::Output, S>;
      fn $fn(self, rhs: $other_ty) -> Self::Output {
        Measure::new(
          $op::$fn(self.0, rhs$(.$field_rhs)?)
        )
      }
    }
  };
}

macro_rules! impl_op_assign {
  ($op:ident<$other_ty:ty>, $fn:ident $(, .$field_rhs:tt)?) => {
    impl<T: $op<U>, U, S> $op<$other_ty> for Measure<T, S> {
      fn $fn(&mut self, rhs: $other_ty) {
        $op::$fn(&mut self.0, rhs$(.$field_rhs)?);
      }
    }
  };
}

impl_op!(Add<Measure<U, S>>, add, .0);
impl_op!(Sub<Measure<U, S>>, sub, .0);
impl_op!(Mul<U>, mul);
impl_op!(MulTrunc<U>, mul_trunc);
impl_op!(Div<U>, div);
impl_op!(WrappingAdd<Measure<U, S>>, wadd, .0);
impl_op!(WrappingSub<Measure<U, S>>, wsub, .0);
impl_op!(WrappingMul<U>, wmul);
impl_op!(WrappingDiv<U>, wdiv);

impl_op_assign!(AddAssign<Measure<U, S>>, add_assign, .0);
impl_op_assign!(SubAssign<Measure<U, S>>, sub_assign, .0);
impl_op_assign!(MulAssign<U>, mul_assign);
impl_op_assign!(DivAssign<U>, div_assign);
