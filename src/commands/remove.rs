use anyhow::{anyhow, Context, Result};
use std::{
    fs::{self, DirEntry},
    path::Path,
};

use crate::addon::{
    self,
    manifest::{Manifest, PackateType},
    world_packet_list::{parse_in_use_packet_list, to_packet_list_string, InUse},
};

fn get_list<P: AsRef<Path>>(
    packate_type: PackateType,
    target: P,
) -> Result<Vec<(String, InUse, DirEntry)>> {
    let list = parse_in_use_packet_list(fs::read_to_string(
        target.as_ref().join(packate_type.get_list_file_string()),
    )?)?;
    let mut out_list = Vec::<(String, InUse, DirEntry)>::new();
    for i in target
        .as_ref()
        .join(packate_type.get_path_name())
        .read_dir()?
        .flatten()
    {
        if let Ok(typ) = i.file_type() {
            if typ.is_dir() {
                let data = Manifest::new(fs::read_to_string(i.path().join("manifest.json"))?)?;
                if let Some(inuse) = list
                    .iter()
                    .find(|i| i.pack_id == data.header.uuid && i.version == data.header.version)
                {
                    out_list.push((data.header.name, inuse.clone(), i));
                }
            }
        }
    }
    Ok(out_list)
}

fn remove_form_list_file<P: AsRef<Path>>(
    target: P,
    info: InUse,
    packate_type: PackateType,
) -> Result<()> {
    let target = target.as_ref();
    let packet_list = if target.join(packate_type.get_list_file_string()).exists() {
        addon::world_packet_list::parse_in_use_packet_list(
            fs::read_to_string(target.join(packate_type.get_list_file_string())).with_context(
                || format!("Failed to read {}", packate_type.get_list_file_string()),
            )?,
        )
        .with_context(|| {
            format!(
                "Failed to properly parse an existing {}",
                packate_type.get_list_file_string()
            )
        })?
    } else {
        return Err(anyhow!(
            "{} not exists",
            packate_type.get_list_file_string()
        ));
    };

    fs::write(
        target.join(packate_type.get_list_file_string()),
        to_packet_list_string(
            packet_list
                .into_iter()
                .filter(|i| !(*i == info))
                .collect::<Vec<_>>(),
        )?,
    )?;
    Ok(())
}

pub fn remove<S: AsRef<str>, P: AsRef<Path>>(name: S, all: bool, target: P) -> Result<()> {
    let bp_list = get_list(PackateType::Behavior, &target)
        .with_context(|| "When reading the behavior packs")?;
    let rp_list = get_list(PackateType::Resource, &target)
        .with_context(|| "When reading the resource packs")?;
    // by uuid
    if let Some(res) = bp_list
        .as_slice()
        .into_iter()
        .find(|i| i.1.pack_id == name.as_ref())
    {
        fs::remove_dir_all(res.2.path())?;
        remove_form_list_file(&target, res.1.clone(), PackateType::Behavior)?;
        println!("Package {} was successfully removed", res.0);
        return Ok(());
    } else if let Some(res) = rp_list
        .as_slice()
        .into_iter()
        .find(|i| i.1.pack_id == name.as_ref())
    {
        fs::remove_dir_all(res.2.path())?;
        remove_form_list_file(&target, res.1.clone(), PackateType::Resource)?;
        println!("Package {} was successfully removed", res.0);
        return Ok(());
    }
    // by name
    let bp_res = bp_list
        .into_iter()
        .filter(|i| i.0 == name.as_ref())
        .collect::<Vec<_>>();
    let rp_res = rp_list
        .into_iter()
        .filter(|i| i.0 == name.as_ref())
        .collect::<Vec<_>>();
    if bp_res.len() > 1 || rp_res.len() > 1 {
        return Err(anyhow!("To find multiple matches, please use uuid"));
    }
    if bp_res.len() == 1 && rp_res.len() == 1 {
        if !all {
            return Err(anyhow!(
                "There are behavior packages and resource packages with the same name,\
             if you need to uninstall them separately, please use uuid."
            ));
        }
        fs::remove_dir_all(bp_res[0].2.path())?;
        remove_form_list_file(&target, bp_res[0].1.clone(), PackateType::Behavior)?;
        fs::remove_dir_all(rp_res[0].2.path())?;
        remove_form_list_file(&target, rp_res[0].1.clone(), PackateType::Resource)?;

        println!("Package {} was successfully removed", bp_res[0].0);
        return Ok(());
    }
    let (res, packate_type) = if bp_res.len() == 1 {
        (&bp_res[0], PackateType::Behavior)
    } else if rp_res.len() == 1 {
        (&rp_res[0], PackateType::Resource)
    } else {
        return Err(anyhow!("No matching packages found"));
    };
    fs::remove_dir_all(res.2.path())?;
    remove_form_list_file(&target, res.1.clone(), packate_type)?;
    println!("Package {} was successfully removed", res.0);

    Ok(())
}
