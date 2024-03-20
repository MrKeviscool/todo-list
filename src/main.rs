use clearscreen::{self, clear};
use std::io::{Read, Write};
use std::fs:: read_to_string;
use std::fs::File;
use std::path::Path;

const SAVEPATH: &str = "/home/daniel/.config/ToDo_data";

//#[derive(Debug)]
struct ListObject{
    name:String,
    content:String,
    completed:bool 
}

fn main() {
    let mut todoos:Vec<ListObject> = Vec::new();
    if !Path::new(SAVEPATH).exists(){savetofile(&mut todoos);}
    loadsaved(&mut todoos);
    loop{
        savetofile(&mut todoos);
        displaylist(&todoos);
        let mut inputbuffer:String = String::new();
        println!("\n\n[A]dd [S]crap [D]isplay-content [F]inish-task");
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
        else if inputbuffer == 's'{
            removeeliment(&mut todoos);
        }
        else if inputbuffer == 'f'{
            togglecompletion(&mut todoos);
        }
    }
}

fn addtodo(todoos: &mut Vec<ListObject>){
    let completed:bool = false;
    let mut name:String = String::new();
    let mut content:String = String::new();
    print!("name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();
    clear().unwrap();
    print!("contnent: ");
    std::io::stdout().flush().unwrap();
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
        if todoos[i].completed == false {
            println!(" {}  [{}]     {}", i, todoos[i].completed, todoos[i].name);
        }
        else{
            println!(" {}  [{}]      {}", i, todoos[i].completed, todoos[i].name);
        }
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
    if inputbuffer > todoos.len() as u8{
        return;
    }
    println!("\n{}", todoos[inputbuffer as usize].content);
    print!("\npress enter to continue... ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read(&mut[0]).unwrap();
}

fn removeeliment(todoos: &mut Vec<ListObject>){
    displaylist(todoos);
    let mut inputbuffer: String = String::new();
    print!("\nA for all. D for all done.   remove ID: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    if inputbuffer.chars().nth(0).unwrap().to_ascii_lowercase() == 'a'
    {
        todoos.clear();
        return;
    }
    else if inputbuffer.chars().nth(0).unwrap().to_ascii_lowercase() == 'd'{
        for i in (0..todoos.len()).rev(){
            if todoos[i].completed == true {
                todoos.remove(i);
                return;
            }
        }
    }
    let inputbuffer:u8 = match inputbuffer.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };
    if inputbuffer+1 > todoos.len() as u8{
        return;
    }
    todoos.remove(inputbuffer as usize);
}

fn togglecompletion(todoos: &mut Vec<ListObject>){
    displaylist(todoos);
    let mut inputbuffer: String = String::new();
    print!("\nA for all.  complete ID: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputbuffer).unwrap();
    if inputbuffer.chars().nth(0).unwrap().to_ascii_lowercase() == 'a'{
        for i in 0..todoos.len(){
            todoos[i].completed = true;
        }
        return;
    }
    let inputbuffer:u8 = match inputbuffer.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };
    if inputbuffer+1 > todoos.len() as u8{
        return;
    }
    if todoos[inputbuffer as usize].completed == true{
        todoos[inputbuffer as usize].completed = false;
    }
    else{
        todoos[inputbuffer as usize].completed = true;
    }
}

fn loadsaved(todoos: &mut Vec<ListObject>){
    let mut buff:String = String::new();
    let mut name:String = String::new();
    let mut data:String = String::new();
    let filecontent: String = read_to_string(SAVEPATH).unwrap();
    for i in filecontent.chars(){
        buff.push(i);
        if i =='-'{
            name = buff.clone();
            buff.clear();
        }
        else if i=='_'{
            data=buff.clone();
            buff.clear();
        }
        else if i=='\n'{
            let mut completed:bool = false;
            if buff.chars().count()-1 < 5 {
                completed = true;
            }
            name.pop();
            data.pop();
            todoos.push(ListObject{name:name.clone(), content:data.clone(), completed});
            //println!("{:?}", todoos);
            buff.clear();
        }
    }
    
}

fn savetofile(todoos: &mut Vec<ListObject>){
    let mut file = File::create(SAVEPATH).unwrap();
    for i in todoos{
        write!(file, "{}-{}_{}\n", i.name, i.content, i.completed).unwrap();
    }
    

}