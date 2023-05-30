use deeply::nested::sample_function as other_function;

mod deeply{
    pub mod nested{
        pub fn sample_function(){
            println!("called deeply::nested::function() \n", );
        }
    }

    fn sample_function(){
        println!("called function() \n");
    }


}

