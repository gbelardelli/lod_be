# lod

## SQLx
sqlx database create --database-url sqlite://./lodapp.db
sqlx migrate run --database-url sqlite://./lodapp.db

sqlx migrate add <migration_name> --database-url sqlite://./lodapp.db
