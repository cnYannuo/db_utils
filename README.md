# db_utils
## 主要是为mysql做的一些简化curd操作，集成了许多工具的功能

## db的连接在对应配置里面
### application.toml为第一层的配置文件
### dev为生产环境文件,test为测试环境文件  里面集成了db连接方式

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

    pub trait DbImpl {
        fn get(table_name: &str) -> usize {
            let mut conn = db_tool::get_db_conn();
            conn.query_first(format!("select count(*) from {}", table_name)).unwrap().unwrap()
        }
    }

    impl DbImpl for Payment {

    }

    #[test]
    fn test() {
        // 获取表数据的数量
        let number = User::get_count("table");
        println!("number: {}", number);
        let number2 = User::get("table");
        println!("number2: {}", number2);
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