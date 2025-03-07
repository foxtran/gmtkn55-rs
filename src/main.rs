mod db;
mod method;
mod reaction;
mod units;

use clap::Parser;
use method::Method;

/// Evaluate datasets with given reactions
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// File with method's data
    #[arg(long)]
    data: std::path::PathBuf,

    /// directory with databases
    #[arg(long)]
    dbdir: std::path::PathBuf,

    /// database name
    #[arg(long)]
    db: Option<String>,

    /// DB name to compute
    #[arg(long)]
    compute: Option<String>,

    /// Compute with uncertainty comes from reference data
    #[arg(long, default_value_t = false)]
    with_uncertainty: bool,
}

fn main() {
    let args = Args::parse();
    if args.compute.is_none() && args.db.is_none() {
        println!("Nothing to compute! Please, provide --compute or --db option!");
        return ();
    }
    let method_data: Method =
        Method::new(&args.data).expect("Method's data is not provided properly!");
    if method_data.data.len() == 0 {
        panic!(
            "File '{}' contains nothing!",
            method_data.filepath.display()
        );
    }
    if args.compute.is_some() {
        panic!("NOT IMPLEMENTED");
    }
    if args.db.is_some() {
        let dbpath = args.dbdir.join(args.db.clone().unwrap() + ".csv");
        let db = db::Database::new(
            &dbpath,
            &args.db.unwrap(),
            &String::from(""),
            &String::from(""),
            &String::from(""),
        )
        .expect("DB's data is not provided properly!");
        let res_diff = db.compute_diff(
            &|key: &str, unit: &str| method_data.get_energy(key, unit),
            Some(args.with_uncertainty),
        );
        let res_stat = db.compute_stat(
            &|key: &str, unit: &str| method_data.get_energy(key, unit),
            Some(args.with_uncertainty),
        );
        res_diff
            .iter()
            .for_each(|x| println!("{:<4} {:8.2}", x.0, x.1));
        println!();
        res_stat
            .iter()
            .for_each(|x| println!("{:<4} {:8.2}", x.0, x.1));
    }
}
