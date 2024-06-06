pub struct Buffer {
   pub lines: Vec<String>,
}

impl Default for Buffer {
   fn default() -> Buffer {
       Self { lines: vec!["Hello, World from buffer. ".to_string()]}
   }
}
