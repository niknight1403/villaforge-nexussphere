#!/bin/bash

# Check if all required environment variables are set

REQUIRED_VARS=("VILLA_HOME" "VILLA_API_KEY")

for VAR in "${REQUIRED_VARS[@]}"; do
  if [ -z "${!VAR}" ]; then
    echo "Error: $VAR is not set"
    exit 1
  fi

done

# Check if all required tools are installed

REQUIRED_TOOLS=("python3" "node" "git")

for TOOL in "${REQUIRED_TOOLS[@]}"; do
  if ! command -v $TOOL &> /dev/null; then
    echo "Error: $TOOL could not be found"
    exit 1
  fi

done

echo "All checks passed"
exit 0