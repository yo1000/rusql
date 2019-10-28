# rusql
Rust SQL exec demo

```bash
docker run -d \
  --name mysql-sakila \
  -p 3306:3306 \
  thebinarypenguin/mysql-sakila:latest

export DATABASE_URL=mysql://root:sakila@127.0.0.1:3306/sakila
export DATABASE_SCHEMA=sakila

cargo run --release
```