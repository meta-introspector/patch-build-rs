#!/bin/bash
# This is a dummy shell script for shebling! macro testing

echo "Hello from dummy script!"

VAR="test value"
echo "$VAR"

if [ -z "$1" ]; then
  echo "No argument provided."
else
  echo "Argument: $1"
fi
