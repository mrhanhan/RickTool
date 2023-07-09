#[cfg(test)]
pub mod tests {
    use std::io::Read;
    use std::time::Duration;

    #[derive(Debug)]
    pub struct Global {
        data: i32,
    }


    static mut g: Option<Global> = None;

    #[test]
    pub fn test_sync() {
        unsafe {g = Some(Global { data: 0 });}
        std::thread::spawn(|| unsafe {
            for _ in 0..100000 {
                let x = g.as_mut().unwrap();
                x.data = x.data + 1;
            }
        });

        std::thread::spawn(|| unsafe {
            for _ in 0..100000 {
                let x = g.as_mut().unwrap();
                x.data = x.data + 1;
            }
        });

        std::thread::spawn(|| unsafe {
            for _ in 0..100000 {
                let x = g.as_mut().unwrap();
                x.data = x.data + 1;
            }
        });
        std::thread::spawn(|| unsafe {
            for _ in 0..100000 {
                let x = g.as_mut().unwrap();
                x.data = x.data + 1;
            }
        }).join().expect("panic message");
        std::thread::sleep(Duration::from_millis(1000));
        unsafe {
            println!("{:?}", g);
        }
    }
}