use std::fs::File;
use std::io::Write;

const INDENTATION: usize = 4;

pub struct LibraryWriter {
    file: File,
    indent: usize,
    string: String,
}
impl LibraryWriter {
    pub fn init(library_path: String, imports: Vec<String>) -> Self {
        let file = File::options()
            .truncate(true)
            .write(true)
            .open(library_path)
            .unwrap();
        let mut myself = Self {
            file,
            indent: 0,
            string: String::with_capacity(1 << 10),
        };
        for i in 0..imports.len() {
            myself.line(imports[i].clone());
        }
        myself
    }
    pub fn init_kernel(&mut self, kernel_name: String, inputs: Vec<String>) {
        self.empty_line()
            .line(format!("kernel void {kernel_name}("))
            .indent();
        for i in 0..inputs.len() - 1 {
            self.line(format!("{},", inputs[i].clone()));
        }
        self.line(format!("{})", inputs[inputs.len() - 1]))
            .outdent()
            .line(format!("{{"))
            .indent();
    }
    fn indent(&mut self) -> &mut Self {
        self.indent += 1;
        self
    }
    fn outdent(&mut self) -> &mut Self {
        self.indent -= 1;
        self
    }
    pub fn flush(&mut self) -> std::io::Result<usize> {
        self.outdent().line(format!("}}"));
        let result = self.file.write(self.string.as_bytes());
        self.indent = 0;
        self.string = String::new();
        result
    }
    pub fn lines(&mut self, lines: Vec<String>) -> &mut Self {
        for (_, line) in lines.into_iter().enumerate() {
            self.string += &format!("\n{}", " ".repeat(self.indent * INDENTATION));
            self.string += &line;
        }
        self
    }
    pub fn line(&mut self, line: String) -> &mut Self {
        self.lines(vec![line])
    }
    pub fn empty_line(&mut self) -> &mut Self {
        self.lines(vec![format!("")])
    }
    pub fn begin_scope(&mut self) -> &mut Self {
        self.line(format!("{{")).indent()
    }
    pub fn end_scope(&mut self) -> &mut Self {
        self.outdent().line(format!("}}"))
    }
    pub fn begin_for(&mut self, init: String, cond: String, update: String) -> &mut Self {
        let opening = format!("for ({init}; {cond}; {update})");
        self.line(opening).begin_scope()
    }
    pub fn end_for(&mut self) -> &mut Self {
        self.end_scope()
    }
    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.line(format!("// {comment}"))
    }
}
