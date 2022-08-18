use std::path::PathBuf;

use clap::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("repo-src")
        .required(true)
        .multiple(true)
        .args(&["token", "repos-file"])
))]
pub struct ClArgs {
    #[clap(short, long, default_value_t = 3u16, value_parser = clap::value_parser!(u16).range(1..100))]
    pub n: u16,
    #[clap(short, long, value_parser, env = "GH_TOKEN")]
    pub token: Option<String>,
    #[clap(short = 'f', long = "file", value_parser)]
    pub repos_file: Option<PathBuf>,
}
