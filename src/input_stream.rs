pub struct InputStream<'a> {
  input: &'a str,
  current_position: usize,
  current_char: u8,
  current_line: usize,
  current_column: usize,
  file_name: String,
  read_position: usize,
  range_start: usize,
}

impl<'a> InputStream<'a> {
  pub fn new(input: &'a str, file_name: String) -> Self {
    let mut l = InputStream {
      input,
      current_position: 0,
      current_char: 0,
      current_line: 0,
      current_column: 0,
      file_name,
      read_position: 0,
      range_start: 0,
    };
    l.next();
    return l;
  }

  pub fn next(&mut self) {
    if self.current_char == b'\n' || self.current_char == b'\r' {
      self.current_line += 1;
      self.current_column = 0;
    } else {
      self.current_column += 1;
    }
    if self.read_position >= self.input.len() {
      self.current_char = 0;
    } else {
      self.current_char = self.input.as_bytes()[self.read_position];
    }
    self.current_position = self.read_position;
    self.read_position += 1;
  }

  pub fn current(&mut self) -> u8 {
    self.current_char
  }

  pub fn current_location(&self) -> (String, usize, usize) {
    (
      self.file_name.clone(),
      self.current_line,
      self.current_column,
    )
  }

  pub fn prefetch(&mut self) -> u8 {
    if self.read_position >= self.input.len() {
      return 0;
    } else {
      return self.input.as_bytes()[self.read_position];
    }
  }

  pub fn current_to_string(&mut self) -> String {
    String::from_utf8(vec![self.current_char]).unwrap()
  }

  pub fn current_2_to_string(&mut self) -> String {
    self.start_range();
    self.next();
    self.range_to_string()
  }

  pub fn start_range(&mut self) {
    self.range_start = self.current_position;
  }

  pub fn range_to_string(&mut self) -> String {
    let start = self.range_start;
    self.range_start = 0;
    self
      .input
      .get(start..self.current_position)
      .unwrap()
      .to_string()
  }
}
