use crate::{Curve, Error};

pub trait HashToCurve: Curve {
    fn hash_to_curve(ctx: Tag, msgs: &[&[u8]]) -> Result<Self::Point, Error>;
    fn hash_to_scalar(ctx: Tag, msgs: &[&[u8]]) -> Result<Self::Scalar, Error>;
}

/// Domain separation tag
///
/// DST is a unique identifier of the protocol in which hash to curve primitive is used.
/// It must satisfy a set of requirements defined in the [hast to curve draft], we only
/// enforce the mandatory requirement that DST must be non-empty. Refer to the spec
/// to see full list of requirements.
///
/// [hast to curve draft]: https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-16.html#name-domain-separation-requireme
#[derive(Clone, Copy)]
pub struct Tag<'s>(&'s [u8]);

impl<'s> Tag<'s> {
    /// Constructs a tag from non-empty bytestring `tag`
    ///
    /// ## Panics
    /// Panics if `tag` is empty
    pub const fn new_unwrap(tag: &'s [u8]) -> Self {
        match Self::new(tag) {
            Some(tag) => tag,
            None => panic!("tag must not be empty"),
        }
    }

    /// Tries to constructs a tag from bytestring `tag`
    ///
    /// Returns `None` if `tag` is empty
    pub const fn new(tag: &'s [u8]) -> Option<Self> {
        if tag.is_empty() {
            None
        } else {
            Some(Self(tag))
        }
    }

    /// Bytestring corresponding to the tag
    pub fn as_bytes(&self) -> &'s [u8] {
        self.0
    }
}
