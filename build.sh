#!/usr/bin/env bash
function get_os() {
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "linux"
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macos"
  elif [[ "$OSTYPE" == "cygwin" ]]; then
    echo "cygwin"
  elif [[ "$OSTYPE" == "msys" ]]; then
    echo "msys"
  elif [[ "$OSTYPE" == "win32" ]]; then
    echo "win32"
  elif [[ "$OSTYPE" == "freebsd"* ]]; then
    echo "freebsd"
  else
    echo "unknown"
  fi
}

function build_linux() {
  PROFILE=${1:-debug}
  case $PROFILE in
  "debug")
    cargo build --debug
    ;;
  "release")
    cargo build --debug
    ;;
  esac

  [[ -d lua ]] || mkdir lua
  cp target/"${PROFILE}"/lib*.so lua/
  for f in lua/lib*.so; do
    t=${f/#lib/}
    [[ "${f}" != "" ]] &&
      echo "mv ${f} ${t}" &&
      mv "${f}" "${t}"
  done
}

function build_macos() {
  PROFILE=${1:-debug}
  case $PROFILE in
  "debug")
    cargo rustc -- \
      -C link-arg=-undefined \
      -C link-arg=dynamic_lookup
    ;;
  "release")
    cargo rustc --release -- \
      -C link-arg=-undefined \
      -C link-arg=dynamic_lookup
    ;;
  esac

  [[ -d lua ]] || mkdir lua
  cp target/"${PROFILE}"/lib*.dylib lua/
  for f in lua/lib*.dylib; do
    t=${f/#lib/}
    t=${t/%dylib/so}
    [[ "${f}" != "" ]] &&
      echo "mv ${f} ${t}" &&
      mv "${f}" "${t}"
  done
}

PROFILE=${1:-debug}

case $(get_os) in
"linux")
  build_linux "$PROFILE"
  ;;
"macos")
  build_macos "$PROFILE"
  ;;
esac