# lod

## SQLx
sqlx database create --database-url sqlite://./lodapp.db
sqlx migrate run --database-url sqlite://./lodapp.db

sqlx migrate add <migration_name> --database-url sqlite://./lodapp.db


curl -c cookies.txt -X POST http://localhost:8080/api/players/login -H "Content-Type: application/json" -d '{"username":"gian", "password":"gian"}'
curl -b cookies.txt http://localhost:8080/api/info

curl -b cookies.txt http://localhost:8080/api/players/1/sessions