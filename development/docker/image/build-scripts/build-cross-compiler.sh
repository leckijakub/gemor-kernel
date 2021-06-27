#!/bin/bash

GCC_VERSION="10.2.0"
BINUTILS_VERSION="2.36"

GCC_DIR="${HOME}/gcc-${GCC_VERSION}"
BINUTILS_DIR="${HOME}/binutils-${BINUTILS_VERSION}"

install -d ${GCC_DIR} ${BINUTILS_DIR}
wget https://ftp.gnu.org/gnu/gcc/gcc-${GCC_VERSION}/gcc-${GCC_VERSION}.tar.gz -O - | tar -xz --strip-components=1 -C ${GCC_DIR}
wget https://ftp.gnu.org/gnu/binutils/binutils-${BINUTILS_VERSION}.tar.gz -O - | tar -xz --strip-components=1 -C ${BINUTILS_DIR}

PREFIX="$HOME/opt/cross"
TARGET=i686-elf
export PATH="$PREFIX/bin:$PATH"

cd ${BINUTILS_DIR}/..
 
install -d build-binutils
cd build-binutils
../binutils-${BINUTILS_VERSION}/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make
make install

cd ${GCC_DIR}/..
 
# The $PREFIX/bin dir _must_ be in the PATH. We did that above.
which -- $TARGET-as || echo $TARGET-as is not in the PATH
 
install -d build-gcc
cd build-gcc
../gcc-${GCC_VERSION}/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make all-gcc
make all-target-libgcc
make install-gcc
make install-target-libgcc
