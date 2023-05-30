// Task : Creating a nested Module
// Author: Litmus
// Version: 1.0.0
// Date: May 30,2023


mod sample_mod{
   pub fn function(){
        println!("A private function `sample_mod::nested_mod::function()`" );
    }

    #[allow(dead_code)]

    fn private_function(){
        println!("Called `sample_mod::nested_mod::private_function()`");
    }

  pub mod private_nested_mod{
        pub fn function()  {
            println!("called sample_mod::private_nested_mod::function()");
            }

    }
}

fn main(){
    sample_mod::function();
    sample_mod::private_nested_mod::function();
}