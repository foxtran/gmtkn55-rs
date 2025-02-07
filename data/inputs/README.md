# Input and Input generators

There are input and input generators used by Igor S. Gerasimov to generate input files.
While input are consistent for later users of this repo, note, that input generator does **NOT** work for them and should be used only for demonstration purposes.

Also, note that number threads/memory resources can be hard-coded in input templates.

Input templates has the following parameters:
- `{FUNC}` - used method for computing
- `{BASIS}` - used basis set for computing (should be taken from description in input structures)
- `{BASISHELP}` - helper for basis sets, may be needed sometimes
- `{chrg}` - charge of system, can be taken from input structures
- `{mult}` - multiplicity of system, can be taken from input structures
- `{coord}` - XYZ encoded structure

## Gaussian

Has `g09.inp` input template and `compute_g09.sh` to fill this template.

## ORCA

Has three sets of inputs for ORCA 6.0.1:
- `orca6-TM-def2J` used for reproducing Turbomole results since definition of basis sets and how `RI`/`DF` works may vary between different programs/implementations;
- `orca6-AutoAux` used for avoiding issues with `def2J` auxiliary basis set;
- `orca6-def2JK` used for proper computing of hybrid functionals with `def2JK` auxiliary basis set.

As well as they have three possible syffixes:
- `.inp` for standard calculations with `def2-QZVP` (with or without ECP) basis set;
- `-WATER27.inp` for computing `WATER27` dataset;
- `-BASIS.inp` for computing with significantly modified basis sets.

Corresponding input generators are called `compute_orca6-TM-def2J.sh`, `compute_orca6-AutoAux.sh`, and `compute_orca6-def2JK.sh`.

## MRCC

Has single input template `MINP` and corresponding `compute_MRCC.sh` for generating inputs. `SLURM` script is not provided.
Note, MRCC 31 Dec, 2024 was used. Previous releases may not work properly.
