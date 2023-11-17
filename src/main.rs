use std::io;

fn delete_by_text(value: &String, todo: &mut Vec<String>){
    let mut result: Vec<String> = Vec::new();
    for i in 0..todo.len(){
        if value != &todo[i]{
            result.push(todo[i].to_string())
        }
    }
    todo.clear();
    todo.extend(result);
}

fn main() {
    let mut todo: Vec<String> = Vec::new();
    

    loop{
        let size = todo.len();

        println!("RUST TODO APP");
        println!("1. Option 1");
        println!("2. Option 2");
        println!("3. Show History");
        println!("4. Test Function");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        

        match choice.trim() {
            "1" => {
                println!("Your Todo");
                println!("-----------------------------");
                if size != 0 {
                    for i in 0..size {
                        println!("{}", todo[i]);
                    }
                } else {
                    println!("Your Todo Still 0")
                }
                println!("-----------------------------");
                
            }
            "2" => {
                println!("Add Todo");
                let mut add = String::new();
                io::stdin().read_line(&mut add).expect("Failed");
                todo.push(add);
            }
            "3" => {
                println!("Remove Todo By Id");
                let mut idx = String::new();
                io::stdin().read_line(&mut idx).expect("Failed");
                let index = idx.trim().parse::<usize>().unwrap();
                todo.remove(index);
            }
            "4" => {
                println!("Remove Todo By Text");
                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed");
                delete_by_text(&text.to_string(), &mut todo);
            }
            "5" => {
                return;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }
    

}
