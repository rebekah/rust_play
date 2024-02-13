use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn create_leaf() -> Rc<Node> {
    Rc::new(
        Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        }
    )
}

fn create_branch(children: Vec<Rc<Node>>) -> Rc<Node> {
  Rc::new(Node{
    value:2, 
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(children),
  })
}

mod tests {
    use super::*;

    #[test]
    fn test_weak_references() {
      let leaf = create_leaf();
      let branch = create_branch(vec![leaf.clone()]);

      assert_eq!(2, Rc::strong_count(&leaf));
      assert_eq!(0, Rc::weak_count(&leaf));

      assert_eq!(1, Rc::strong_count(&branch));
      assert_eq!(0, Rc::weak_count(&branch));

      *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);

      assert_eq!(1, Rc::weak_count(&branch));
    }

    #[test]
    fn test_weak_references_on_delete() {
        let leaf = create_leaf();

        //block created so that branch is deleted before final assertion
        {
            let branch = create_branch(vec![leaf.clone()]);
            *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);
            assert!(leaf.parent.borrow().upgrade().is_some());
        }

        // The weak reference to branch changes to None when the branch no longer exists
        assert!(leaf.parent.borrow().upgrade().is_none());
      }
}

