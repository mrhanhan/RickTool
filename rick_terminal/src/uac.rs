use std::env::args;


/// 基于UAC 提权
/// 程序会启动 此程序 -> uac.exe [UAC ID] [UAC 数据输入管道名称] [UAC 数据输出管道名称]
fn main() {
    for ref x in args() {
        println!("Args:{}", x);
    }
    println!("Hello World");
}