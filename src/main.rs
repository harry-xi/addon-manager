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
#[command(args_conflicts_with_subcommands = true)]
enum Commands {
    /// list all installed addon(s). Default will show all in-use resource and behavior packages.
    List {
        #[arg(short, long, default_value_t = String::from("Bedrock level") )]
        /// Declare the name of the world you want to operate on. Invalid when the work path is a level.
        world: String,
        #[arg(long)]
        force_dirtype: Option<DirTypeFlag>,
        /// show resource packages
        #[arg(short, long)]
        resource: bool,
        /// show behavior packages
        #[arg(short, long)]
        behavior: bool,
    },
    /// install addon to the level.
    Install {
        #[arg(short, long, default_value_t = String::from("Bedrock level") )]
        /// Declare the name of the world you want to operate on. Invalid when the work path is a level.
        world: String,
        #[arg(long)]
        force_dirtype: Option<DirTypeFlag>,
        /// The addon to be installed, or when using flag '--dir', the palce you put all addon you want to install.
        file: PathBuf,
        /// Treat the input as a folder where packages to be installed are stored.
        #[arg(long)]
        dir: bool,
    },
    /// Uninstall the addon to install to the level.
    Remove {
        #[arg(short, long, default_value_t = String::from("Bedrock level") )]
        /// Declare the name of the world you want to operate on. Invalid when the work path is a level.
        world: String,
        #[arg(long)]
        force_dirtype: Option<DirTypeFlag>,
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

fn get_world_path(world:&str,force_dirtype: Option<DirTypeFlag>) -> Result<PathBuf>{
    let workdir = std::env::current_dir()?;
    let work_dir_type = if let Some(typ) = force_dirtype {
        typ.into()
    } else {
        get_work_path_type(&workdir)?
    };

    if work_dir_type == WorkDirType::Bds && !workdir.join("worlds").join(world).exists() {
        return Err(anyhow!("world {} not exists", world));
    }
    Ok(match work_dir_type {
        WorkDirType::Bds => workdir.join("worlds").join(world),
        WorkDirType::Level => workdir,
    })
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

    match args.command {
        None => {
            if let Some(file) = args.file {
                install::install(file, get_world_path(&args.world,args.force_dirtype)?)?;
            }
            // args.file.is_none() && args.command.is_none() (only use command it self) is at start of this function
        }
        Some(Commands::List {world, force_dirtype, resource, behavior  }) => list::list(get_world_path(&world,force_dirtype)?, resource, behavior)?,
        Some(Commands::Install {world, force_dirtype, file, dir: false }) => install::install(file, get_world_path(&world,force_dirtype)?)?,
        Some(Commands::Install {world, force_dirtype, file, dir: true }) => install::install_all(file, get_world_path(&world,force_dirtype)?)?,
        Some(Commands::Remove {world, force_dirtype, name, all }) => remove::remove(name, all, get_world_path(&world,force_dirtype)?)?,
    }
    Ok(())
}
