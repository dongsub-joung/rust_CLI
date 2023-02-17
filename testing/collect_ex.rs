fn main() {
    ex1();
    ex2();
    ex3_string()
}

fn ex1(){
  let a= [1,2,3];

  let double: Vec<i32>= a.iter()
      .map(|&x| x*2)
      .collect();
                                    
  assert_eq!(vec![2,4,6] , double);
}

fn ex2(){
  let a= [1,2,3];
    
  let doubled= a.iter().map(|x| x*2).collect::<Vec<i32>>();

  assert_eq!(vec![2,4,6] , doubled);
}

fn ex3_string(){
    let chars = ['g', 'd', 'k', 'k', 'n'];

    let hello: String= chars.iter()
        .map(|&x| x as u8)
        .map(|x| (x+1) as char)
        .collect();
    
    let pre= "hello";
    type_of(&pre);
    type_of(&hello);
    assert_eq!(pre, hello);
}

fn type_of<T>(_: &T){
    println!("{}", std::any::type_name::<T>())
}
