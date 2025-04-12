#!/bin/bash


docker exec -it projectmanager-db-1 psql -U pm -d pm -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"

