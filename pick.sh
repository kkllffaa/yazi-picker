#!/bin/sh

usage() {
    echo "Usage: ypick [OPTIONS]"
    echo "Options:"
    echo "  -o, --outfile [path]   Output to file instead of stdout"
    echo "  -m, --mode [1-6]       Set picker mode (default: 1)"
    echo "Modes:"
    echo "  1: Single existing file"
    echo "  2: Single file (New or Overwrite)"
    echo "  3: Single directory"
    echo "  4: Multiple files"
    echo "  5: Multiple directories"
    echo "  6: Files and directories"
    exit 1
}

MODE="1"
while [ "$#" -gt 0 ]; do
    case "$1" in
        -o|--outfile) OUT_FILE="$2"; shift 2 ;;
        -m|--mode) MODE="$2"; shift 2 ;;
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
export YAZI_PICKER_MODE="$MODE"
export YAZI_PICKER_OUT="$OUT_FILE"

# Launch Yazi
yazi

# Read result, cleanup, and output to stdout
if [ -s "$OUT_FILE" ]; then
	[ "$DO_CAT" -eq 1 ] && cat "$OUT_FILE"
    exit 0
else
    exit 1
fi
