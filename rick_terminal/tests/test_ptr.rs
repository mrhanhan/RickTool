use std::ops::Deref;

fn process(ptr: usize) {
    unsafe {
        let p = ptr as *mut i32;
        *p = 10
    }
}

#[test]
fn test_ptr() {
    let i = Box::new(1);
    process(&*i as *const i32 as usize);
    println!("{}", i);

}