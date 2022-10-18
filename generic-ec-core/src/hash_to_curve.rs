use crate::{Curve, Error};

pub trait HashToCurve: Curve {
    fn hash_to_curve(ctx: Tag, msgs: &[&[u8]]) -> Result<Self::Point, Error>;
    fn hash_to_scalar(ctx: Tag, msgs: &[&[u8]]) -> Result<Self::Scalar, Error>;
}

/// Domain separation tag
#[derive(Clone, Copy)]
pub struct Tag<'s>(&'s [u8]);

impl<'s> Tag<'s> {
    pub const fn new_unwrap(tag: &'s [u8]) -> Self {
        match Self::new(tag) {
            Some(tag) => tag,
            None => panic!("tag must not be empty"),
        }
    }

    pub const fn new(tag: &'s [u8]) -> Option<Self> {
        if tag.is_empty() {
            None
        } else {
            Some(Self(tag))
        }
    }
}
