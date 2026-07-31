use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct Comment {
    pub comment: String,
    pub post_hash: ActionHash,
}
