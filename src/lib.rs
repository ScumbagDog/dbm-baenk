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
    fn is_satisfied(dbm: &Self, i: usize, j: usize, bound_is_strict: bool, constant: T) -> bool;

    fn close(dbm: &mut Self);

    fn future(dbm: &mut Self);
    fn past(dbm: &mut Self);
    fn restrict(dbm: &mut Self, i: usize, j: usize, bound_is_strict: bool, constant: T);
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

    fn is_satisfied(dbm: &Self, i: usize, j: usize, bound_is_strict: bool, constant: i32) -> bool {
        return udbm::satisfies(dbm, i, j, udbm::encode_bound(constant, bound_is_strict));
    }
    fn close(dbm: &mut Self) {
        udbm::close(dbm);
    }

    fn future(dbm: &mut Self) {
        udbm::up(dbm);
    }

    fn past(dbm: &mut Self) {
        udbm::down(dbm);
    }

    fn restrict(dbm: &mut Self, i: usize, j: usize, bound_is_strict: bool, constant: i32) {
        udbm::and(dbm, i, j, udbm::encode_bound(constant, bound_is_strict));
    }

    fn free(dbm: &mut Self, clock: usize) {
        udbm::free(dbm, clock);
    }

    fn assign(dbm: &mut Self, clock: usize, constant: i32) {
        udbm::assign(dbm, clock, constant);
    }

    fn copy(dbm: &mut Self, clock_to: usize, clock_from: usize) {
        udbm::copy(dbm, clock_to, clock_from);
    }

    fn shift(dbm: &mut Self, clock: usize, shift_constant: i32) {
        udbm::shift(dbm, clock, shift_constant);
    }
}

impl<T: std::default::Default + std::ops::Neg<Output = T> + Zero + Bounded + Clone + Ord + num::Saturating> DBM<T>
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
    fn is_satisfied(dbm: &Self, i: usize, j: usize, is_bound_strict: bool, constant: T) -> bool {
        let constraint_op = match is_bound_strict {
            true => rdbm::ConstraintOp::LessThan,
            false => rdbm::ConstraintOp::LessThanEqual,
        };
        return rdbm::DBM::satisfied(dbm, i as u8, j as u8, constraint_op, constant).unwrap();
    }

    fn close(dbm: &mut Self) {
        rdbm::DBM::close(dbm);
    }

    fn future(dbm: &mut Self) {
        rdbm::DBM::up(dbm).unwrap();
    }

    fn past(dbm: &mut Self) {
        rdbm::DBM::down(dbm).unwrap();
    }

    fn restrict(dbm: &mut Self, i: usize, j: usize, bound_is_strict: bool, constant: T) {
        let constraint_op = match bound_is_strict {
            true => rdbm::ConstraintOp::LessThan,
            false => rdbm::ConstraintOp::LessThanEqual,
        };
        rdbm::DBM::and(dbm, i as u8, j as u8, constraint_op, constant).unwrap();
    }

    fn free(dbm: &mut Self, clock: usize) {
        rdbm::DBM::free(dbm, clock as u8).unwrap();
    }

    fn assign(dbm: &mut Self, clock: usize, constant: T) {
        rdbm::DBM::reset(dbm, clock as u8, constant).unwrap();
    }

    fn copy(dbm: &mut Self, clock_to: usize, clock_from: usize) {
        rdbm::DBM::copy(dbm, clock_to as u8, clock_from as u8).unwrap();
    }

    fn shift(dbm: &mut Self, clock: usize, shift_constant: T) {
        rdbm::DBM::shift(dbm, clock as u8, shift_constant).unwrap();
    }
}

