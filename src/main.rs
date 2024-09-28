use addon_manager::*;
use anyhow::{anyhow, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use commands::{install, list, remove};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
/// A command line tool for installing and managing addons on bds
struct Cli {
    /// The addon to be installed
    file: Option<PathBuf>,
    #[arg(short, long, default_value_t = String::from("Bedrock level") )]
    /// Declare the name of the world you want to operate on. Invalid when the work path is a level.
    world: String,
    #[arg(long)]
    force_dirtype: Option<DirTypeFlag>,
    // #[arg(long)]
    // force:bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// list all installed addon(s). Default will show all in-use resource and behavior packages.
    List {
        /// show resource packages
        #[arg(short, long)]
        resource: bool,
        /// show behavior packages
        #[arg(short, long)]
        behavior: bool,
    },
    /// install addon to the level.
    Install {
        /// The addon to be installed.
        file: PathBuf,
    },
    /// Uninstall the addon to install to the level.
    Remove {
        /// Name or uuid of the Addon to be uninstalled.
        name: String,
        /// Uninstall both behavior and resource packages with the same name.
        #[arg(long)]
        all: bool,
    },
    // Show {
    //     /// Name or uuid of the Addon to be show the infomation
    //     name: String,
    // }
    // Enable {
    //     /// Name or uuid of the Addon to enable.
    //     name: String,
    //     /// Enable both behavior and resource packages with the same name.
    //     #[arg(long)]
    //     all: bool,
    // },
    // Disable {
    //     /// Name or uuid of the Addon to be disable.
    //     name: String,
    //     /// Disable both behavioral and resource packages with the same name.
    //     #[arg(long)]
    //     all: bool,
    // },
}

#[derive(ValueEnum, Clone)]
enum DirTypeFlag {
    /// Skip checking and treat the working path as the bds root directory
    Bds,
    /// Skip checking and treat the working path as a level
    Level,
}
impl From<DirTypeFlag> for WorkDirType {
    fn from(item: DirTypeFlag) -> WorkDirType {
        match item {
            DirTypeFlag::Bds => WorkDirType::Bds,
            DirTypeFlag::Level => WorkDirType::Level,
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if args.file.is_none() && args.command.is_none() {
        // show help when there are no args
        println!(
            "{}    version:{}",
            Cli::command().get_name(),
            Cli::command().get_version().unwrap()
        );
        Cli::command().print_help()?;
        return Ok(());
    }

    let workdir = std::env::current_dir()?;
    let work_dir_type = if let Some(typ) = args.force_dirtype {
        typ.into()
    } else {
        get_work_path_type(&workdir)?
    };

    if work_dir_type == WorkDirType::Bds && !workdir.join("worlds").join(&args.world).exists() {
        return Err(anyhow!("world {} not exists", &args.world));
    }
    let world_path = match work_dir_type {
        WorkDirType::Bds => workdir.join("worlds").join(&args.world),
        WorkDirType::Level => workdir,
    };

    match args.command {
        None => {
            if let Some(file) = args.file {
                install::install(file, world_path)?;
            }
            // args.file.is_none() && args.command.is_none() (only use command it self) is at start of this function
        }
        Some(Commands::List { resource, behavior }) => list::list(world_path, resource, behavior)?,
        Some(Commands::Install { file }) => install::install(file, world_path)?,
        Some(Commands::Remove { name, all }) => remove::remove(name, all, world_path)?,
    }
    Ok(())
}
