use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct Post {
    pub title: String,
    pub content: String,
}
