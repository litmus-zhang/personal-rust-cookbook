// Task: create a module with struct
// AUthor: Litmus Zhang
// Version : 1.0.0
// Date: 30 May 2023

mod sample_struct{
    pub struct WhiteBox<T>{
        pub information: T,
    }

    #[allow(dead_code)]
    pub struct BlackBox<T>{
        information : T,
    }

    impl<T> BlackBox<T>{
        pub fn const_new(information: T) -> BlackBox<T>{
            BlackBox{
                information : information
            }
        }
    }
}

fn main(){
    let white_box = sample_struct::WhiteBox{information: "public information n"};

    println!("whitebox contains {} \n", white_box.information );
 //   let black_box = sample_struct::BlackBox{information: "classified information n"};

    //println!("black box contains {} \n", black_box.information );
}