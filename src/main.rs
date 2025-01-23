mod db;
mod method;
mod reaction;

use clap::Parser;
use method::Method;

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
    if args.dbfile.is_some() {
        let db = db::Database::new(
            &args.dbfile.clone().unwrap(),
            &args.dbfile.unwrap().into_os_string().into_string().unwrap(),
            &String::from(""),
            &String::from(""),
            &String::from(""),
        )
        .expect("DB's data is not provided properly!");
        let res = db.compute_stat(
            move |key: &String| method_data.get_energy(key),
            Some(args.with_uncertainty),
        );
        res.iter().for_each(|x| println!("{:<4} {:8.2}", x.0, x.1));
    }
}
