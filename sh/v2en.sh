#!/usr/bin/env bash

v2() {
  declare q="$*"
  curl https://d.supjohn.com/"${q// /%20}"
}

v2-sh() {
  while
    echo -n "v2en> "
    read -r input
    [[ -n $input ]]
  do
    v2 "$input"
  done
}

v2-sh
