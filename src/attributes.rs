#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VersionTuple(usize, usize, usize);

impl From<VersionTuple> for (usize, usize, usize) {
    fn from(v: VersionTuple) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<[usize; 3]> for VersionTuple {
    fn from(v: [usize; 3]) -> Self {
        VersionTuple(v[0], v[1], v[2])
    }
}

use std::fmt;
impl fmt::Display for VersionTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2)
    }
}

pub const DEPENDENT_VERSION: VersionTuple = VersionTuple(0, 4, 52);
