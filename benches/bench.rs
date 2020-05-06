
#![feature(test)]

extern crate test;
// extern crate rust_stack_string_bench;

use rust_stack_string_bench::StackString20;

struct MyComplexStruct {
    pub field1: String,
    pub field2: String,
    pub some_num: usize,
    pub field3: String,
}

struct MyComplexStructStack {
    pub field1: StackString20,
    pub field2: StackString20,
    pub some_num: usize,
    pub field3: StackString20,
}


#[bench]
fn test_heap(b: &mut test::Bencher) {
    b.iter(|| MyComplexStruct{
        field1: "name".to_string(),
        field2: "adress streed".to_string(),
        some_num: 20,
        field3: "age".to_string(),
    })
}

#[bench]
fn test_stack(b: &mut test::Bencher) {
    b.iter(|| MyComplexStructStack{
        field1: StackString20::new("name"),
        field2: StackString20::new("adress streed"),
        some_num: 20,
        field3: StackString20::new("age"),
    })
}
