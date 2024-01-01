use std::any::Any;
mod my_box;
mod mutex_experiments;

fn main() {
  mutex_experiments::run();
}

fn play_with_boxes(){
    let boxed = my_box::return_a_box_with_an_int();
    //let boxed = my_box::return_a_box_with_an_string();
    let boxed_val = &*boxed;
    if let Some(boxed_int) = boxed_val.downcast_ref::<i32>() {
        println!("my boxed int: {:?}", boxed_int);
    } else {
        println!("the type is not i32");
    }
}
