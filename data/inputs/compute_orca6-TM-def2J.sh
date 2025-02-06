#!/usr/bin/env bash

FUNC=$1
REALNAME=$2

if [ $# -lt 2 ]; then
  echo "REALNAME is not defined!"
  exit 1
fi

mkdir ${REALNAME}

for i in ../GMTKN55/GEOM_ALL/*.xyz
do
    strname=`echo ${i} | cut -d"/" -f4 | sed "s/.xyz//"`
    cmb=`head -n 2 ${i} | tail -n 1`
    charge=`echo ${cmb} | cut -d" " -f1`
    mult=`echo ${cmb} | cut -d" " -f2`
    basis=`echo ${cmb} | cut -d" " -f3`
    struct=`tail -n +3 ${i} | sed "s/^ *//; /^$/d" | tr "\n" "|"`
    echo ${strname} ${charge} ${mult} ${basis}
    if [ "${basis}" = "def2QZVP" ]; then
      cat orca6-TM-def2J.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/def2-QZVP/; s/{BASISHELP}//" | tr "|" "\n" > ${REALNAME}/${strname}.inp
    elif [ "${basis}" = "def2QZVP+def2ECP" ]; then
      cat orca6-TM-def2J.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/def2-QZVP/; s/{BASISHELP}//" | tr "|" "\n" > ${REALNAME}/${strname}.inp
    elif [ "${basis}" = "def2QZVP-WATER27" ]; then
      cat orca6-TM-def2J-WATER27.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/def2-QZVP/;" | tr "|" "\n" > ${REALNAME}/${strname}.inp
      echo "Skip"
    elif [ "${basis}" = "aug'-def2QZVP" ]; then
      cat orca6-TM-def2J-BASIS.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s|{BASISHELP}|maug-def2QZVP.obs|" | tr "|" "\n" > ${REALNAME}/${strname}.inp
    else
      echo "Unknown basis"
    fi
done
