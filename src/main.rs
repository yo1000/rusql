#[macro_use]
extern crate mysql;
use mysql::{
    Opts,
    OptsBuilder,
};

extern crate r2d2;
extern crate r2d2_mysql;
use r2d2_mysql::{
    MysqlConnectionManager,
};

use std::{
    env,
    str::FromStr,
    sync::Arc,
};


const DATABASE_HOST: &str = "DATABASE_HOST";
const DATABASE_PORT: &str = "DATABASE_PORT";
const DATABASE_USER: &str = "DATABASE_USER";
const DATABASE_PASS: &str = "DATABASE_PASS";
const DATABASE_NAME: &str = "DATABASE_NAME";

const DATABASE_POOL_SIZE: u32 = 4;

fn main() {
    let db_host = env_var::<String>(DATABASE_HOST, Some("127.0.0.1".to_string()));
    let db_port = env_var::<String>(DATABASE_PORT, Some("3306".to_string()));
    let db_user = env_var::<String>(DATABASE_USER, None);
    let db_pass = env_var::<String>(DATABASE_PASS, None);
    let db_name = env_var::<String>(DATABASE_NAME, None);

    assert_ne!(db_host, "");
    assert_ne!(db_port, "");
    assert_ne!(db_user, "");
    assert_ne!(db_pass, "");
    assert_ne!(db_name, "");

    let db_url = format!(
        "mysql://{user}:{pass}@{host}:{port}/{name}",
        user = db_user,
        pass = db_pass,
        host = db_host,
        port = db_port,
        name = db_name
    );

    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);

    let manager = MysqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder()
        .max_size(DATABASE_POOL_SIZE)
        .build(manager).unwrap());

    let items = query_table_outline(pool, db_name);
    for item in items {
        println!(
            "TableOutline\n\
            table_name: {}\n\
            table_comment: {}\n\
            table_fqn: {}\n",
            item.table_name,
            item.table_comment.unwrap(),
            item.table_fqn
        );
    }
}

fn env_var<T>(name: &str, def_var: Option<T>) -> T where T: FromStr {
    let var: Result<String, _> = env::var(name);
    match var {
        Ok(v) => {
            let parsed = v.parse::<T>();
            match parsed {
                Ok(p) => p,
                _ => def_var.expect(format!("{} must be set", name).as_str()),
            }
        },
        _ => def_var.expect(format!("{} must be set", name).as_str()),
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
            })
        .map::<Vec<TableOutline>, _>(|result| {
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