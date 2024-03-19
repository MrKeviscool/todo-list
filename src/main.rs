use std::io::Write;

use clearscreen::{self, clear};

#[derive(Debug)]
struct ListObject{
    name:String,
    content:String,
    completed:bool 
}

fn main() {
    let mut todoos:Vec<ListObject> = Vec::new();
    loop{
        displaylist(&todoos);
        let mut inputbuffer:String = String::new();
        println!("\n\n[A]dd [D]isplay-content [S]crap");
        print!("command: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inputbuffer).unwrap();
        let inputbuffer:char = inputbuffer.chars().nth(0).unwrap().to_ascii_lowercase();
        clear().unwrap();
        if inputbuffer == 'a'{
            addtodo(&mut todoos);
        }
        else if inputbuffer=='d'{
            showcontents(&todoos);
        }

    }
}

fn addtodo(todoos: &mut Vec<ListObject>){
    let completed:bool = false;
    let mut name:String = String::new();
    let mut content:String = String::new();
    print!("name: ");
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut name).unwrap();
    clear().unwrap();
    print!("contnent: ");
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut content).unwrap();
    clear().unwrap();
    name.pop();
    content.pop();
    todoos.push(ListObject{completed, name, content});

}

fn displaylist(todoos: &Vec<ListObject>){
    clear().unwrap();
    println!("id: completed:  name:");
    for i in 0..todoos.len(){
        println!(" {}  [{}]     {}", i, todoos[i].completed, todoos[i].name);
    }
    std::io::stdout().flush().unwrap();

}

fn showcontents(todoos: &Vec<ListObject>){
    displaylist(todoos);
    print!("\n")
}