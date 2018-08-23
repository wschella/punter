#[macro_use]
extern crate failure;
extern crate dirs;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use failure::{ensure, Error, Fail};
use structopt::StructOpt;
use toml::Value;

fn main() -> Result<(), Error> {
  let args = Cli::from_args();
  let base = args.path.clone().unwrap_or(".".into()); // NLL Clone
  let config_path = base.join("punter.toml");
  let config = FileConfig::from_path(&config_path)?;

  println!("{:?}", &args);
  println!("{:?}", read_loose(&config_path));
  println!("{:?}", &config);

  let cli_command = args.command.clone(); // NLL CLone;
  let context = (base, args, config);
  let command = match cli_command {
    CliCommand::Sync => SyncCommand::new(context),
  };

  let actions = command.prepare()?;

  for action in actions {
    action.execute()?;
  }

  Ok(())
}

type Base = PathBuf;
type Context = (Base, Cli, FileConfig);
trait Command {
  type A: Action;
  fn new(context: Context) -> Self;
  fn prepare(self) -> Result<Vec<Self::A>, Error>;
}

struct SyncCommand {
  context: Context,
}

impl Command for SyncCommand {
  type A = SyncAction;

  fn new(context: Context) -> Self {
    SyncCommand { context }
  }

  fn prepare(self) -> Result<Vec<SyncAction>, Error> {
    let (base, cli, config) = self.context;
    for entry in fs::read_dir(base)? {
      println!("{:?}", entry?.path());
    }
    Ok(vec![])
  }
}

trait Action {
  fn execute(self) -> Result<(), Error>;
}

struct SyncAction;

impl Action for SyncAction {
  fn execute(self) -> Result<(), Error> {
    Ok(())
  }
}

#[derive(StructOpt, Debug)]
struct Cli {
  #[structopt(subcommand)]
  command: CliCommand,

  /// Specify path for your dotfiles directory
  #[structopt(short = "p", long = "path", parse(from_os_str))]
  path: Option<PathBuf>,
}

#[derive(Debug, StructOpt, Clone)]
enum CliCommand {
  /// Synchronize your dotfiles
  #[structopt(name = "sync")]
  Sync,
}

#[derive(Deserialize, Debug)]
struct FileConfig {
  files: HashMap<String, LinkValue>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum LinkValue {
  Simple(String),
}

impl FileConfig {
  pub fn from_path<P>(path: P) -> Result<Self, Error>
  where
    P: Into<PathBuf>,
  {
    let content = fs::read_to_string(path.into())?;
    let toml = toml::from_str(&content).map_err(|e| FileConfigError::InvalidConfig(e))?;
    Ok(toml)
  }
}

fn read_loose<P>(path: P) -> Result<Value, Error>
where
  P: Into<PathBuf>,
{
  let content = fs::read_to_string(path.into())?;
  let val = content.parse::<Value>()?;
  Ok(val)
}

#[derive(Fail, Debug)]
enum FileConfigError {
  #[fail(display = "Invalid config: {:?}", _0)]
  InvalidConfig(toml::de::Error),
}