#[cfg(test)]
macro_rules! generate_tests { //Eli Bendersky came up with this approach, and I think it is really ingenious.
    ($($name:ident: $type:ty,)*) => {
        $(
            mod $name {
                use super::*;

                #[test]
                fn test_init() {
                    let _ = <$type>::init(3);
                }

                #[test]
                fn test_relation_init() {
                    let x:$type = DBM::init(3);
                    let y:$type = DBM::init(3);
                    assert_eq!(DBM::is_included_in(&x, &y), true);
                    assert_eq!(DBM::is_included_in(&y, &x), true);
                }

                #[test]
                fn test_zero_close() {
                    let dim: usize = 10;
                    let dbm_zero:$type = DBM::zero(dim);
                    let mut dbm_closed:$type = DBM::zero(dim);
                    DBM::close(&mut dbm_closed);

                    assert_eq!(DBM::is_included_in(&dbm_zero, &dbm_closed), true);
                    assert_eq!(DBM::is_included_in(&dbm_closed, &dbm_zero), true);
                }

                #[test]
                fn test_init_close() {
                    let dim: usize = 10;
                    let dbm_init:$type = DBM::init(dim);
                    let mut dbm_closed:$type = DBM::init(dim);
                    DBM::close(&mut dbm_closed);

                    assert_eq!(DBM::is_included_in(&dbm_init, &dbm_closed), true); //as an init'd dbm is already on closed form, no change should happen.
                    assert_eq!(DBM::is_included_in(&dbm_closed, &dbm_init), true); //this is a way to test if the dbm's are equal
                }

                #[test]
                fn test_init_different_from_zero() {
                    let dim: usize = 10;
                    let dbm_init:$type = DBM::init(dim);
                    let dbm_zero:$type = DBM::zero(dim);

                    assert_eq!(DBM::is_included_in(&dbm_zero, &dbm_init), true); //a zero-dbm is included in a dbm whose bounds (except for lower-bounds and the diagonal) are set to infinity, or at least max
                    assert_eq!(DBM::is_included_in(&dbm_init, &dbm_zero), false);
                }


                #[test]
                fn test_zero() {
                    let dim: usize = 10;
                    let mut dbm:$type = DBM::zero(dim);
                    DBM::close(&mut dbm);
                    assert_eq!(DBM::is_satisfied(&dbm, 1, 2, false, 3), false);
                }

                #[test]
                fn test_future_included_in() {
                    let dim: usize = 10;
                    let mut dbm:$type = DBM::zero(dim);
                    let dbm2:$type = DBM::zero(dim);
                    DBM::future(&mut dbm);
                    assert_eq!(DBM::is_included_in(&dbm2, &dbm), true); //a future dbm always includes the present it started from.
                    assert_eq!(DBM::is_included_in(&dbm, &dbm2), false); //but a present does not include a future.
                }

                #[test]
                fn test_restrict() {
                    let dim: usize = 10;
                    let mut dbm:$type = DBM::init(dim);
                    let dbm2:$type = DBM::init(dim);
                    DBM::restrict(&mut dbm, 1, 0, false, 10);
                    assert_eq!(DBM::is_included_in(&dbm, &dbm2), true); //since dbm has been restricted, dbm2 should now include it, but not the other way around.
                    assert_eq!(DBM::is_included_in(&dbm2, &dbm), true);
                }

                #[test]
                fn test_free() {
                    let dim: usize = 10;
                    let mut dbm:$type = DBM::zero(dim);
                    let dbm2:$type = DBM::zero(dim);
                    DBM::free(&mut dbm, 1);
                    assert_eq!(DBM::is_included_in(&dbm2, &dbm), true); //since dbm is freed, it should now include dbm2, while not being included by dbm2 itself.
                    assert_eq!(DBM::is_included_in(&dbm, &dbm2), false);
                }

                #[test]
                fn test_assign() {
                    let dim: usize = 10;
                    let mut dbm:$type = DBM::zero(dim);
                    let dbm2:$type = DBM::zero(dim);
                    DBM::assign(&mut dbm, 1, 10); //set clock 1 to a value of 10
                    assert_eq!(DBM::is_included_in(&dbm, &dbm2), false); //as dbm has clock 1 set to 10, it will not include the zero dbm
                    assert_eq!(DBM::is_included_in(&dbm2, &dbm), false); //likewise, the zero dbm does not include dbm
                }

                #[test]
                fn test_copy() {
                    let dim: usize = 10;
                    let mut dbm:$type = DBM::zero(dim);
                    let dbm2:$type = DBM::zero(dim);
                    DBM::free(&mut dbm, 1);
                    DBM::copy(&mut dbm, 1, 2); //copy clock 2 to clock 1 in dbm (effectively undoing the free)
                    assert_eq!(DBM::is_included_in(&dbm2, &dbm), true); //
                    assert_eq!(DBM::is_included_in(&dbm, &dbm2), true); //
                }

                #[test]
                fn test_shift() {

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
