#[macro_use]
extern crate mysql;
extern crate r2d2_mysql;
extern crate r2d2;

use std::env;
use std::sync::Arc;
use mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;

fn main() {
	let db_url = env::var("DATABASE_URL").unwrap();
    let db_schema = env::var("DATABASE_SCHEMA").unwrap();

    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());

    let items = query_table_outline(pool, db_schema);
    for item in items {
        println!("\
            TableOutline\n\
            table_name: {}\n\
            table_comment: {}\n\
            table_fqn: {}\n
            ", item.table_name, item.table_comment.unwrap(), item.table_fqn);
    }
}

fn query_table_outline(
    pool: Arc<r2d2::Pool<MysqlConnectionManager>>,
    param: String
) -> Vec<TableOutline> {
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();

    return conn.prep_exec(r#"
            SELECT
                tbl.table_name      AS table_name,
                tbl.table_comment   AS table_comment,
                CONCAT(tbl.table_schema, '.', tbl.table_name)
                                    AS table_fqn
            FROM
                information_schema.tables tbl
            WHERE
                tbl.table_schema = :param_schema_name
            AND tbl.table_type = 'BASE TABLE'
            ORDER BY
                tbl.table_name
            "#, params!{
                "param_schema_name" => param
            }).map::<Vec<TableOutline>, _>(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (table_name, table_comment, table_fqn) = mysql::from_row(row);
                TableOutline {
                    table_name,
                    table_comment,
                    table_fqn,
                }
            }).collect()
    }).unwrap();
}

#[derive(Debug, PartialEq, Eq)]
struct TableOutline {
    table_name: String,
    table_comment: Option<String>,
    table_fqn: String,
}