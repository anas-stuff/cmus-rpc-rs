use crate::args::cli_options::CliOptions;
mod args;

fn main() {
    let cli_options = CliOptions::new(args::get_args_app().get_matches());

    
}
