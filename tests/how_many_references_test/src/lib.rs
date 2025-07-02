use std::rc::Rc;

use how_many_references::*;

#[test]
fn test_add_element() {
    let a = Rc::new("a".to_owned());
    let b = Rc::new("b".to_owned());
    let c = Rc::new("c".to_owned());

    let mut new_node = Node::new(vec![Rc::clone(&a)]);
    new_node.add_element(Rc::clone(&a));
    new_node.add_element(Rc::clone(&b));
    new_node.add_element(Rc::clone(&c));

    assert_eq!(new_node.ref_list, vec![Rc::clone(&a), a, b, c]);
}

#[test]
fn test_how_many_references() {
    let a = Rc::new("a".to_owned());
    let b = Rc::new("b".to_owned());
    let c = Rc::new("c".to_owned());
    let d = Rc::new("d".to_owned());

    let mut new_node = Node::new(Vec::new());
    new_node.add_element(b.clone());
    new_node.add_element(a.clone());
    new_node.add_element(c.clone());
    new_node.add_element(a.clone());

    assert_eq!(how_many_references(&d), 1);
    assert_eq!(how_many_references(&a), 3);
    assert_eq!(how_many_references(&b), 2);
    assert_eq!(how_many_references(&c), 2);
}

#[test]
fn test_rm_all_ref() {
    let a = Rc::new("a".to_owned());
    let b = Rc::new("b".to_owned());
    let c = Rc::new("c".to_owned());
    let d = Rc::new("d".to_owned());

    let a1 = Rc::new("a".to_owned());
    let b1 = Rc::new("b".to_owned());
    let c1 = Rc::new("c".to_owned());
    let d1 = Rc::new("d".to_owned());

    let mut new_node = Node::new(vec![
        d.clone(),
        d.clone(),
        b.clone(),
        a.clone(),
        c.clone(),
        a.clone(),
        d.clone(),
    ]);

    new_node.rm_all_ref(a1.clone());
    assert_eq!(how_many_references(&a), 3);
    new_node.rm_all_ref(a.clone());
    assert_eq!(how_many_references(&a), 1);

    new_node.rm_all_ref(b1.clone());
    assert_eq!(how_many_references(&b), 2);
    new_node.rm_all_ref(b.clone());
    assert_eq!(how_many_references(&b), 1);

    new_node.rm_all_ref(c1.clone());
    assert_eq!(how_many_references(&c), 2);
    new_node.rm_all_ref(c.clone());
    assert_eq!(how_many_references(&c), 1);

    new_node.rm_all_ref(d1.clone());
    assert_eq!(how_many_references(&d), 4);
    new_node.rm_all_ref(d.clone());
    assert_eq!(how_many_references(&d), 1);
}
