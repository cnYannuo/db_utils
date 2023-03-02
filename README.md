# db_utils
## 主要是为mysql做的一些简化curd操作，集成了许多工具的功能

## 最重要的是macro_builder中的过程宏，在quote!块里面修改或新增功能，并且在CommonDbImpl trait模块里面添加对应函数

### 例子：
 ```
    use macro_builder::CommonDbMacro;
    use serde_derive::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize, CommonDbMacro)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub address: String,
        pub phone: u32
    }

    impl User {
        pub fn new(id: u32, name: String, address: String, phone: u32) -> Self {
            Self {id, name, address, phone,}
        }
    }


    #[test]
    fn test() {
        // 获取表数据的数量
        let number = User::get_count("table");
        println!("number: {}", number);
        // 获取表数据
        let user_list = User::batch_select("table", 1, 100);
        for item in user_list {
            println!("user: {:?}", item);
        }
        let user = User::new(1, "tester".to_owned(), "earth".to_owned(), 10086);
        // 插入
        user.insert("table");
        // 更新
        user.update("table");
        // 删除
        User::delete("table", 1);
    }
 ```