fn main() {
    let mut list: Vec<usize>= Vec::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    println!("{:?}", list);
    
    list= deleting(list);
    
    println!("{:?}", list);

    
    fn deleting(mut list: Vec<usize>)-> Vec<usize>{
        list.remove(0);

        list
    }
}
