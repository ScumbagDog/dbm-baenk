pub use self::udbm::DBM as UDBM;
use crate::DBM;
use udbm_rs::udbm;

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
