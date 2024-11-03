use std::ops::Deref;

pub struct Variable<T> {
    test: T,
    live: T,
}

pub const fn variable<T>(test: T, live: T) -> Variable<T> {
    Variable { test, live }
}

impl<T> Deref for Variable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let is_test = unsafe { crate::TEST };
        if is_test {
            &self.test
        } else {
            &self.live
        }
    }
}

impl<T: std::cmp::PartialEq> PartialEq<T> for Variable<T> {
    fn eq(&self, other: &T) -> bool {
        self.deref() == other
    }
}

impl<T: std::cmp::PartialOrd> PartialOrd<T> for Variable<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.deref().partial_cmp(other)
    }
}

// impl<T: std::cmp::Ord> Ord for Variable<T> {
//     fn cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
//         self.deref().cmp(other)
//     }
// }
