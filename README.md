# Reproduce GMTKN55 dataset

This repo collects data and scripts for reproducing GMTKN55 and related datasets (mostly from Tim Gould)

## Dataset notices

- WATER27 dataset has improper reference data in `._res` script. Use reference data from GMTKN55 website.
- Reaction #2 from DC13 uses old value (from DC9) during computing error for, at least, PBE0 and PBEh1PBE functionals, therefore the data is not consistent between officially published and presented here.
- For BH76 and BH76RC datasets, `H2` and `h2` geometries were renamed to `H2_v1` and `h2_v2`, respectively, for compability with register-independent filesystems.
- For NBPRC dataset, `H2` and `h2`, `BH3` and `bh3` geometries were renamed to `H2_v1` and `h2_v2`, `BH3_v1` and `bh3_v2`, respectively, for compability with register-independent filesystems.

## Computation notices

### Gaussian

All calculation were computed with Gaussian 09 rev. B. Therefore, data is presented ONLY for dispersionless functionals.

`RC21_2p2` was computed with non-standard guess. Used guess is `guess(Huckel)`.
