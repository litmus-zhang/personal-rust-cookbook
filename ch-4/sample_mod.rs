// -- #################
// -- Task: To create a sample module to illustrate how to use module in rust
// -- Author: Litmus Zhang
// -- Version: 2023.0.0
// -- Date: 4 March 17
// -- #################

mod sample_mod {
    //  All items in modules are private by default
    fn private_function() {
        println!("Called `sample_mod::private_function()`\n");
    }

    pub fn sample_function() {
        println!("Called `sample_mod::sample_function()` \n ");
    }

    pub fn indirect_private_fn() {
        print!("Called `sample_mod::indirect_access()`, that \n");
        private_function();
    }
    

}





fn sample_function() {
    println!("Called the `sample_function()` which is not a part of mod `sample_mod` \n");
}

fn main(){
    sample_function();

    sample_mod::indirect_private_fn();
    sample_mod::sample_function();
   // sample_mod::private_function();
}
