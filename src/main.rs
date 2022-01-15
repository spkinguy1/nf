use std::fs::DirBuilder;
use std::time::Duration;
use std::thread::sleep;
fn main() {
    DirBuilder::new()
        .recursive(true)
        .create("./dir_builder1/nested/nested1")
        .unwrap();

    DirBuilder::new()
        .recursive(true)
        .create("./dir_builder2")
        .unwrap();

    println!("Direcoties created ! ,\n
    don't forget remove directories beside the program\n
    and the program will end after10 secconds");

    let secconds = Duration::from_secs(1);
    for i in 1..=10 {
        println!("{}",i);
        sleep(secconds);
    }
}
