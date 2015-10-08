#!/bin/sh

set -e

BINARIES_DIR="target/release"

binaries=$(find $BINARIES_DIR -maxdepth 1 -type f -executable \
  -exec file -i '{}' \; | cut -d":" -f1)
num_binaries=$(printf "%d" "$(echo "${binaries}" | wc -w)")

echo "Running with $(which mpiexec)"

num_ok=0
num_failed=0
result="ok"

for binary in ${binaries}
do
  echo "Starting benchmark: ${binary}."
  output_file=${binary}_output
  if (mpiexec -np 2 ./${binary} > "${output_file}")
  then
    echo "ok"
    num_ok=$((${num_ok} + 1))
  else
    echo "output:"
    cat "${output_file}"
    num_failed=$((${num_failed} + 1))
    result="failed"
  fi
  rm -f "${output_file}"
done

echo "${num_ok}/${num_binaries} worked; ${num_failed} failed."
if [ "$num_failed" -ne "0" ]; then
  exit 1
fi
