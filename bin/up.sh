#!/usr/bin/env bash
#
# up.sh - Start docker compose with the given profiles
#
# Usage: ./bin/up.sh [profiles...]
#
# Examples:
#   ./bin/up.sh              # Start without any profiles (just dependencies)
#   ./bin/up.sh app          # Start with app profile
#   ./bin/up.sh app integration-tests  # Start with multiple profiles

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_DIR"

# Build profile arguments
PROFILE_ARGS=""
for profile in "$@"; do
  PROFILE_ARGS="$PROFILE_ARGS --profile $profile"
done

echo "Starting docker compose..."

if [ -z "$PROFILE_ARGS" ]; then
  echo "No profiles specified - starting base services only (postgres, redis, httpbin)"
  docker compose up -d
else
  echo "Starting with profiles: $*"
  # shellcheck disable=SC2086
  docker compose $PROFILE_ARGS up -d
fi

echo ""
echo "Services started successfully!"
echo ""
echo "To view logs, run:"
echo "  docker compose logs -f"
echo ""
echo "To stop services, run:"
echo "  ./bin/down.sh $*"
