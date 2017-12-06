use core::slice::{Iter, IterMut};
use core::{borrow, cmp, default, fmt, hash};

#[derive(Copy, Clone)]
pub struct Tile(pub [u8; 64]);

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0[..], f)
    }
}

impl<'a> IntoIterator for &'a Tile {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;

    fn into_iter(self) -> Iter<'a, u8> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Tile {
    type Item = &'a mut u8;
    type IntoIter = IterMut<'a, u8>;

    fn into_iter(self) -> IterMut<'a, u8> {
        self.0.iter_mut()
    }
}

impl cmp::PartialEq for Tile {
    fn eq(&self, rhs: &Self) -> bool {
        cmp::PartialEq::eq(&self.0[..], &rhs.0[..])
    }
}

impl cmp::Eq for Tile {}

impl hash::Hash for Tile {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.0[..], state)
    }
}

impl AsRef<[u8]> for Tile {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Tile {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl borrow::Borrow<[u8]> for Tile {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl borrow::BorrowMut<[u8]> for Tile {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl default::Default for Tile {
    fn default() -> Self {
        Tile([0; 64])
    }
}