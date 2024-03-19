#[derive(Debug)]
struct ListObject{
    //id:u16,
    name:String,
    content:String,
    completed:bool 
}

fn main() {
    let mut todoos:Vec<ListObject> = Vec::new();
    //let mut uptoo: u16 = 0;
    addtodo(&mut todoos);
    dbg!(&todoos[0]);
}

fn addtodo(todoos: &mut Vec<ListObject>){
    let completed:bool = false;
    let mut name:String = String::new();
    let mut content:String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    std::io::stdin().read_line(&mut content).unwrap();
    todoos.push(ListObject{completed, name, content});

}

fn displaytodo(todoos: &mut Vec<ListObject>){
    
}