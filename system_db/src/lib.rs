use std::collections::HashMap;

use dao::db_tool;

pub mod dao;

pub trait CommonDbImpl {

    fn get_count(table: &str) -> usize {
        db_tool::get_count(table)
    }

    /// 查看信息
    fn select(table: &str, id: u32) -> Self;

    /**
        批量查看信息
    */
    fn batch_select(table: &str, page: u32, limit: u32) -> Vec<Self> where Self: Sized;

    /**
        创建一个角色
    */
    fn insert(&self, table: &str);

    /*
        修改一个用户
    */
    fn update(&self, table: &str);

    /*
        删除一个用户
    */
    fn delete(table: &str, id: u32) {
        db_tool::delete(table, HashMap::from([("id", id.to_string().as_str())]));
    }
}

// pub trait CommonOtherDbImpl {
//     /**
//         创建一个角色
//     */
//     fn insert(&self, table: &str);

//     /*
//         修改一个用户
//     */
//     fn update(&self, table: &str);
// }