pub fn return_a_box_with_an_int() -> Box<dyn std::any::Any> {
    return Box::new(5);
}

pub fn return_a_box_with_an_string() -> Box<dyn std::any::Any> {
    return Box::new("my string");
}

//fn play_with_boxes(){
//  let boxed = my_box::return_a_box_with_an_int();
//  //let boxed = my_box::return_a_box_with_an_string();
//  let boxed_val = &*boxed;
//  if let Some(boxed_int) = boxed_val.downcast_ref::<i32>() {
//      println!("my boxed int: {:?}", boxed_int);
//  } else {
//      println!("the type is not i32");
//  }
//}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_box_with_string() {
        let boxed = return_a_box_with_an_string();
        let boxed_val = &*boxed;
        let is_str = if let Some(_) = boxed_val.downcast_ref::<&str>() {
            true
        } else {
            false
        };
        assert!(is_str);
    }

    #[test]
    fn test_box_with_int() {
        let boxed = return_a_box_with_an_int();
        let boxed_val = &*boxed;
        let is_str = if let Some(_) = boxed_val.downcast_ref::<i32>() {
            true
        } else {
            false
        };
        assert!(is_str);
    }
}
