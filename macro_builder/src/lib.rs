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


extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, Data, Fields};

#[proc_macro_derive(CommonDbMacro)]
pub fn common_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate

    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_db_macro(&ast)
}

fn impl_db_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // 提取结构体里的字段
    let expanded = match ast.data {
        Data::Struct(DataStruct{ref fields,..}) => {
            if let Fields::Named(ref fields_name) = fields {
                // 结构体中可能是多个字段
                let field_rows: Vec<_> = fields_name.named.iter().enumerate().map(|(index, field)| {
                    let field_name = field.ident.as_ref().unwrap(); // 字段名字
                    quote! {
                        #field_name: row.get(#index).unwrap(),
                    }
                }).collect();

                let filed_names: Vec<_> = fields_name.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap(); // 字段名字
                    field_name.to_string()
                }).collect();

                let fileds_values: Vec<_> = fields_name.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap(); // 字段名字
                    quote! {
                        self.#field_name.to_string()
                    }
                }).collect();

                quote! {
                    
                    // use mysql::{Params, Value};
                    use mysql::prelude::FromRow;
                    use crate::CommonDbImpl;
                    use crate::dao::db_tool;
                    use std::collections::HashMap;

                    impl FromRow for #name {
                        fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
                        where
                            Self: Sized {
                            Ok(
                                #name {
                                    #(#field_rows)*
                                }
                            )
                        }
                    }   
                    
                    impl CommonDbImpl for #name {

                        /// 查看信息
                        fn select(table: &str, id: u32) -> Self {
                            db_tool::select(table, HashMap::from([("id", id.to_string().as_str())]))
                        }

                        /// 批量查看信息
                        fn batch_select(table: &str, page: u32, limit: u32) -> Vec<Self> {
                            db_tool::batch_select::<Self>(table, page, limit, "id")
                        }

                        /// 插入一条数据
                        fn insert(&self, table: &str) {
                            let v1 = vec![#(#filed_names,)*];
                            let v2 = vec![#(#fileds_values,)*];
                            let mut map: HashMap<&str, &str> = HashMap::new();
                            v1.iter().enumerate().for_each(|(index, k)| {
                                map.insert(k, &v2[index]);
                            });
                            map.remove("id");
                            db_tool::insert(table, map);
                        }

                        // //修改一个用户
                        fn update(&self, table: &str) {
                            let v1 = vec![#(#filed_names,)*];
                            let v2 = vec![#(#fileds_values,)*];
                            let mut map: HashMap<&str, &str> = HashMap::new();
                            v1.iter().enumerate().for_each(|(index, k)| {
                                map.insert(k, &v2[index]);
                            });
                            map.remove("id");
                            db_tool::update(table, map, HashMap::from([("id", self.id.to_string().as_str())]));
                        }
                    }    
                }
                
                } else {
                    panic!("sorry, may it's a complicated struct.");
                }
        }
        _ => panic!("sorry, Show is not implemented for union or enum type.")
    };
    expanded.into()
}

