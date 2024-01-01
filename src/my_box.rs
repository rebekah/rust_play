pub fn return_a_box_with_an_int() -> Box<dyn std::any::Any> {
  return Box::new(5);
}

pub fn return_a_box_with_an_string() -> Box<dyn std::any::Any> {
  return Box::new("my string");
}
