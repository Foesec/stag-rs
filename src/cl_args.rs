use clap::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct ClArgs {
    #[clap(short, long, value_parser)]
    pub token: Option<String>,
    #[clap(short, long, default_value_t = 3u16, value_parser = clap::value_parser!(u16).range(1..100))]
    pub n: u16,
    #[clap(short='f', long="file", value_parser)]
    pub repos_file: Option<String>
}
