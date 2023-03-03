use std::{time::{SystemTime, UNIX_EPOCH}, collections::HashMap};

use db_utils_config::db_config::Db;
use log::error;
use mysql::{PooledConn, Opts, Pool, prelude::FromRow};
use lazy_static::lazy_static;
use redis::{Client, Connection};
use mysql::prelude::Queryable;

lazy_static! {
    static ref DB_POOL: Pool = {
        let url = Opts::from_url(&Db::get_db_url()).unwrap();
        match Pool::new(url) {
            Ok(pool) => pool,
            Err(err) => {
                error!("mysql error: {}", err);
                panic!("mysql error: {}", err)
            },
        }
    };
    
    static ref REDIS_CLIENT: Client = redis::Client::open(Db::get_redis_url()).unwrap();
}

// 获取条数
/// # Examples
/// 转换为： "select count(*) from table_name"
pub fn get_count(table_name: &str) -> usize {
    let mut con = get_db_conn();
    con.query_first(format!("select count(*) from {}", table_name)).unwrap().unwrap()
}

// 搜查  得到全部字段
/// # Examples
/// ```
///     let mut filters: HashMap<&str, &str> = HashMap::new();
///     filters.insert("k1", "v1");    
///     filters.insert("k2", "v2");  
/// ```
/// 转换为： "SELECT * from table_name where k1 = v1 and k2 = v2;"
pub fn select<T: FromRow>(table_name: &str, filters: HashMap<&str, &str>) -> T {
    let mut conn = get_db_conn();
    let str1 = format_and_filters(filters);
    conn.query_first(format!("SELECT * from {} where {}", table_name, str1)).unwrap().unwrap()
}

// 或搜查  得到全部字段
/// # Examples
/// ```
///     let mut filters: HashMap<&str, &str> = HashMap::new();
///     filters.insert("k1", "v1"); 
///     filters.insert("k2", "v2");     
/// ```
/// 转换为： "SELECT * from table_name where k1 = v1 or k2 = v2;"
pub fn select_or<T: FromRow>(table_name: &str, filters: HashMap<&str, &str>) -> T {
    let mut conn = get_db_conn();
    let str1 = format_or_filters(filters);
    conn.query_first(format!("SELECT * from {} where {}", table_name, str1)).unwrap().unwrap()
}

/// 批量搜查
/// 转换为： "SELECT * from table_name order by order_by limit page,limit;"
pub fn batch_select<T: FromRow>(table_name: &str, page: u32, limit: u32, order_by: &str) -> Vec<T> {
    let mut con = get_db_conn();
    con.query(format!("select * from {} order by {} limit {},{}", table_name, order_by, (page-1)*limit, limit), ).unwrap()
}

/// 批量搜查or  得到全部字段
/// # Examples
/// ```
///     let mut filters: HashMap<&str, &str> = HashMap::new();
///     filters.insert("key", "value"); 
///     filters.insert("k2", "v2");     
/// ```
/// 转换为： "SELECT * from table_name where k1 = v1 or k2 = v2 order by order_by limit page,limit;"
pub fn batch_select_or<T: FromRow>(table_name: &str, page: u32, limit: u32, order_by: &str, filters: HashMap<&str, &str>) -> Vec<T> {
    let mut con = get_db_conn();
    let str = format_or_filters(filters);
    con.query(format!("select * from {} where {} order by {} limit {},{}", table_name, str, order_by, (page-1)*limit, limit), ).unwrap()
}

/// 批量搜查and  得到全部字段
/// # Examples
/// ```
///     let mut filters: HashMap<&str, &str> = HashMap::new();
///     filters.insert("key", "value"); 
///     filters.insert("k2", "v2");     
/// ```
/// 转换为： "SELECT * from table_name where k1 = v1 and k2 = v2 order by order_by limit page,limit;"
pub fn batch_select_and<T: FromRow>(table_name: &str, page: u32, limit: u32, order_by: &str, filters: String) -> Vec<T> {
    let mut con = get_db_conn();
    con.query(format!("select * from {} where {} order by {} limit {},{}", table_name, filters, order_by, (page-1)*limit, limit), ).unwrap()
}

