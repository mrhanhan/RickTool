use std::sync::Arc;

fn main() {
    //Must be run as Administrator because we create network adapters
//Load the wintun dll file so that we can call the underlying C functions
//Unsafe because we are loading an arbitrary dll file
    let wintun = unsafe { wintun::load_from_path(r"E:\Rick\RickTools\rick_net\lib\wintun\bin\amd64\wintun.dll") }
        .expect("Failed to load wintun dll");

    let _a = match wintun::Adapter::open(&wintun, "Demo") {
        Ok(a) => a,
        Err(_) => {
            //If loading failed (most likely it didn't exist), create a new one
            wintun::Adapter::create(&wintun, "Example", "Demo", None)
                .expect("Failed to create wintun adapter!")
        }
    };
    let session = Arc::new(_a.start_session(wintun::MAX_RING_CAPACITY).unwrap());
    //Get a 20 byte packet from the ring buffer
    for _ in 0..100000 {
        if let Ok(_a) = session.receive_blocking() {
            println!("{:?}", _a.bytes());
        }
    }

    session.shutdown();

    println!("Hello");
}
