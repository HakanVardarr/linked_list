use linked_list::List;

#[test]
fn list_pop() {
    let mut list = List::new();
    list.push(10);

    assert_eq!(Some(10), list.pop());
    assert_eq!(None, list.pop());
}
#[test]
fn list_len() {
    let mut list = List::new();
    list.push(10);
    list.push(20);
    list.push(30);

    assert_eq!(3, list.len());
}
#[test]
fn list_peak() {
    let mut list = List::new();
    list.push(10);

    assert_eq!(Some(&10), list.peak());
}
#[test]
fn list_peak_mut() {
    let mut list = List::new();
    list.push(10);

    list.peak_mut().map(|value| *value += 10);

    assert_eq!(Some(&mut 20), list.peak_mut());
}
#[test]
fn list_reverse() {
    let mut list = List::new();
    list.push(10);
    list.push(20);
    list.push(30);

    list.reverse();
}
#[test]
fn list_into_iter() {
    let mut list = List::new();
    list.push(10);

    let mut list_into_iter = list.into_iter();
    assert_eq!(Some(10), list_into_iter.next());
}
#[test]
fn list_iter() {
    let mut list = List::new();
    list.push(10);

    let mut list_iter = list.iter();
    assert_eq!(Some(&10), list_iter.next());
}
#[test]
fn list_iter_mut() {
    let mut list = List::new();
    list.push(10);

    list.iter_mut().next().map(|value| *value += 1);
    assert_eq!(Some(&mut 11), list.iter_mut().next())
}
