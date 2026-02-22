#!/bin/sh

usage() {
    echo "Usage: ypick [OPTIONS]"
    echo "Options:"
    echo "  -o, --outfile [path]   Output to file instead of stdout"
    echo "  -j, --json [path]      Path to json file with options"
    echo "  -m, --mode [Mode]      Set picker mode (default: 1 - Open file)"
    echo "Modes:"
    echo "  0, x: Special"
    echo "  1, f: Open file"
    echo "  2, s: Save file (New or Overwrite)"
    echo "  3, d: Single directory"
    echo "  4, ff: Multiple files"
    echo "  5, dd: Multiple directories"
    exit 1
}

function get_mode() {
	case "$1" in
		0|x)  MODE=0 ;;
		1|f)  MODE=1 ;;
		2|s)  MODE=2 ;;
		3|d)  MODE=3 ;;
		4|ff) MODE=4 ;;
		5|dd) MODE=5 ;;
	esac
}

MODE=1
while [ "$#" -gt 0 ]; do
    case "$1" in
        -o|--outfile) OUT_FILE="$2"; shift 2 ;;
        -j|--json) JSON_FILE="$2"; shift 2 ;;
        -m|--mode) get_mode "$2"; shift 2 ;;
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
export YAZI_PICKER_JSON="$JSON_FILE"

# Launch Yazi
yazi

# Read result, cleanup, and output to stdout
if [ -s "$OUT_FILE" ]; then
	[ "$DO_CAT" -eq 1 ] && cat "$OUT_FILE"
    exit 0
else
    exit 1
fi
