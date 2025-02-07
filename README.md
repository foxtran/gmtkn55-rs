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

## Basis set dependence

DC21 dataset has significant difference for some reactions (noticible for reactions 2, 7, 9, 10). See the table below.
Other datasets may also have difference depending on chosen RI/DF approximations.

|      | PBE0 (Gaussian) | PBE0 + def2J (ORCA) | PBE0 + AutoAuxJ (ORCA) | PBE0 + def2JK (ORCA) | PBE0 + DF-JK (MRCC) | PBE0 + def2J (TM/orig.data) |
|-----:|----------------:|--------------------:|-----------------------:|---------------------:|--------------------:|----------------------------:|
|    1 |            0.98 |                0.98 |                   0.98 |                 0.98 |                0.96 |                        0.98 |
|    2 |            3.65 |                3.71 |                   3.56 |                 3.71 |                3.66 |                        9.20 |
|    3 |            8.10 |                8.09 |                   8.10 |                 8.09 |                8.11 |                        8.09 |
|    4 |           10.03 |               10.07 |                  10.02 |                10.07 |               10.08 |                       10.04 |
|    5 |           -7.68 |               -7.72 |                  -7.71 |                -7.72 |               -7.73 |                       -7.76 |
|    6 |           22.20 |               22.23 |                  22.20 |                22.23 |               22.24 |                       22.23 |
|    7 |           -1.07 |               -1.17 |                  -1.07 |                -1.17 |               -1.09 |                       -1.17 |
|    8 |          -21.09 |              -21.14 |                 -21.08 |               -21.14 |              -21.09 |                      -21.14 |
|    9 |            8.93 |                8.77 |                   8.93 |                 8.77 |                8.96 |                        8.76 |
|   10 |           -5.96 |               -5.95 |                  -5.86 |                -5.95 |               -5.93 |                       -5.97 |
|   11 |           -8.33 |               -8.37 |                  -8.33 |                -8.37 |               -8.34 |                       -8.37 |
|   12 |           -5.54 |               -5.58 |                  -5.55 |                -5.58 |               -5.56 |                       -5.58 |
|   13 |          -11.57 |              -11.53 |                 -11.55 |               -11.53 |              -11.51 |                      -11.51 |
|   MD |           -0.57 |               -0.58 |                  -0.57 |                -0.58 |               -0.56 |                             |
|  MAE |            8.86 |                8.87 |                   8.84 |                 8.87 |                8.87 |                        9.29 |
| RMSE |           10.84 |               10.85 |                  10.83 |                10.85 |               10.85 |                       11.10 |
|  MIN |          -21.09 |              -21.14 |                 -21.08 |               -21.14 |              -21.09 |                             |
|  MAX |           22.20 |               22.23 |                  22.20 |                22.23 |               22.24 |                             |
| AMAX |           22.20 |               22.23 |                  22.20 |                22.23 |               22.24 |                             |
