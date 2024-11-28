use std::io;

fn main() {
    let mut items: Vec<String> = Vec::new();

    loop {
        let input_u = input();

        match input_u.as_str() {
            "exit" => break,
            "ls" => {
                for l in &items  {
                    println!("");
                    println!("{}",l);
                }
            } ,
            "Add" => {
                println!("");
                println!("enter item name");
                let new_item = input(); 
                items.push(new_item);    
            },

            "delete" => {
                println!("");
                println!("enter delete item name");
                let del_input = input();
                items.retain(|item| item != &del_input);
            }

            _ => println!("Type something valid"),
        }
    }
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Please enter the text");
    input.trim().to_string()
}
