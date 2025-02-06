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
    natoms=`head -n 1 ${i}`
    echo ${strname} ${charge} ${mult} ${basis}
    mkdir -p ${REALNAME}/${strname}
    cp submit-mrcc.sh ${REALNAME}/${strname}
    if [ "${basis}" = "def2QZVP" ]; then
      cat MINP | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{natoms}/${natoms}/; s/{struct}/${struct}/; s/{BASIS}/def2-QZVP/; s/{ECP_BASIS}/def2-ECP-60/; s/{DF_BASIS}/def2-QZVPP-RI-JK/" | tr "|" "\n" > ${REALNAME}/${strname}/MINP
    elif [ "${basis}" = "def2QZVP+def2ECP" ]; then
      cat MINP | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{natoms}/${natoms}/; s/{struct}/${struct}/; s/{BASIS}/def2-QZVP/; s/{ECP_BASIS}/def2-ECP-60/; s/{DF_BASIS}/def2-QZVPP-RI-JK/" | tr "|" "\n" > ${REALNAME}/${strname}/MINP
    elif [ "${basis}" = "def2QZVP-WATER27" ]; then
      cat MINP | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{natoms}/${natoms}/; s/{struct}/${struct}/; s/{BASIS}/def2-QZVP-WATER27/; s/{ECP_BASIS}/def2-ECP-60/; s/{DF_BASIS}/def2-QZVPP-RI-JK/" | tr "|" "\n" > ${REALNAME}/${strname}/MINP
      cp BASIS/def2QZVP-WATER27.MRCCbs ${REALNAME}/${strname}/GENBAS
    elif [ "${basis}" = "aug'-def2QZVP" ]; then
      cat MINP | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{natoms}/${natoms}/; s/{struct}/${struct}/; s/{BASIS}/maug-def2-QZVP/; s/{ECP_BASIS}/def2-ECP-60/; s/{DF_BASIS}/def2-QZVPP-RI-JK/" | tr "|" "\n" > ${REALNAME}/${strname}/MINP
      cp BASIS/maug-def2QZVP.MRCCbs ${REALNAME}/${strname}/GENBAS
    else
      echo "Unknown basis"
    fi
done
