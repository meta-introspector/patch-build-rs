#!/bin/bash

# This script searches for Rust code related to directory scanning, file reading,
# and code quoting using ripgrep.

# Define the keywords in a JSON structure
KEYWORDS_JSON='{
  "directory_scanning": [
    "std::fs::read_dir",
    "walkdir",
    "Path::new",
    "PathBuf",
    "fs::metadata",
    "is_dir",
    "is_file"
  ],
  "file_reading": [
    "std::fs::read_to_string",
    "File::open",
    "BufReader",
    "lines()"
  ],
  "code_quoting_rust": [
    "proc_macro2::TokenStream",
    "quote!",
    "syn::",
    "parse_macro_input!",
    "to_tokens"
  ]
}'

# Extract all keywords and join them with '|' for ripgrep's OR regex
SEARCH_PATTERN=$(echo "$KEYWORDS_JSON" | jq -r 'map(.[] | select(type == "string")) | join("|")')

# Run ripgrep and output null-delimited list of files that contain matches
rg --files-with-matches -0 -t rust "$SEARCH_PATTERN"
