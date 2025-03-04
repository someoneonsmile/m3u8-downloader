use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
