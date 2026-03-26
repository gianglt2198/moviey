#!/bin/bash

# Database Backup Script
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_DIR="./backups"
DB_NAME="moviey_db"
DB_USER="moviey_prod"
DB_HOST="localhost"
DB_PORT="5432"

mkdir -p "$BACKUP_DIR"

# Create backup
BACKUP_FILE="$BACKUP_DIR/moviey_backup_$TIMESTAMP.sql"

pg_dump \
    --host=$DB_HOST \
    --port=$DB_PORT \
    --username=$DB_USER \
    --format=custom \
    --file=$BACKUP_FILE \
    $DB_NAME

if [ $? -eq 0 ]; then
    echo "✅ Backup successful: $BACKUP_FILE"
    
    # Keep only last 7 days of backups
    find "$BACKUP_DIR" -name "moviey_backup_*.sql" -mtime +7 -delete
    echo "🧹 Old backups cleaned"
else
    echo "❌ Backup failed"
    exit 1
fi
