use clearscreen;
use std::io::{Read, Write};
use std::fs::{read_to_string, File};
use std::path::Path;
use homedir;

struct ListObject{
    name:String,
    content:String 
}

fn main() {
    let mut save_path = homedir::get_my_home().unwrap().unwrap().to_string_lossy().to_string();
    save_path.push_str("/.config/ToDo_data");
    let mut todoos:Vec<ListObject> = Vec::new();
    if !Path::new(&save_path).exists(){savetofile(&mut todoos, &save_path);}
    else{loadsaved(&mut todoos, &save_path);}
    loop{
        savetofile(&mut todoos, &save_path);
        displaylist(&todoos);
        let mut inputbuffer:String = String::new();
        println!("\n[A]dd [S]crap [D]isplay-content modi[F]y");
        print!("command: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inputbuffer).unwrap();
        let inputbuffer:char = inputbuffer.chars().nth(0).unwrap().to_ascii_lowercase();
        clearscreen::clear().unwrap();
        if inputbuffer == 'a'{
            addtodo(&mut todoos);
        }
        else if inputbuffer=='d'{
            showcontents(&todoos);
        }
        else if inputbuffer == 's'{
            removeeliment(&mut todoos);
        }
        else if inputbuffer == 'f'{
            editeliment(&mut todoos);
        }
    }
}

fn addtodo(todoos: &mut Vec<ListObject>){
    let mut name:String = String::new();
    let mut content:String = String::new();
    print!("name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();
    clearscreen::clear().unwrap();
    print!("contnent: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut content).unwrap();
    clearscreen::clear().unwrap();
    name.pop();
    content.pop();
    todoos.push(ListObject{name, content});

}

fn displaylist(todoos: &Vec<ListObject>){
    clearscreen::clear().unwrap();
    println!("id:  name:");
    for i in 0..todoos.len(){
        println!("{}    {}", i+1, todoos[i].name);
    }
    std::io::stdout().flush().unwrap();
}

fn showcontents(todoos: &Vec<ListObject>){
    displaylist(todoos);
    let mut inputbuffer: String = String::new();
    print!("\ncontent ID: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    let inputbuffer:u8 = match inputbuffer.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };
    if inputbuffer > todoos.len() as u8 || inputbuffer <= 0{
        return;
    }
    println!("\n{}", todoos[inputbuffer as usize-1].content);
    print!("\npress enter to continue... ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read(&mut[0]).unwrap();
}

fn removeeliment(todoos: &mut Vec<ListObject>){
    displaylist(todoos);
    let mut inputbuffer: String = String::new();
    print!("\nA for all.   todo ID: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    if inputbuffer.chars().nth(0).unwrap().to_ascii_lowercase() == 'a'
    {
        todoos.clear();
        return;
    }
    let inputbuffer:u8 = match inputbuffer.trim().parse(){
        Ok(num) => num,
        Err(_) => return
    };
    if inputbuffer > todoos.len() as u8 || inputbuffer <= 0{
        return;
    }
    todoos.remove(inputbuffer as usize-1);
}

fn editeliment(todoos: &mut Vec<ListObject>){
    displaylist(todoos);
    let mut inputbuffer: String = String::new();
    print!("todo ID: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    let index:u8 = match inputbuffer.trim().parse() {
        Ok(num) => num,
        Err(_) => return
    };
    let index: u8 = index-1;
    if index > todoos.len() as u8 /*|| index < 0*/{return;}
    
    clearscreen::clear().unwrap();
    inputbuffer.clear();
    println!("current name: {}", todoos[index as usize].name);
    print!("enter for no change. change name to: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    if inputbuffer != "\n"{
        inputbuffer.pop();
        todoos[index as usize].name = inputbuffer.clone();
    }
    clearscreen::clear().unwrap();
    inputbuffer.clear();
    println!("current content: {}", todoos[index as usize].content);
    print!("enter for no change. change content to: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    if inputbuffer != "\n"{
        inputbuffer.pop();
        todoos[index as usize].content = inputbuffer.clone();
    }
}

fn loadsaved(todoos: &mut Vec<ListObject>, save_path: &String){
    let mut buff:String = String::new();
    let mut name:String = String::new();
    let filecontent: String = read_to_string(&save_path).unwrap();
    for i in filecontent.chars(){
        buff.push(i);
        if i =='█'{
            name = buff.clone();
            buff.clear();
        }
        else if i == '\n'{
            buff.pop();
            name.pop();
            todoos.push(ListObject{name:name.clone(), content:buff.clone()});
            buff.clear();
        }
    }
    
}

fn savetofile(todoos: &mut Vec<ListObject>, save_path: &String){
    let mut file = File::create(&save_path).unwrap();
    for i in todoos{
        write!(file, "{}█{}\n", i.name, i.content).unwrap();
    }
}