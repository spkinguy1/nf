use std::env;
use std::fs::DirBuilder;

// how to use ? a CLI program
// pass arguments when you run the program
// , and it would make directory for you

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if the first argument is "!help" , shows how to use !

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
    // otherwise parse arguments and gather them in a vector    
    } else {

        // this big for loop will execute per argument
        //arguments seperates by [space] in runtime
        
        for i in 1..args.len() {

            //this is vector parsed first by "/" and then ","
            //initialize empty
            let mut buffer: Vec<Vec<&str>> = vec![];

            // parse command by "/" and make it into vector
            let parser_1: Vec<&str> = args[i].split("/").collect();
            
            // parse each element in vector by "," and take it back again
            for f in 0..parser_1.len() {
                let parser_2: Vec<&str> = parser_1[f].split(",").collect();

                buffer.push(parser_2);
            }
            

            // create repo vector from buffer and make a
            // repo that is resource for create Dirs from it
            let mut repo: Vec<String> = vec![]; 
            
            //for each element in buffer
            for g in 0..buffer.len() {

                // we try to do that by example :
                // command : a,b/z/f
                // [a , b ,b/z , b/z/f]
                let delimiter = repo.last().unwrap_or(&"".to_owned()).clone();
                
                //for each element in elements in buffer
                for d in 0..buffer[g].len() {
                    if repo.len() == 0 {
                        repo.push(buffer[g][d].to_owned() + "/");
                    } else {
                        repo.push(String::from(&delimiter) + buffer[g][d] + "/");
                    }
                }
            }

            // create directories from the repo

            for s in 0..repo.len() {
                DirBuilder::new().recursive(true).create(&repo[s]).unwrap()
            }
        }
    }
}
