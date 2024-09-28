use crate::addon::manifest::PackateType;
use crate::addon::world_packet_list::{to_packet_list_string, InUse};
use crate::addon::{self, manifest};
use anyhow::{anyhow, Context, Result};
use copy_dir::copy_dir;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use tempfile::tempdir;
use zip::ZipArchive;

fn install_single_pack<P: AsRef<Path>>(dir: &fs::DirEntry, target: P) -> Result<()> {
    let target = target.as_ref();
    let package_manifest = dir.path().join("manifest.json");
    if package_manifest.exists() {
        let manifest_info = manifest::Manifest::new(
            fs::read_to_string(package_manifest).with_context(|| "file to read manifest")?,
        )
        .with_context(|| "Failed to successfully read package manifest.json information")?;
        let packate_type = PackateType::try_from(&manifest_info)
            .with_context(|| "Installation of world_template is not supported")?;
        let mut packetlist = if target.join(packate_type.get_list_file_string()).exists() {
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
            Vec::new()
        };
        match packetlist
            .iter()
            .find(|i| i.pack_id == manifest_info.header.uuid)
        {
            Some(i) if i.version == manifest_info.header.version => println!(
                "addon {} [version: {}] already exists, skip installation",
                manifest_info.header.name,
                manifest_info.header.version.to_string()
            ),

            Some(i) if i.version > manifest_info.header.version => println!(
                "A newer version of addon {} [version: {}] already exists.\
                    The installation of the current version {} has been skipped.",
                manifest_info.header.name,
                i.version.to_string(),
                manifest_info.header.version.to_string()
            ),

            None | Some(_) => {
                let targe_dir = target
                    .join(packate_type.get_path_name())
                    .join(&manifest_info.header.name);
                if targe_dir.exists() {
                    fs::remove_dir(&targe_dir)?;
                }
                match fs::create_dir_all(target.join(packate_type.get_path_name())) {
                    Ok(_) => (),
                    Err(err) => {
                        if err.kind() != io::ErrorKind::AlreadyExists {
                            return Err(err.into());
                        }
                    }
                }
                copy_dir(dir.path(), targe_dir).with_context(|| "while copy")?;
                let version_str = manifest_info.header.version.to_string();
                packetlist.push(InUse {
                    pack_id: manifest_info.header.uuid,
                    version: manifest_info.header.version,
                });
                fs::write(
                    target.join(packate_type.get_list_file_string()),
                    to_packet_list_string(packetlist)?,
                )?;
                println!(
                    "success to install {} [{}]",
                    manifest_info.header.name, version_str
                )
            }
        }
    }
    Ok(())
}

fn install_mcaddon<P: AsRef<Path>>(mut addon: ZipArchive<fs::File>, target: P) -> Result<()> {
    let target = target.as_ref();
    let temp_dir = tempdir().with_context(|| "fail to create temp dir")?;
    let temp_path = temp_dir.path();
    addon
        .extract(&temp_path)
        .with_context(|| "fail to extract the zip file")?;
    for i in temp_path
        .read_dir()
        .with_context(|| "file to read the temp dir")?
        .flatten()
        .filter(|i| {
            if let Ok(filetype) = i.file_type() {
                filetype.is_dir()
            } else {
                false
            }
        })
    {
        install_single_pack(&i, target)
            .with_context(|| format!("fail to install {}", i.file_name().to_string_lossy()))?;
    }
    Ok(())
}

pub fn install<P: AsRef<Path>, P1: AsRef<Path>>(addon: P, target: P1) -> Result<()> {
    let target = target.as_ref();

    let filename = addon.as_ref().to_string_lossy();

    let file = fs::File::open(&addon)
        .with_context(|| format!("Failed to open addon file {}", filename))?;
    let mut archive = zip::ZipArchive::new(file)
        .with_context(|| format!("Unable to read zip file {}", filename))?;

    if filename.ends_with(".mcaddon") {
        return install_mcaddon(archive, target);
    }

    let mut manifest = String::new();
    if let Ok(mut file) = archive.by_name("manifest.json") {
        file.read_to_string(&mut manifest)
            .with_context(|| "Failed to read the manifest.json")?;
    } else {
        return Err(anyhow!("Failed to find manifest.json"));
    }
    let data = manifest::Manifest::new(manifest)
        .with_context(|| "Failed to successfully read package manifest.json information")?;

    let packate_type = PackateType::try_from(&data)
        .with_context(|| "Installation of world_template is not supported")?;

    let mut packetlist = if target.join(packate_type.get_list_file_string()).exists() {
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
        Vec::new()
    };

    match packetlist.iter().find(|i| i.pack_id == data.header.uuid) {
        Some(i) if i.version == data.header.version => println!(
            "addon {} [version: {}] already exists, skip installation",
            data.header.name,
            data.header.version.to_string()
        ),

        Some(i) if i.version > data.header.version => println!(
            "A newer version of addon {} [version: {}] already exists.\
                The installation of the current version {} has been skipped.",
            data.header.name,
            i.version.to_string(),
            data.header.version.to_string()
        ),

        None | Some(_) => {
            archive
                .extract(
                    target
                        .join(packate_type.get_path_name())
                        .join(&data.header.name),
                )
                .with_context(|| {
                    format!(
                        "Failed to extract addon {} to target folder",
                        &data.header.name
                    )
                })?;
            let version_str = data.header.version.to_string();
            packetlist.push(InUse {
                pack_id: data.header.uuid,
                version: data.header.version,
            });
            fs::write(
                target.join(packate_type.get_list_file_string()),
                to_packet_list_string(packetlist)?,
            )?;
            println!("success to install {} [{}]", data.header.name, version_str)
        }
    }
    Ok(())
}
