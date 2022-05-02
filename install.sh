#!/usr/bin/env bash

PROFILE=${1:-debug}

echo "install ${PROFILE}"

[[ -d lua ]] || mkdir lua
cp target/${PROFILE}/lib*.so lua/


for f in $(ls lua/lib*.so); do
    t=${f/lib/};
    [[ "${f}" != "" ]] \
        && echo "mv ${f} ${t}" \
        && mv "${f}" "${t}";
done 
