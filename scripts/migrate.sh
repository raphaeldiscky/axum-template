#!/bin/bash
set -euo pipefail

# =============================================================================
# Per-service database migration script using sqlx-cli.
#
# Reads DATABASE_URL from each service's .env file.
#
# Usage:
#   bash scripts/migrate.sh up                # migrate all services
#   bash scripts/migrate.sh up app-user       # migrate one service
#   bash scripts/migrate.sh down              # rollback all services
#   bash scripts/migrate.sh down app-user     # rollback one service
# =============================================================================

# ---------------------------------------------------------------------------
# Service registry — add new services here
# ---------------------------------------------------------------------------

get_service_env_file() {
    case "$1" in
        app-user) echo "services/user-service/.env" ;;
        *) echo ""; return 1 ;;
    esac
}

get_all_services() {
    echo "app-user"
}

# ---------------------------------------------------------------------------
# Migration logic
# ---------------------------------------------------------------------------

run_migration() {
    local direction="$1"
    local service="$2"

    local env_file
    env_file=$(get_service_env_file "$service")
    if [ -z "$env_file" ]; then
        echo "ERROR: Unknown service '$service'"
        exit 1
    fi

    if [ ! -f "$env_file" ]; then
        echo "ERROR: Env file not found at $env_file"
        exit 1
    fi

    local migration_dir="crates/${service}/migrations"
    if [ ! -d "$migration_dir" ]; then
        echo "SKIP: No migration directory found at $migration_dir"
        return 0
    fi

    # Read DATABASE_URL from the service's .env file
    local database_url
    database_url=$(grep -E "^DATABASE_URL=" "$env_file" | cut -d '=' -f2-)
    if [ -z "$database_url" ]; then
        echo "ERROR: DATABASE_URL not found in $env_file"
        exit 1
    fi

    echo "==> [$service] Running migrate $direction"

    if [ "$direction" = "up" ]; then
        DATABASE_URL="$database_url" sqlx migrate run --source "$migration_dir"
    elif [ "$direction" = "down" ]; then
        DATABASE_URL="$database_url" sqlx migrate revert --source "$migration_dir"
    else
        echo "ERROR: Unknown direction '$direction'. Use 'up' or 'down'."
        exit 1
    fi

    echo "==> [$service] Done"
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if [ $# -lt 1 ]; then
    echo "Usage: bash scripts/migrate.sh <up|down> [service]"
    exit 1
fi

DIRECTION="$1"
SERVICE="${2:-}"

if [ -n "$SERVICE" ]; then
    run_migration "$DIRECTION" "$SERVICE"
else
    for svc in $(get_all_services); do
        run_migration "$DIRECTION" "$svc"
    done
fi
