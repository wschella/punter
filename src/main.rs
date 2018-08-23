#![feature(transpose_result)]

#[macro_use]
extern crate failure;
extern crate dirs;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use std::path::PathBuf;

use failure::{ensure, Error, Fail};
use structopt::StructOpt;

/// Manage your dotfiles
#[derive(Debug, StructOpt)]
struct CliConfig {
    #[structopt(subcommand)]
    command: SubCommand,

    /// Punter config location [default: dotter.toml]
    #[structopt(long = "config", short = "c", parse(from_os_str))]
    config: Option<PathBuf>,

    // TODO: Replace with silent option
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

#[derive(Deserialize, Debug)]
struct FileConfig {
    verbosity: Option<u8>,
    sync: SyncConfig,
}

struct Context {
    verbosity: u8,
    command: SubCommand,
    links: Vec<Link>,
}

struct Link {}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// Synchronize your dotfiles
    #[structopt(name = "sync")]
    SyncConfig(SyncConfig),
}

#[derive(Deserialize, StructOpt, Debug)]
struct SyncConfig {
    /// Dotfile source directory [default: .]
    #[structopt(long = "src", short = "s", parse(from_os_str))]
    src: Option<PathBuf>,

    /// Dotfile destination directory [default: your home folder]
    #[structopt(long = "dest", short = "d", parse(from_os_str))]
    dest: Option<PathBuf>,
}

struct SyncOp {
    src: PathBuf,
    dest: PathBuf,
}

fn main() -> Result<(), Error> {
    let cli_args = CliConfig::from_args();
    let file_args = cli_args
        .config
        .clone()
        .and_then(|p| Some(FileConfig::from_path(p)))
        .or(FileConfig::default())
        .transpose()?;
    let merged_args = merge_args(cli_args, file_args);

    println!("{:?}", merged_args);
    match merged_args.command {
        SubCommand::SyncConfig(args) => {
            let src = args.src.unwrap_or(PathBuf::from("."));
            let dest = args.dest.or(dirs::home_dir()).ok_or(SyncError::NoDestination)?;
            ensure!(src.is_dir(), SyncError::SourceNotADirectory(src));
            ensure!(dest.is_dir(), SyncError::DestinationNotADirectory(dest));

            sync(SyncOp { src, dest })
        }
    }
}

fn merge_args<'a>(mut cli_config: CliConfig, mut file_config_o: Option<FileConfig>) -> CliConfig {
    if file_config_o.is_none() {
        return cli_config;
    }
    let mut file_config = file_config_o.unwrap();

    cli_config.verbosity = 0;
    file_config.verbosity = Some(0);
    cli_config
}

impl FileConfig {
    pub fn from_path(path: PathBuf) -> Result<FileConfig, Error> {
        ensure!(path.is_file(), SyncError::InvalidConfigPath(path));
        let content = fs::read_to_string(path)?;
        let toml = toml::from_str(&content).map_err(|e| SyncError::InvalidConfig(e))?;
        Ok(toml)
    }

    pub fn default() -> Option<Result<FileConfig, Error>> {
        let path = PathBuf::from("punter.toml");
        if path.is_file() {
            Some(FileConfig::from_path(path))
        } else {
            None
        }
    }
}

fn sync(op: SyncOp) -> Result<(), Error> {
    for entry in fs::read_dir(op.src)? {
        println!("{:?}", entry?.path());
    }
    Ok(())
}

#[derive(Debug, Fail)]
enum SyncError {
    #[fail(display = "Home folder could not be found and no destination given")]
    NoDestination,

    #[fail(display = "Source with path {:?} is not a directory", _0)]
    SourceNotADirectory(PathBuf),

    #[fail(display = "Destination with path {:?} is not a directory", _0)]
    DestinationNotADirectory(PathBuf),

    #[fail(display = "Invalid config path {:?}", _0)]
    InvalidConfigPath(PathBuf),

    #[fail(display = "Invalid config: ")]
    InvalidConfig(toml::de::Error),
}
