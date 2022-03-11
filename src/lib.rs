use rdbm::rdbm;
use udbm_rs::udbm;

use num::Bounded;
use num::Zero;

use crate::rdbm::DBM as RDBM;
use udbm::DBM as UDBM; //had some trouble with namespacing in the original repo, and decided to just leave it. Might fix later (probably not)

trait DBM<T> {
    fn init(dim: usize) -> Self;
    fn zero(dim: usize) -> Self;
    fn is_included_in(rhs_dbm: &Self, lhs_dbm: &Self) -> bool;
    fn is_satisfied(dbm: &Self, i: usize, j: usize, val: T);

    fn close(dbm: &mut Self);

    fn future(dbm: &mut Self);
    fn past(dbm: &mut Self);
    fn restrict(dbm: &mut Self, i: usize, j: usize, constant: T);
    fn free(dbm: &mut Self, clock: usize);
    fn assign(dbm: &mut Self, clock: usize, constant: T);
    fn copy(dbm: &mut Self, clock_to: usize, clock_from: usize);
    fn shift(dbm: &mut Self, clock: usize, shift_constant: T);
}

impl DBM<i32> for UDBM {
    fn init(dim: usize) -> UDBM {
        return udbm::init(dim);
    }

    fn zero(dim: usize) -> UDBM {
        return udbm::zero(dim);
    }

    fn is_included_in(lhs: &UDBM, rhs: &UDBM) -> bool {
        return udbm::is_subset(lhs, rhs);
    }

    fn is_satisfied(dbm: &Self, i: usize, j: usize, val: i32) {}
    fn close(dbm: &mut Self) {}

    fn future(dbm: &mut Self) {}
    fn past(dbm: &mut Self) {}
    fn restrict(dbm: &mut Self, i: usize, j: usize, constant: i32) {}
    fn free(dbm: &mut Self, clock: usize) {}
    fn assign(dbm: &mut Self, clock: usize, constant: i32) {}
    fn copy(dbm: &mut Self, clock_to: usize, clock_from: usize) {}
    fn shift(dbm: &mut Self, clock: usize, shift_constant: i32) {}
}

impl<T: std::default::Default + std::ops::Neg<Output = T> + Zero + Bounded + Clone + Ord> DBM<T>
    for RDBM<T>
{
    fn init(dim: usize) -> RDBM<T> {
        let clocks = (1..dim as u8).collect();
        return rdbm::DBM::new(clocks);
    }

    fn zero(dim: usize) -> RDBM<T> {
        let clocks = (1..dim as u8).collect();
        return rdbm::DBM::zero(clocks);
    }

    fn is_included_in(lhs: &RDBM<T>, rhs: &RDBM<T>) -> bool {
        return rdbm::DBM::is_included_in(lhs, rhs);
    }
    fn is_satisfied(dbm: &Self, i: usize, j: usize, val: T) {}

    fn close(dbm: &mut Self) {}

    fn future(dbm: &mut Self) {}
    fn past(dbm: &mut Self) {}
    fn restrict(dbm: &mut Self, i: usize, j: usize, constant: T) {}
    fn free(dbm: &mut Self, clock: usize) {}
    fn assign(dbm: &mut Self, clock: usize, constant: T) {}
    fn copy(dbm: &mut Self, clock_to: usize, clock_from: usize) {}
    fn shift(dbm: &mut Self, clock: usize, shift_constant: T) {}
}

#[cfg(test)]
macro_rules! generate_tests { //Eli Bendersky came up with this approach, and I think it is really ingenious.
    ($($name:ident: $type:ty,)*) => {
        $(
            mod $name {
                use super::*;

                #[test]
                fn init_test() {
                    let x = <$type>::init(3);
                }

                #[test]
                fn relation_test() {
                    let x = <$type>::init(3);
                    let y = <$type>::init(3);
                    assert_eq!(<$type>::is_included_in(&x, &y), <$type>::is_included_in(&y,&x));
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::{DBM, RDBM, UDBM};
    generate_tests! {
        udbm: UDBM,
        rdbm: RDBM<i8>,
    }
}
