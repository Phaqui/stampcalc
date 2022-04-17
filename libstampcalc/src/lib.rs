mod factors;
mod vec_mulsum;
pub mod new_algo;

use std::ops::{AddAssign, Div};
use std::iter::Sum;
use num::{Integer, ToPrimitive, FromPrimitive};
use crate::factors::{Factors, FactorsIter};
use crate::vec_mulsum::vec_mulsum;

// Just grouping all the trait bounds together
pub trait Number: Integer + AddAssign + Sum + Div + ToPrimitive + FromPrimitive + Clone + Copy {}
impl Number for u32 {}

pub struct Solutions<T: Number> {
    stamps: Vec<T>,
    factors: Factors<T>,
    price: T,
}

pub struct SolutionsIter<'a, T: Number> {
    factors_iter: FactorsIter<'a, T>,
    stamps: Vec<T>,
    price: T,
}

impl<T: Number> Solutions<T> {
    pub fn new(price: T, stamps: &Vec<T>) -> Self {
        let max_factors = stamps.into_iter()
            .map(|s| price / *s)
            .collect::<Vec<T>>();

        let factors = Factors::new(&max_factors);

        Self {
            factors,
            stamps: stamps.clone(),
            price,
        }
    }

    pub fn num_iterations_for(&self) -> usize {
        let max_factors = self.stamps.iter()
            .map(|s| self.price / *s).collect::<Vec<_>>();
        Factors::new(&max_factors).make_into_iterator().size_hint().0
    }

    pub fn make_into_iterator<'a>(&'a mut self) -> SolutionsIter<'a, T> {
        let factors_iter = self.factors.make_into_iterator();

        SolutionsIter {
            stamps: self.stamps.clone(),
            factors_iter,
            price: self.price,
        }
    }
}

impl<T> Iterator for SolutionsIter<'_, T>
where
    T: Number
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.factors_iter.next() {
                None => break None,
                Some(factors) => {
                    if self.price == vec_mulsum(&factors, &self.stamps) {
                        break Some(factors.clone())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod solutions_test {
    use super::*;

    #[test]
    fn simple() {
        let mut solutions = Solutions::new(100, &vec![20, 50]);
        let answers = solutions.make_into_iterator();
        let mut nres: usize = 0;
        for _ in answers {
            nres += 1;
        }
        assert_eq!(nres, 2);
    }
}
