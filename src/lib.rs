


pub struct StackString20 {
    data: [u8; 20],
    len: u8,
}

impl StackString20 {

    pub fn new(input: &str) -> Self {
        let mut data = [0;20];
        let str_bytes = input.as_bytes();
        data[..str_bytes.len()].copy_from_slice(str_bytes);
        StackString20 {
            data,
            len: str_bytes.len() as u8
        }
    }
    pub fn get_string(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(&self.data[..self.len as usize])
        }
    }
}


// fn main() {
//     println!("Hello, world!");
//     println!("{:?}", std::mem::size_of::<StackString>());
//     println!("{:?}", StackString::new("test").get_string());
// }
