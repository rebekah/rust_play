use std::cell::RefCell;

struct Test {
    list: Vec<i32>,
    a: i32
}

impl Test {
    pub fn new() -> Self {
        Test { list:vec![1,2,3,4,5,6,7], a:0 }
    }

    pub fn run(&mut self) {
        for i in (0..self.list.len()) {
            self.do_something(self.list[i])
        }

    }

    pub fn do_something(&mut self, n: i32) {
        self.a = n;
    }
}

// FIX the error without removing any code line
struct TestWithRefCell {
    list: Vec<i32>,
    a: RefCell<i32>
}

impl TestWithRefCell {
    pub fn new() -> Self {
        TestWithRefCell { list:vec![1,2,3,4,5,6,7], a:RefCell::new(0) }
    }

    pub fn run(&self) {
        for i in self.list.iter() {
            self.do_something(*i)
        }

    }

    pub fn do_something(&self, n: i32) {
        *self.a.borrow_mut() = n;
    }
}

#[cfg(test)]
mod  tests {
    use super::*;

    #[test]
    fn test_run() {
      let mut test = Test::new();
      test.run();
      assert_eq!(test.a, 7);
    }

    #[test]
    fn test_with_refcell_run() {
      let test = TestWithRefCell::new();
      test.run();
      assert_eq!(*(test.a.borrow()), 7);
    }
}