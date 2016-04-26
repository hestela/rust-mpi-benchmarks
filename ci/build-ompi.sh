#!/bin/bash

set -e

if [ -f "$HOME/ompi/bin/mpicc" ]; then
  export PATH=$HOME/ompi/bin:$PATH
  export LD_LIBRARY_PATH=$HOME/ompi/lib:$LD_LIBRARY_PATH
  echo "Openmpi already installed."
  exit 0
fi

wget http://www.open-mpi.org/software/ompi/v1.10/downloads/openmpi-1.10.0.tar.gz --no-check-certificate
tar xf openmpi-1.10.0.tar.gz

cd openmpi-1.10.0

echo "Building OMPI..."
mkdir -p $HOME/ompi
./configure --enable-shared --prefix=$HOME/ompi > /dev/null
make -j 2 > /dev/null
make -j 2 install > /dev/null

