fn main() {
    let mut list: Vec<usize>= Vec::new();
    list.push(1);
    println!("{:?}", list);
    
    list= adding(list);
    println!("{:?}", list);

    fn adding(mut list: Vec<usize>)-> Vec<usize>{
        list.push(2);
        list
    }
}
