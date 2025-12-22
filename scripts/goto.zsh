# Zsh wrapper for goto
# Add this to your ~/.zshrc:
# source /path/to/goto/scripts/goto.zsh

goto() {
    local output=$(~/.local/bin/goto "$@")
    if [[ $output == GOTO:* ]]; then
        cd "${output#GOTO:}"
    else
        echo "$output"
    fi
}
