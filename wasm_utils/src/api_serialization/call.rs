use holochain_core_types::{dna::capabilities::CapabilityCall, error::HolochainError, json::*};

/// Struct for input data received when Zome API function call() is invoked
#[derive(Deserialize, Default, Clone, PartialEq, Eq, Hash, Debug, Serialize, DefaultJson)]
pub struct ZomeFnCallArgs {
    pub zome_name: String,
    pub cap: Option<CapabilityCall>,
    pub fn_name: String,
    pub fn_args: String,
}
