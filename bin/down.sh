#!/usr/bin/env bash
#
# down.sh - Bring down docker compose environment with the given profiles
#
# Usage: ./bin/down.sh [profiles...]
#
# Examples:
#   ./bin/down.sh              # Stop all services
#   ./bin/down.sh app          # Stop with app profile
#   ./bin/down.sh app integration-tests  # Stop with multiple profiles

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_DIR"

# Build profile arguments
PROFILE_ARGS=""
for profile in "$@"; do
  PROFILE_ARGS="$PROFILE_ARGS --profile $profile"
done

echo "Stopping docker compose..."

if [ -z "$PROFILE_ARGS" ]; then
  echo "No profiles specified - stopping all services"
  docker compose down
else
  echo "Stopping with profiles: $*"
  # shellcheck disable=SC2086
  docker compose $PROFILE_ARGS down
fi

echo ""
echo "Services stopped successfully!"
echo ""
echo "To remove volumes as well, run:"
echo "  docker compose down -v"
