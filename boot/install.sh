#!/bin/sh

verify () {
	if [ ! -f "$1" ]; then
		echo ""                                                   1>&2
		echo " *** Missing file: $1"                              1>&2
		echo ' *** You need to run "make" before "make install".' 1>&2
		echo ""                                                   1>&2
		exit 1
	fi
}

# Make sure the files actually exist
verify "$2"
verify "$3"

# User may have a custom install script
if [ -x ~/bin/${INSTALLKERNEL} ]; then exec ~/bin/${INSTALLKERNEL} "$@"; fi
if [ -x /sbin/${INSTALLKERNEL} ]; then exec /sbin/${INSTALLKERNEL} "$@"; fi

if [ "$(basename $2)" = "zImage" ]; then
# Compressed install
  echo "Installing compressed kernel"
  base=vmlinuz
else
# Normal install
  echo "Installing normal kernel"
  base=vmlinux
fi

if [ -f $4/$base-$1 ]; then
  mv $4/$base-$1 $4/$base-$1.old
fi
cat $2 > $4/$base-$1

# Install system map file
if [ -f $4/System.map-$1 ]; then
  mv $4/System.map-$1 $4/System.map-$1.old
fi
cp $3 $4/System.map-$1

if [ -x /sbin/loadmap ]; then
  /sbin/loadmap
else
  echo "You have to install it yourself"
fi

