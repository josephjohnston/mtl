// here we need to generate
// our kernels will hav input and output.
//

#[derive(Debug)]
pub struct Function {
    string: String,
}

impl Function {
    pub fn new() -> Self {
        Self {
            string: String::with_capacity(1 << 10),
        }
    }
    pub fn for_loop(&mut self) -> &mut Self {
        let x = format!("s {}", self.string);
        self.string.push_str(&x);
        self.string.push_str(&x);
        self
    }
    pub fn print(&self) {}
}
