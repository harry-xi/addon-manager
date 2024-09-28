use super::AddonVersion;
use serde::{Deserialize, Serialize};
pub mod version;

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub description: Option<String>,
    pub name: String,
    pub uuid: String,
    // The docs say it might be SemVer String here, maybe it needs to be supported, note that all `version` fields.
    pub version: AddonVersion,
    // and more ...
}
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    #[serde(rename = "resources")]
    Resources,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "world_template")]
    WorldTemplate,
    #[serde(rename = "script")]
    Script,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    #[serde(rename = "type")]
    pub modles_type: ModuleType, // and more ...
}

#[derive(Serialize, Deserialize)]
pub struct Dependencie {
    pub uuid: Option<String>,
    pub module_name: Option<String>,
    pub version: AddonVersion,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub url: Option<String>, // and more ...
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub header: Header,
    pub modules: Vec<Module>,
    pub dependencies: Option<Vec<Dependencie>>,
    pub matedata: Option<Metadata>, // and more ...
}

impl Manifest {
    pub fn new<S: AsRef<str>>(str: S) -> Result<Manifest, serde_jsonc::Error> {
        serde_jsonc::from_str(str.as_ref())
    }

    pub fn get_type(&self) -> Vec<ModuleType> {
        self.modules.iter().map(|i| i.modles_type).collect()
    }

    #[inline]
    pub fn is_behavior_pack(&self) -> bool {
        self.get_type().contains(&ModuleType::Data) || self.get_type().contains(&ModuleType::Script)
    }

    #[inline]
    pub fn is_resource_pack(&self) -> bool {
        self.get_type().contains(&ModuleType::Resources)
    }
}

pub enum PackateType {
    Behavior,
    Resource,
}

#[derive(thiserror::Error, Debug)]
#[error("packet is world template")]
pub struct IsWorldTemplate {}

impl TryFrom<&Manifest> for PackateType {
    type Error = IsWorldTemplate;
    fn try_from(value: &Manifest) -> Result<Self, Self::Error> {
        if value.is_behavior_pack() {
            Ok(PackateType::Behavior)
        } else if value.is_resource_pack() {
            Ok(PackateType::Resource)
        } else {
            Err(IsWorldTemplate {})
        }
    }
}

impl PackateType {
    pub fn get_list_file_string(&self) -> &str {
        match self {
            PackateType::Behavior => "world_behavior_packs.json",
            PackateType::Resource => "world_resource_packs.json",
        }
    }
    pub fn get_path_name(&self) -> &str {
        match self {
            PackateType::Behavior => "behavior_packs",
            PackateType::Resource => "resource_packs",
        }
    }
}
