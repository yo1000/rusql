# rusql
Rust SQL exec demo

```bash
docker run -d \
  --name mysql-sakila \
  -p 3306:3306 \
  thebinarypenguin/mysql-sakila:latest

# Requirements
export DATABASE_USER=root
export DATABASE_PASS=sakila
export DATABASE_NAME=sakila

# Options
export DATABASE_HOST=127.0.0.1
export DATABASE_PORT=3306

cargo run --release
```