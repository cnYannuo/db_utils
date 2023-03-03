// Copyright (C) 2023-2024 db_utils Inc.
// This file is part of the db_utils library.

// The db_utils library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The db_utils library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the db_utils library. If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

use dao::db_tool;

pub mod dao;
mod test;

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