/// 创建
/// # Examples
/// ```
///     let mut fields: HashMap<&str, &str> = HashMap::new();
///     fields.insert("key", "value");    
/// ```
/// 转换为： "insert into table_name (key) values (value)"
pub fn insert(table_name: &str, fields: HashMap<&str, &str>) {

    if fields.len() > 0 {
        let mut con = get_db_conn();
        let (mut str1, mut str2) = fields.iter().fold(("".to_string(), "".to_string()), 
        |(fild_str, v_str), (k, v)| {
            (format!("{}`{}`,", fild_str, k), format!("{},{}", v_str, v))
        });
        // 去掉最后的,
        str1.remove(str1.len() - 1);
        // 去掉第一个
        str2.remove(0);
        con.query_drop(format!("insert into {} ({}) values ({})", table_name, str1, str2)).unwrap();
    }

}

// 修改 
///  args: params 的顺序一定是fields在前  filters在后
/// # Examples
/// ```
///     let mut fields: HashMap<&str, &str> = HashMap::new();
///     fields.insert("field", "value");
///     let mut filters: HashMap<&str, &str> = HashMap::new();
///     filters.insert("key", "value");    
/// ```
/// 转换为： "UPDATE table_name SET field = value where key = value;"
pub fn update(table_name: &str, fields: HashMap<&str, &str>, filters: HashMap<&str, &str>) {

    if fields.len() > 0 && filters.len() > 0 {
        let mut con = get_db_conn();

        let str1 = format_fields(fields);

        let str2 = format_and_filters(filters);
        con.query_drop(format!("UPDATE {} SET {} where {};", table_name, str1, str2)).unwrap();

    }
}

/// 删除
/// # Examples
/// ```
///     let mut filters: HashMap<&str, &str> = HashMap::new();
///     filters.insert("key", "value");    
/// ```
/// 转换为： "DELETE FROM table_name where key = value;"
pub fn delete(table_name: &str, filters: HashMap<&str, &str>) {
    let mut con = get_db_conn();
    let str1 = format_and_filters(filters);
    con.query_drop(format!("DELETE FROM {} where {};", table_name, str1)).unwrap();
}

// // 批量删除
// pub fn batch_delete<T>(table_name: &str, filters:Vec<&str>, params: T) 
// where Params: From<T>{
//     let mut con = get_db_conn();
//     let str1 = format_in_filters(filters);
//     con.exec_drop(format!("DELETE FROM {} where {};", table_name, str1), params).unwrap();
// }

fn format_fields(fields: HashMap<&str, &str>) -> String {
    let mut str1 = fields.iter().fold("".to_string(), 
    |fild_str, (k, v)| {
        if *v != "" {
            format!("{} {} = {},", fild_str, k, v)
        }else {
            fild_str
        }
    });

    // 去掉','
    str1.remove(str1.len() - 1);
    str1
}

fn format_and_filters(filters: HashMap<&str, &str>) -> String {
    let str2 = filters.iter().fold("".to_string(), 
        |fild_str, (k, v)| {
            if *v != "" {
                format!("{} {} = {} and", fild_str, k, v)
            }
            else {
                fild_str
            }
        });

    // 去掉'and'
    let (str2, _) = str2.split_at(str2.len() - 3);    
    // str2.remove(str2.len() - 1);
    // str2.remove(str2.len() - 1);
    // str2.remove(str2.len() - 1);
    // str2
    str2.to_string()
}

fn format_or_filters(filters: HashMap<&str, &str>) -> String {
    let str2 = filters.iter().fold("".to_string(), 
        |fild_str, (k, v)| {
            if *v != "" {
                format!("{} {} = {} or", fild_str, k, v)
            }else {
                fild_str
            }
        });
    // 去掉'or'
    let (str2, _) = str2.split_at(str2.len() - 2);    
    // str2.remove(str2.len() - 1);
    // str2.remove(str2.len() - 1);
    // str2.remove(str2.len() - 1);
    // str2
    str2.to_string()
}

// fn format_in_filters(filters: Vec<&str>) -> String {
//     let mut str2 = filters.iter().fold("".to_string(), 
//         |fild_str, v| {
//             format!("{} {} in ? or", fild_str, v)
//         });

//     str2.remove(str2.len() - 1);
//     str2.remove(str2.len() - 1);
//     str2
// }

/// 获得mysql连接
pub fn get_db_conn() -> PooledConn {
    DB_POOL.get_conn().unwrap()
}

/// 获得redis连接
pub fn get_redis_client() -> Connection {
    REDIS_CLIENT.get_connection().unwrap()
}

/// 获得当前时间戳
pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}