#[cfg(test)]
pub mod tests {
    use std::any::Any;

    struct Demo {}
    trait Hello {
        fn hello(&self);
    }

    impl Hello for Demo {
        fn hello(&self) {
            println!("Demo Hello");
        }
    }
    #[test]
    fn test_type() {
        let _box: Box<dyn Hello> = Box::new(Demo {});
        let _any: Box<dyn Any> = Box::new(Demo {});
        let _any1: Box<dyn Any> = Box::new(1);
        println!("&str {:?}", std::any::Any::type_id(""));
        println!("&i32 {:?}", std::any::Any::type_id(&3));
        println!("&Demo {:?}", std::any::Any::type_id(&Demo {}));
        println!("&BoxI32 {:?}", std::any::Any::type_id(&Box::new(1)));
        println!("&Boxstr {:?}", std::any::Any::type_id(&Box::new("")));
        println!("&BoxHello {:?}", std::any::Any::type_id(&_box));
        println!("&BoxHello {:?}", std::any::type_name::<dyn Hello>());
        let hello = (&_any as &dyn Any)
            .downcast_ref::<Box<dyn Hello>>()
            .unwrap();
        hello.hello();
        // let hello1 = _any1.downcast_ref::<Box<dyn Hello>>().unwrap();
        // hello1.hello()
    }
}
