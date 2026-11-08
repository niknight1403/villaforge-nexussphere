#!/bin/bash
# NexusSphere Build Wrapper with Exponential Backoff
MAX_RETRIES=3
RETRY_DELAY=2

attempt=1
until cargo "$@"; do
  exit_code=$?
  if [ $attempt -le $MAX_RETRIES ]; then
    echo "Build failed with code $exit_code. Retrying in $RETRY_DELAY seconds... (Attempt $attempt/$MAX_RETRIES)"
    sleep $RETRY_DELAY
    attempt=$((attempt + 1))
    RETRY_DELAY=$((RETRY_DELAY * 2))
  else
    echo "Build failed after $MAX_RETRIES attempts."
    exit $exit_code
  fi
done
echo "Build successful."