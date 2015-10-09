#!/bin/bash

set -e

if [ -d "$HOME/ompi" ]; then
  echo "Openmpi already installed."
  exit 0
fi

curl http://www.open-mpi.org/software/ompi/v1.10/downloads/openmpi-1.10.0.tar.gz | tar zx

cd openmpi-1.10.0

mkdir -p $HOME/ompi
./configure --enable-shared --prefix=$HOME/ompi --disable-fortran
make -j 2
make -j 2 install

