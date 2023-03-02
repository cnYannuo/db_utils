#[cfg(test)]
mod test {
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
        let number = User::get_count("table");
        println!("number: {}", number);
        let user_list = User::batch_select("table", 1, 100);
        for item in user_list {
            println!("user: {:?}", item);
        }
        let user = User::new(1, "tester".to_owned(), "earth".to_owned(), 10086);
        user.insert("table");
        user.update("table");
        User::delete("table", 1);
    }
}