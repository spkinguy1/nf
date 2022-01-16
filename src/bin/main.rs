use std::env;
use std::fs::DirBuilder;

// how to use ? a CLI program
// pass arguments when you run the program
// , and it would make directory for you

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "!help" {
        println!(
            "Created by Mahdi Rashidi ! 
    
    => description \n
    build nested folders by \"/\" and create sblings folders by seperate \",\"

    ***NOTE : the last element after\",\" would be a parent again for rest of nested_folders

    by [space] you can specify another path and create another folders

    Command example : 

    folder folder_name1/nested1,ntestd2,nested3/nested3_1 folder_name2 folder_name3
        |
        #folder_name1/#nested_1 -- #nested2 -- #nested_3/nested3_1
        #folder_name2
        #folder_name3
    "
        )
    }else {


    for i in 1..args.len() {
        let mut buffer: Vec<Vec<&str>> = vec![];

        let parser_1: Vec<&str> = args[i].split("/").collect();

        for f in 0..parser_1.len() {
            let parser_2: Vec<&str> = parser_1[f].split(",").collect();

            buffer.push(parser_2);
        }

        // println!("{:?}",buffer);
        // Gets whole the parsed values as Vector in Vector

        let mut repo: Vec<String> = vec![]; // the repo for create Dirs from this

        for g in 0..buffer.len() {
            let delimiter = repo.last().unwrap_or(&"".to_owned()).clone();

            for d in 0..buffer[g].len() {
                if repo.len() == 0 {
                    repo.push(buffer[g][d].to_owned() + "/");
                } else {
                    repo.push(String::from(&delimiter) + buffer[g][d] + "/");
                }
            }
        }
        // let fix_manual_bug = repo.remove(0);
        // drop(fix_manual_bug);

        for s in 0..repo.len() {
            DirBuilder::new()
                .recursive(true)
                .create(&repo[s])
                .unwrap()
        }
    }
}
    // thread::sleep(Duration::from_millis(500));
    // println!("hello");

    // thread::sleep(Duration::from_millis(500));
    // println!("Folders created don't forget to remove them after run the program");
    // thread::sleep(Duration::from_millis(3000));
    // println!("programm will end after 3 secconds");
    // thread::sleep(Duration::from_millis(500));

    // for i in 1..=3 {

    //     println!("{}",i);
    //     thread::sleep(Duration::from_millis(1000));

    // }
}
