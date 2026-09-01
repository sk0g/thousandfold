use crate::internal_prelude::*;

#[extfn]
pub fn as_name(self: &str) -> Name { Name::from(self) }
