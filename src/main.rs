use clap::Parser;

/// Evaluate datasets with given reactions
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File with method's data
    #[arg(long)]
    data: std::path::PathBuf,

    /// File with database
    #[arg(long)]
    dbfile: Option<std::path::PathBuf>,

    /// DB name to compute
    #[arg(long)]
    compute: Option<String>,

    /// Compute with uncertainty comes from reference data
    #[arg(long, default_value_t = false)]
    with_uncertainty: bool,
}

fn main() {
    let args = Args::parse();
    if args.compute.is_none() && args.dbfile.is_none() {
        println!("Nothing to compute! Please, provide --compute or --dbfile option!");
        return ();
    }
    if args.compute.is_some() {
        panic!("NOT IMPLEMENTED");
    }
    if args.dbfile.is_some() {
        panic!("NOT IMPLEMENTED");
    }
}
