pub mod comment;
pub use comment::*;
pub mod post;
use hdi::prelude::*;
pub use post::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Post(Post),
    Comment(Comment),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    PostUpdates,
    PostToComments,
    AllPosts,
}

// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// This fixture app exists to exercise the runtime (install, enable, zome calls,
// signals) in integration tests, not DHT validation — everything is accepted.
// The scaffolded per-op validation boilerplate was dropped in the hdi 0.8 port;
// see the hdi docs for the current validation callback patterns.
#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
