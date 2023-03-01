pub mod tools{

    fn read_up(){
        //mysql show
    }

    fn upload(){
        //mysql insert
    }

    fn remove(){
        //mysql delte
    }

    pub mod List{
        // or Vec<String>
        pub fn list_up(list: &Vec<String>){
            println!("{:?}", list); 
        }
    }

    pub mod Add{
        pub fn adding(inputed_text: String, list: Vec<String>){         
            let mut list= list.clone();

            // upload();
            list.push(inputed_text);

            list
        }
    }

    pub mod Delte{
        pub fn deleting(index: i32, list: Vec<String>) -> Vec<String> {
            // let mut list= read_up()
            let mut list= list.clone();
           
            //remove(); 
            list.remove(index as usize);

            list
        }
    }
}
