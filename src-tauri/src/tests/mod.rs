mod tests {

    #[test]
    fn test_sql() {
        
        let _conn = sqlite::Connection::open("../db/test.db").unwrap();
        _conn.execute("INSERT INTO users VALUES ('Alice', 42); 
        INSERT INTO users VALUES ('Bob', 69);").unwrap();
        let mut count = 0;
        _conn.iterate("SELECT * FROM users ", |pairs| {
            for &(name, value) in pairs.iter() {
                let _name = name;
                let _value = value.unwrap();
                println!("{} = {}", name, _value);
                count = count + 1;
            }
            true
        }).unwrap();
        println!("{:?}", "Hello World");
        assert_eq!(count, 4, "{count}")
    }

}