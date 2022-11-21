
fn main() {
    const MAX_POINTS:u32 = 100_1000;
    // const mut MAX_POINTS:u32 = 100_1000;

    let mut points:u32=100;
    points=100_1000;

    assert_eq!(MAX_POINTS, points);
}
