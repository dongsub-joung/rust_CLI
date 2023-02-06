fn main() {
    let mut list= Vec::new();
    let one= String::from("One");
    list.push(one);
    let list= deleting(list);

    let two= String::from("Two");
    let result= list.contains(&two);

    fn deleting(mut list: Vec<String> -> Vec<String>{
        // receive the index
        //let two= String::from("Two"); 
        
        list.remove(two);
        list
    }

}
