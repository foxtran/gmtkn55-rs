#!/usr/bin/env bash

FUNC=$1
REALNAME=$2

if [ $# -lt 2 ]; then
  echo "REALNAME is not defined!"
  exit 1
fi

mkdir ${REALNAME}

j=0
echo "" > ${REALNAME}/ID.list
for i in ../GMTKN55/GEOM_ALL/*.xyz
do
    j=$((j+1))
    strname=`echo ${i} | cut -d"/" -f4 | sed "s/.xyz//"`
    cmb=`head -n 2 ${i} | tail -n 1`
    charge=`echo ${cmb} | cut -d" " -f1`
    mult=`echo ${cmb} | cut -d" " -f2`
    basis=`echo ${cmb} | cut -d" " -f3`
    struct=`tail -n +3 ${i} | sed "s/^ *//; /^$/d" | tr "\n" "|"`
    echo ${strname} ${charge} ${mult} ${basis}
    if [ "${basis}" = "def2QZVP" ]; then
      cat g09.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/def2QZVP/; s/{BASISHELP}//" | tr "|" "\n" > ${REALNAME}/${strname}.gjf
    elif [ "${basis}" = "def2QZVP+def2ECP" ]; then
      cat g09.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/def2QZVP/; s/{BASISHELP}//" | tr "|" "\n" > ${REALNAME}/${strname}.gjf
    elif [ "${basis}" = "def2QZVP-WATER27" ]; then
      cat g09.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/GEN/; s|{BASISHELP}|@../BASIS/def2QZVP-WATER27.gbs|" | tr "|" "\n" > ${REALNAME}/${strname}.gjf
    elif [ "${basis}" = "aug'-def2QZVP" ]; then
      cat g09.inp | sed "s/{FUNC}/${FUNC}/; s/{chrg}/${charge}/; s/{mult}/${mult}/; s/{coord}/${struct}/; s/{BASIS}/GEN/; s|{BASISHELP}|@../BASIS/maug-def2QZVP.gbs|" | tr "|" "\n" > ${REALNAME}/${strname}.gjf
    else
      echo "Unknown basis"
    fi
    echo "${j} ${strname}.gjf" >> ${REALNAME}/ID.list
done
