use crate::cmd::cat_file;
use crate::cmd::init;
use clap::{Parser, Subcommand};
use cmd::hash_object;
use cmd::ls_tree;

pub mod cmd;
pub mod errors;
pub mod types;

#[derive(Debug, Parser)]
#[clap(name = "rit", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Init a new git repository
    Init,
    // cat the contents of a blob
    CatFile {
        /// The object to display.
        #[clap(long, short = 'p')]
        blob_sha: String,
    },
    // Hash a file
    HashObject {
        /// The file to hash.
        #[clap(long, short = 'w')]
        file: String,
    },
    // List a tree object.
    LsTree {
        /// The file to hash
        #[clap(long)]
        name_only: bool,
        tree_sha: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = App::parse();
    match args.command {
        Command::Init => init::init(),
        Command::CatFile { blob_sha } => cat_file::cat_file(&blob_sha),
        Command::HashObject { file } => hash_object::hash_object(&file),
        Command::LsTree {
            tree_sha,
            name_only,
        } => ls_tree::ls_tree(&tree_sha, name_only),
    }
}
