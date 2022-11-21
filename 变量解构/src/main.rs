fn main() {
    let (a, mut b) = (true, false);

    println!("a={:?}, b={:?}", a, b);

    b = true;
    assert_eq!(a, b);
}
