use rdbm::rdbm;
use udbm_rs::udbm;

use num::Bounded;
use num::Zero;

use crate::rdbm::DBM as RDBM;
use udbm::DBM as UDBM; //had some trouble with namespacing in the original repo, and decided to just leave it. Might fix later (probably not)

trait DBM {
    fn init(dim: usize) -> Self;
    fn zero(dim: usize) -> Self;
    fn relation(rhs_dbm: &Self, lhs_dbm: &Self) -> bool;
}

impl DBM for UDBM {
    fn init(dim: usize) -> UDBM {
        return udbm::init(dim);
    }

    fn zero(dim: usize) -> UDBM {
        return udbm::zero(dim);
    }

    fn relation(lhs: &UDBM, rhs: &UDBM) -> bool {
        return udbm::relation(lhs, rhs) > 0;
    }
}

impl<T: std::default::Default + std::ops::Neg<Output = T> + Zero + Bounded + Clone + Ord> DBM
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

    fn relation(lhs: &RDBM<T>, rhs: &RDBM<T>) -> bool {
        return rdbm::DBM::relation(lhs, rhs);
    }
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
                    assert_eq!(<$type>::relation(&x, &y), <$type>::relation(&y,&x));
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
