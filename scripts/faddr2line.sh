#!/bin/bash

# This file was copied from linux
# It probably makes startup work

set -o errexit
set -o nounset

READELF="${CROSS_COMPILE:-}readelf"
ADDR2LINE="${CROSS_COMPILE:-}addr2line"
SIZE="${CROSS_COMPILE:-}size"
NM="${CROSS_COMPILE:-}nm"

command -v awk >/dev/null 2>&1 || die "awk isn't installed"
command -v ${READELF} >/dev/null 2>&1 || die "readelf isn't installed"
command -v ${ADDR2LINE} >/dev/null 2>&1 || die "addr2line isn't installed"
command -v ${SIZE} >/dev/null 2>&1 || die "size isn't installed"
command -v ${NM} >/dev/null 2>&1 || die "nm isn't installed"

usage() {
	echo "usage: faddr2line [--list] <object file> <func+offset> <func+offset>..." >&2
	exit 1
}

warn() {
	echo "$1" >&2
}

die() {
	echo "ERROR: $1" >&2
	exit 1
}

# Try to figure out the source directory prefix so we can remove it from the
# addr2line output.  HACK ALERT: This assumes that start_kernel() is in
# init/main.c!  This only works for vmlinux.  Otherwise it falls back to
# printing the absolute path.
find_dir_prefix() {
	local objfile=$1

	local start_kernel_addr=$(${READELF} -sW $objfile | awk '$8 == "start_kernel" {printf "0x%s", $2}')
	[[ -z $start_kernel_addr ]] && return

	local file_line=$(${ADDR2LINE} -e $objfile $start_kernel_addr)
	[[ -z $file_line ]] && return

	local prefix=${file_line%init/main.c:*}
	if [[ -z $prefix ]] || [[ $prefix = $file_line ]]; then
		return
	fi

	DIR_PREFIX=$prefix
	return 0
}

__faddr2line() {
	local objfile=$1
	local func_addr=$2
	local dir_prefix=$3
	local print_warnings=$4

	local func=${func_addr%+*}
	local offset=${func_addr#*+}
	offset=${offset%/*}
	local size=
	[[ $func_addr =~ "/" ]] && size=${func_addr#*/}

	if [[ -z $func ]] || [[ -z $offset ]] || [[ $func = $func_addr ]]; then
		warn "bad func+offset $func_addr"
		DONE=1
		return
	fi

	# Go through each of the object's symbols which match the func name.
	# In rare cases there might be duplicates.
	file_end=$(${SIZE} -Ax $objfile | awk '$1 == ".text" {print $2}')
	while read symbol; do
		local fields=($symbol)
		local sym_base=0x${fields[0]}
		local sym_type=${fields[1]}
		local sym_end=${fields[3]}

		# calculate the size
		local sym_size=$(($sym_end - $sym_base))
		if [[ -z $sym_size ]] || [[ $sym_size -le 0 ]]; then
			warn "bad symbol size: base: $sym_base end: $sym_end"
			DONE=1
			return
		fi
		sym_size=0x$(printf %x $sym_size)

		# calculate the address
		local addr=$(($sym_base + $offset))
		if [[ -z $addr ]] || [[ $addr = 0 ]]; then
			warn "bad address: $sym_base + $offset"
			DONE=1
			return
		fi
		addr=0x$(printf %x $addr)

		# weed out non-function symbols
		if [[ $sym_type != t ]] && [[ $sym_type != T ]]; then
			[[ $print_warnings = 1 ]] &&
				echo "skipping $func address at $addr due to non-function symbol of type '$sym_type'"
			continue
		fi

		# if the user provided a size, make sure it matches the symbol's size
		if [[ -n $size ]] && [[ $size -ne $sym_size ]]; then
			[[ $print_warnings = 1 ]] &&
				echo "skipping $func address at $addr due to size mismatch ($size != $sym_size)"
			continue;
		fi

		# make sure the provided offset is within the symbol's range
		if [[ $offset -gt $sym_size ]]; then
			[[ $print_warnings = 1 ]] &&
				echo "skipping $func address at $addr due to size mismatch ($offset > $sym_size)"
			continue
		fi

		# separate multiple entries with a blank line
		[[ $FIRST = 0 ]] && echo
		FIRST=0

		# pass real address to addr2line
		echo "$func+$offset/$sym_size:"
		local file_lines=$(${ADDR2LINE} -fpie $objfile $addr | sed "s; $dir_prefix\(\./\)*; ;")
		[[ -z $file_lines ]] && return

		if [[ $LIST = 0 ]]; then
			echo "$file_lines" | while read -r line
			do
				echo $line
			done
			DONE=1;
			return
		fi

		# show each line with context
		echo "$file_lines" | while read -r line
		do
			echo
			echo $line
			n=$(echo $line | sed 's/.*:\([0-9]\+\).*/\1/g')
			n1=$[$n-5]
			n2=$[$n+5]
			f=$(echo $line | sed 's/.*at \(.\+\):.*/\1/g')
			awk 'NR>=strtonum("'$n1'") && NR<=strtonum("'$n2'") { if (NR=='$n') printf(">%d<", NR); else printf(" %d ", NR); printf("\t%s\n", $0)}' $f
		done

		DONE=1

	done < <(${NM} -n $objfile | awk -v fn=$func -v end=$file_end '$3 == fn { found=1; line=$0; start=$1; next } found == 1 { found=0; print line, "0x"$1 } END {if (found == 1) print line, end; }')
}

[[ $# -lt 2 ]] && usage

objfile=$1

LIST=0
[[ "$objfile" == "--list" ]] && LIST=1 && shift && objfile=$1

[[ ! -f $objfile ]] && die "can't find objfile $objfile"
shift

DIR_PREFIX=supercalifragilisticexpialidocious
find_dir_prefix $objfile

FIRST=1
while [[ $# -gt 0 ]]; do
	func_addr=$1
	shift

	# print any matches found
	DONE=0
	__faddr2line $objfile $func_addr $DIR_PREFIX 0

	# if no match was found, print warnings
	if [[ $DONE = 0 ]]; then
		__faddr2line $objfile $func_addr $DIR_PREFIX 1
		warn "no match for $func_addr"
	fi
done
