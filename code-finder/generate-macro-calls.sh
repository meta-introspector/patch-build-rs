#!/bin/bash

# This script reads ripgrep --json output from stdin,
# and transforms each match into a Rust macro call.

# Example usage:
# ./code-finder/find-code.sh | ./code-finder/generate-macro-calls.sh > output_macro_calls.rs

echo "// Generated Rust macro calls from ripgrep results"
echo ""

# Read JSON input line by line and process with jq
# We assume each line from stdin is a complete JSON object as output by `rg --json`
while IFS= read -r line; do
  if [[ -n "$line" ]]; then
    # Extract relevant fields using jq
    FILE_PATH=$(echo "$line" | jq -r 'select(.type == "match") | .data.path.text')
    LINE_NUMBER=$(echo "$line" | jq -r 'select(.type == "match") | .data.line_number')
    MATCH_TEXT=$(echo "$line" | jq -r 'select(.type == "match") | .data.lines.text')
    COLUMN_NUMBER=$(echo "$line" | jq -r 'select(.type == "match") | .data.submatches[0].start')

    # Only output if all fields are successfully extracted
    if [[ -n "$FILE_PATH" && -n "$LINE_NUMBER" && -n "$MATCH_TEXT" && -n "$COLUMN_NUMBER" ]]; then
      # Escape double quotes and backslashes in MATCH_TEXT for Rust string literal
      ESCAPED_MATCH_TEXT=$(echo "$MATCH_TEXT" | sed 's/\\/\\\\/g; s/"/\"/g')

      echo "process_match! {"
      echo "    file: \"$FILE_PATH\","
      echo "    line: $LINE_NUMBER,"
      echo "    column: $COLUMN_NUMBER,"
      echo "    text: \"$ESCAPED_MATCH_TEXT\""
      echo "};
"
    fi
  fi
done < /dev/stdin
