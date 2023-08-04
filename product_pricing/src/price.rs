use std::{fmt::Display, ops::Div};
use std::{iter::Sum, ops::Add};

use anyhow::bail;

// The underlying type is i32;
// there's probably a better way of handling this, but
// this will do for the demo; At the DB level,
// we have a check that ensures the price is not less than zero;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cents(i32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(Cents);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Margin(Cents);

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0 .0, f)
    }
}

impl TryFrom<i32> for Price {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            bail!("Value cannot be less than zero")
        }

        Ok(Price(Cents(value)))
    }
}

impl Price {
    pub(crate) fn is_zero(&self) -> bool {
        self.0 .0 == 0
    }
    pub(crate) fn zero() -> Self {
        Self(Cents(0))
    }
}

impl AsRef<i32> for Price {
    fn as_ref(&self) -> &i32 {
        &self.0 .0
    }
}

impl Add for Price {
    type Output = Price;

    fn add(self, rhs: Self) -> Self::Output {
        let cents = self.0 .0 + rhs.0 .0;
        Self(Cents(cents))
    }
}

impl Add<Margin> for Price {
    type Output = Price;
    fn add(self, rhs: Margin) -> Self::Output {
        let cents = self.0 .0 + rhs.0 .0;
        Self(Cents(cents))
    }
}

impl Div<usize> for Price {
    type Output = Price;
    fn div(self, rhs: usize) -> Self::Output {
        let division = self.0 .0 / rhs as i32;
        Self(Cents(division))
    }
}

impl Sum for Price {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Price(Cents(0)), |acc, x| acc + x)
    }
}
