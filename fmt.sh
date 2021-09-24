#!/bin/bash

files=(`ls ./`)
for name in "${files[@]}"; do
	if [[ -d ${name}  ]] && [[ -f "${name}/Cargo.toml"  ]]
	then
		echo `cd ${name} && cargo fmt` 
	fi
done
