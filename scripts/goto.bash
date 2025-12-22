# Bash wrapper for goto
# Add this to your ~/.bashrc:
# source /path/to/goto/scripts/goto.bash

goto() {
    local output=$(~/.local/bin/goto "$@")
    if [[ $output == GOTO:* ]]; then
        cd "${output#GOTO:}"
    else
        echo "$output"
    fi
}
