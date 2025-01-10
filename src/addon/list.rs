use super::world_packet_list::{
    pares_packs_history_list, parse_in_use_packet_list, to_packet_list_string,
    to_packs_history_list_string, History, InUse,
};

pub struct PacketList {
    pub inuse: Vec<InUse>,
    pub installed: Vec<History>,
}

pub struct WorldPacketList {
    pub behavior: PacketList,
    pub resource: PacketList,
}

// impl WorldPacketList {
//     pub fn new<S: AsRef<str>, S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(
//         bp_inuse: S,
//         bp_his: S1,
//         rp_inuse: S2,
//         rp_his: S3,
//     ) {
//     }
// }

#[derive(thiserror::Error, Debug)]
#[error("the histore packet list didn't incloud all inuse package")]
pub struct PacketListUnMatch {}

impl PacketList {
    pub fn parse<S: AsRef<str>, S1: AsRef<str>>(
        inuse: S,
        histore: S1,
    ) -> Result<PacketList, serde_json::Error> {
        Ok(PacketList {
            inuse: parse_in_use_packet_list(inuse)?,
            installed: pares_packs_history_list(histore)?.packs,
        })
    }

    pub fn get_list_file_string(self: &Self) -> Result<String, serde_json::Error> {
        to_packet_list_string(&self.inuse)
    }
    pub fn get_history_list_file_string(self: &Self) -> Result<String, serde_json::Error> {
        to_packs_history_list_string(&self.installed)
    }

    pub fn check(self: &Self) -> Result<(), PacketListUnMatch> {
        if (&self.inuse)
            .into_iter()
            .all(|i| (&self.installed).into_iter().any(|h| i.pack_id == h.uuid))
        {
            Ok(())
        } else {
            Err(PacketListUnMatch {})
        }
    }
}
