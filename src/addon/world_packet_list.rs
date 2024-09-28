use std::hash::Hash;

use super::AddonVersion;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
pub struct InUse {
    pub pack_id: String,
    pub version: AddonVersion,
}

pub fn parse_in_use_packet_list<S: AsRef<str>>(str: S) -> Result<Vec<InUse>, serde_json::Error> {
    serde_json::from_str(str.as_ref())
}

pub fn to_packet_list_string<V: AsRef<Vec<InUse>>>(list: V) -> Result<String, serde_json::Error> {
    serde_json::to_string(list.as_ref())
}

#[derive(Serialize, Deserialize)]
pub struct History {
    pub can_be_redownloaded: bool,
    pub name: String,
    pub uuin: String,
    pub version: AddonVersion,
}

#[derive(Serialize, Deserialize)]
pub struct HistoryList {
    pub packs: Vec<History>,
}

pub fn pares_packs_history_list<S: AsRef<str>>(str: S) -> Result<HistoryList, serde_json::Error> {
    serde_json::from_str(str.as_ref())
}

pub fn to_packs_history_list_string<V: AsRef<HistoryList>>(
    val: V,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(val.as_ref())
}
