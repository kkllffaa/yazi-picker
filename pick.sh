#!/bin/sh

usage() {
	echo "Usage: ypick [MODE] [OPTIONS]"
	echo "Options:"
	echo "  -o, --outfile [path]   Output to file instead of stdout"
	echo "  -i, --infile [path]    Override mode with json options file"
	echo "  -j, --json             Output in json format instead of literal paths"
	echo "Modes:"
	echo "  0, x: Special (Default)"
	echo "  1, f: Open file"
	echo "  2, s: Save file (New or Overwrite)"
	echo "  3, d: Single directory"
	echo "  4, ff: Multiple files"
	echo "  5, dd: Multiple directories"
	exit 1
}

MODE=0
while [ "$#" -gt 0 ]; do
	case "$1" in
		-o|--outfile) OUT_FILE="$2"; shift 2 ;;
		-i|--infile) IN_FILE="$2"; shift 2 ;;
		-j|--json) JSON=1; shift 1 ;;
		0|x)  MODE=0; shift 1 ;;
		1|f)  MODE=1; shift 1 ;;
		2|s)  MODE=2; shift 1 ;;
		3|d)  MODE=3; shift 1 ;;
		4|ff) MODE=4; shift 1 ;;
		5|dd) MODE=5; shift 1 ;;
		-h|--help) usage ;;
		*) usage ;;
	esac
done

if [ -n "$OUT_FILE" ]; then
	DO_CAT=0
else
	OUT_FILE=$(mktemp /tmp/yazi-picker.XXXXXX)
	trap 'rm -f "$OUT_FILE"' EXIT
	DO_CAT=1
fi

# Export variables for the Lua plugin to read
if [ -n "$IN_FILE" ]; then
	export YAZI_PICKER_IN="$IN_FILE"
else
	export YAZI_PICKER_MODE="$MODE"
fi
export YAZI_PICKER_OUT="$OUT_FILE"
export YAZI_PICKER_JSON="$JSON"

# Launch Yazi
yazi

# Read result, cleanup, and output to stdout
if [ -s "$OUT_FILE" ]; then
	[ "$DO_CAT" -eq 1 ] && cat "$OUT_FILE"
	exit 0
else
	exit 1
fi
