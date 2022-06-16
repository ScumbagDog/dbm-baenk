pub use self::rdbm::DBM as RDBM;
use crate::DBM;
use rdbm::rdbm;

use num::Bounded;
use num::Zero;

impl<T: std::ops::Neg<Output = T> + Zero + Bounded + Clone + Ord + num::Saturating> DBM<T>
    for RDBM<T>
{
    fn init(dim: usize) -> Self {
        return rdbm::DBM::new(dim);
    }

    fn zero(dim: usize) -> Self {
        return rdbm::DBM::zero(dim);
    }

    fn is_included_in(lhs: &Self, rhs: &Self) -> bool {
        return rdbm::DBM::is_included_in(lhs, rhs);
    }
    fn is_satisfied(dbm: &Self, i: usize, j: usize, is_bound_strict: bool, constant: T) -> bool {
        let constraint_op = match is_bound_strict {
            true => rdbm::ConstraintOp::LessThan,
            false => rdbm::ConstraintOp::LessThanEqual,
        };
        return rdbm::DBM::satisfied(dbm, i, j, constraint_op, constant).unwrap();
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
        rdbm::DBM::and(dbm, i, j, constraint_op, constant).unwrap();
    }

    fn free(dbm: &mut Self, clock: usize) {
        rdbm::DBM::free(dbm, clock).unwrap();
    }

    fn assign(dbm: &mut Self, clock: usize, constant: T) {
        rdbm::DBM::reset(dbm, clock, constant).unwrap();
    }

    fn copy(dbm: &mut Self, clock_to: usize, clock_from: usize) {
        rdbm::DBM::copy(dbm, clock_to, clock_from).unwrap();
    }

    fn shift(dbm: &mut Self, clock: usize, shift_constant: T) {
        rdbm::DBM::shift(dbm, clock, shift_constant).unwrap();
    }
}
