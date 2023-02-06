fn main() {
    let mut list= Vec::new();
    let one= String::from("One");
    list.push(one);
    let list= adding(list);

    let two= String::from("Two");
    let result= list.contains(&two);
    println!("{}", result);

    fn adding(mut list: Vec<String>) -> Vec<String> {
        let two= String::from("Two");
        list.push(two);

        list 
    }
}
