//Unit Enum
enum MyUnitEnum {
    VariantOne,
    VariantTwo,
    VariantThree
}

enum MyTupleEnum {
    VariantOne(i32),
    VariantTwo{number: i32, name: String},
    VariantThree(f64, String),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unit_enums_with_if_let() {
    let my_enum = MyUnitEnum::VariantThree;
    let success = if let MyUnitEnum::VariantThree = my_enum {
        true
    } else {
        false
    };

    assert!(success);
  }

  #[test]
  fn test_tuple_enums_with_one_val_if_let() {
    let my_enum_with_number = MyTupleEnum::VariantOne(0);

    let success: bool = if let MyTupleEnum::VariantOne(num) = my_enum_with_number {
      num == 0
    } else {
      false
    };

    assert!(success);
  }

  #[test]
  fn test_tuple_enums_with_named_vals_if_let() {
    let my_enum_with_fields = MyTupleEnum::VariantTwo{number: 1, name: "One".to_string()};

    let success: bool = if let MyTupleEnum::VariantTwo{number, name}   = my_enum_with_fields {
      number == 1 && name == "One"
    } else {
      false
    };

    assert!(success);
  }

  #[test]
  fn test_tuple_enums_with_multiple_vals_if_let() {
    let my_enum_with_multiple_vals = MyTupleEnum::VariantThree(1.0, "One".to_string());;

    let success = if let MyTupleEnum::VariantThree(num, name) = my_enum_with_multiple_vals {
      num == 1.0 && name == "One"
    } else {
      false
    };

    assert!(success);
  }

  #[test]
  fn test_enums_with_if_let_fails() {
    let my_tuple_enum = MyTupleEnum::VariantOne(0);

    let success: bool = if let MyTupleEnum::VariantTwo{number, name} = my_tuple_enum {
      true
    } else {
      false
    };

    assert!(!success);
  }
}