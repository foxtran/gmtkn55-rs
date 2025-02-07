# Reproduce GMTKN55 dataset

This repo collects data and scripts for reproducing GMTKN55 and related datasets (mostly from Tim Gould)

## How to use?

1. [Install Rust language](https://www.rust-lang.org/tools/install)
2. Clone this repo via [Git](https://git-scm.com): `git clone https://github.com/foxtran/gmtkn55-rs.git`
3. Go to directory: `cd gmtkn55-rs`
4. Use [cargo](https://doc.rust-lang.org/cargo/) to run: `cargo run --release -- --help`. It will provide some help.
5. To compute something useful, use: `cargo run --release -- --data data/computed/PBE0-Gaussian09revB.csv --dbfile data/databases/NBPRC.csv`. You can check data formats and provide your own data and datasets.

## Dataset notices

- WATER27 dataset has improper reference data in `.res` script. Use reference data from GMTKN55 website.
- Reaction #2 from DC13 uses old value (from DC9) during computing error for, at least, PBE0 and PBEh1PBE functionals, therefore the data is not consistent between officially published and presented here.
- For BH76 and BH76RC datasets, `C2H5` and `c2h5` geometries were renamed to `C2H5_v1` and `c2h5_v2`, respectively, for compability with register-independent filesystems.
- For NBPRC dataset, `H2` and `h2`, `BH3` and `bh3` geometries were renamed to `H2_v1` and `h2_v2`, `BH3_v1` and `bh3_v2`, respectively, for compability with register-independent filesystems.

## Computation notices

### Gaussian

All calculation were computed with Gaussian 09 rev. B. Therefore, data is presented ONLY for dispersionless functionals.

#### PBEh1PBE

`RC21_2p2` was computed with non-standard guess. Used guess is `guess(Huckel)`.

#### PBE0

`RC21_2p2` was computed with non-standard guess. Used guess is `guess(Huckel)`.

## Data collection notices

To collect data, I originally used `.res` scripts with my own implementation of `tmer2_GMTKN` script for converting data formats.
Note, some datasets uses another script inside `.res` scripts and therefore you need to patch them.
Geometries can be nicely collected from ORCA 6.0.1 release, or you can collect them by your own hands.
To avoid any issues, it is recommended to do it on register-dependent filesystems.
