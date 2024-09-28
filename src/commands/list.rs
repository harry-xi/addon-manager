use crate::addon::{self, manifest::Manifest, world_packet_list::*};
use addon::manifest::PackateType;
use anyhow::Result;
use prettytable::{
    format::{self, LineSeparator},
    table,
};
use std::{fs, path::Path};

fn print_list<P: AsRef<Path>>(list_type: PackateType, target: P) -> Result<()> {
    let target = target.as_ref();
    let list = parse_in_use_packet_list(fs::read_to_string(
        target.join(list_type.get_list_file_string()),
    )?)?;
    for i in target.join(list_type.get_path_name()).read_dir()?.flatten() {
        if let Ok(typ) = i.file_type() {
            if typ.is_dir() {
                let data = Manifest::new(fs::read_to_string(i.path().join("manifest.json"))?)?;
                if list
                    .iter()
                    .any(|i| i.pack_id == data.header.uuid && i.version == data.header.version)
                {
                    let mut tab = table!(
                        [Fm->"name", Fb->data.header.name],
                        [Fm->"version", Fc->data.header.version],
                        [Fm->"uuid", Fy->data.header.uuid],
                        [Fm->"type", Fc->list_type.get_path_name().replace("_", " ")],
                        [
                            Fm->"description",
                            if let Some(a) = data.header.description {
                                a
                            } else {
                                "".to_string()
                            }
                        ]
                    );
                    tab.set_format(
                        format::FormatBuilder::new()
                            .column_separator('|')
                            .padding(1, 1)
                            .separator(
                                format::LinePosition::Bottom,
                                LineSeparator::new(' ', ' ', ' ', ' '),
                            )
                            .build(),
                    );
                    tab.printstd()
                }
            }
        }
    }
    Ok(())
}

pub fn list<P: AsRef<Path>>(target: P, resource: bool, behavior: bool) -> Result<()> {
    let (res, beh) = match (resource, behavior) {
        (false, false) => (true, true),
        a => a,
    };
    let target = target.as_ref();
    if target
        .join(PackateType::Resource.get_list_file_string())
        .exists()
        && res
    {
        print_list(PackateType::Resource, target)?;
    }
    if target
        .join(PackateType::Behavior.get_list_file_string())
        .exists()
        && beh
    {
        print_list(PackateType::Behavior, target)?;
    }
    Ok(())
}
