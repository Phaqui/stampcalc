use std::ops::AddAssign;
use std::marker::PhantomData;
use std::ptr::NonNull;
use num::{Integer, ToPrimitive, FromPrimitive};

pub struct Factors<T: Integer + ToPrimitive + Clone> {
    factors: Vec<T>,

    // pointed-to (and modified) by the iter struct
    current: Vec<T>,
}

pub struct FactorsIter<'a, T: 'a + FromPrimitive + ToPrimitive> {
    // points to Factors.current
    // will mutate through it
    current: NonNull<Vec<T>>,

    // points to Factors.factors (used for bounds checking), will only be read
    factors: &'a Vec<T>,

    marker: PhantomData<&'a T>,
    iterations_left: usize,
}

impl<T: Integer + ToPrimitive + FromPrimitive + Clone> Factors<T> {
    pub fn new(input: &Vec<T>) -> Self {
        Self {
            factors: input.clone(),
            current: vec![T::zero(); input.len()],
        }
    }

    pub fn make_into_iterator<'a>(&'a mut self) -> FactorsIter<'a, T> {
        FactorsIter {
            current: NonNull::new(&mut self.current as *mut _).unwrap(),
            factors: &self.factors,
            iterations_left: if self.factors.len() == 0 { 0 } else {
                self.factors.iter().map(|v| (*v).to_usize().unwrap() + 1).product()
            },
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for FactorsIter<'a, T>
where
    T: 'a + Integer + FromPrimitive + AddAssign + ToPrimitive + Clone,
{
    type Item = &'a mut Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations_left == 0 { return None; }
        self.iterations_left -= 1;

        let current: &mut Vec<T>;
        // SAFETY:
        // self.ptr is made from factors.current, a normal Vec<T>, and so is
        // (1) properly aligned
        // (2) deferenceable
        // (3) pointing to an initialized Vec<T>
        // (4) ... lifetime rules?
        unsafe {
            current = self.current.as_mut();
        }

        for i in (0..current.len()).rev() {
            current[i] += T::one();

            if current[i] > self.factors[i] {
                current[i] = T::zero();
            } else {
                break;
            }
        }

        return Some(current);
        /*
        // 0,0 -> 0,1 -> 0,2 -> 0,3 -> 1,0 -> 1,1 -> ...

        let ret = Some(self.current.clone());

        for i in (0..self.current.len()).rev() {
            self.current[i] += T::one();

            if self.current[i] > self.factors[i] {
                self.current[i] = T::zero();
            } else {
                break;
            }
        }

        return ret;
        */
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.iterations_left;
        (l, Some(l))
    }
}

//impl<T: Integer + AddAssign + Clone> core::iter::FusedIterator for Factors<T> {}

/*
pub fn factors<'a, T>(input: &'a Vec<T>) -> FactorsIter<'a, T>
where
    T: Integer + ToPrimitive + Clone,
{
    let mut facs = Factors::new(input);

    facs.make_into_iterator()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zerolen() {
        let expected: Vec<Vec<u32>> = Vec::new();
        //let actual: Vec<Vec<u32>> = factors(&vec![]).collect();
        let mut factors = Factors::new(&vec![]);
        let actual = factors.make_into_iterator();
        for (e, a) in zip(expected, actual) {
            assert_eq!(e, *a);
        }
    }

    #[test]
    fn test_only_one_zero() {
        let expected: Vec<Vec<usize>> = vec![vec![0]];
        let mut factors: Factors<usize> = Factors::new(&vec![0]);
        let actual = factors.make_into_iterator();
        for (e, a) in zip(expected, actual) {
            assert_eq!(e, *a);
        }
    }

    #[test]
    fn test_single_dimension() {
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![2], vec![3]];
        let mut factors: Factors<u32> = Factors::new(&vec![3]);
        let actual = factors.make_into_iterator();
        for (e, a) in zip(expected, actual) {
            assert_eq!(e, *a);
        }
    }

    #[test]
    fn test_multiple_dimensions() {
        let expected: Vec<Vec<usize>> = vec![
            /* skip */  vec![0, 1], vec![0, 2], vec![0, 3],
            vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3],
            vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3],
        ];
        let mut factors: Factors<usize> = Factors::new(&vec![2, 3]);
        let actual = factors.make_into_iterator();

        for (e, a) in zip(expected, actual) {
            assert_eq!(e, *a);
        }
    }
}
