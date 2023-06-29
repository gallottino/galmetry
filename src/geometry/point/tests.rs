#[cfg(test)]
use crate::geometry::point::Point;

#[test]
fn new_point() {
    let p = Point::new(5.0, 5.0, 5.0);

    assert_eq!(p.x, 5.0);
    assert_eq!(p.y, 5.0);
    assert_eq!(p.z, 5.0);
}

#[test]
fn sub_point() {
    let p1 = Point::new(5.0, 5.0, 5.0);
    let p2 = Point::new(6.0, 5.0, 5.0);
    let p3 = p2 - p1;

    assert_eq!(p3.x, 1.0);
    assert_eq!(p3.y, 0.0);
    assert_eq!(p3.z, 0.0);
}

#[test]
fn distance_from() {
    let p1 = Point::new(5.0, 5.0, 5.0);
    let p2 = Point::new(6.0, 5.0, 5.0);

    let distance = p1.distance_from(&p2);
    assert_eq!(distance, 1.0);
}

#[test]
fn distance_from_2d() {
    let p1 = Point::from2d(0.0, 0.0);
    let p2 = Point::from2d(3.0, 4.0);

    let distance = p1.distance_from(&p2);
    assert_eq!(distance, 5.0);
}

#[test]
fn get_distance() {
    let p1 = Point::from2d(0.0, 0.0);
    let p2 = Point::from2d(3.0, 4.0);

    let distance = Point::distance(&p1, &p2);
    assert_eq!(distance, 5.0);
}

#[test]
fn test_lexicographic_order() {
    let mut p1 = Point::from2d(2.0, 4.0);
    let mut p2 = Point::from2d(3.0, 0.0);

    assert_eq!(p1.lexicograph_cmp(&p2), std::cmp::Ordering::Less);

    p1 = Point::from2d(3.0, 0.0);
    p2 = Point::from2d(3.0, 4.0);

    assert_eq!(p1.lexicograph_cmp(&p2), std::cmp::Ordering::Less);

    p1 = Point::from2d(3.0, 4.0);
    p2 = Point::from2d(3.0, 0.0);

    assert_eq!(p1.lexicograph_cmp(&p2), std::cmp::Ordering::Greater);

    p1 = Point::from2d(0.2, 0.9);
    p2 = Point::from2d(0.1, 0.8);

    println!("{:?}", p1.lexicograph_cmp(&p2));
    assert_eq!(p1.lexicograph_cmp(&p2), std::cmp::Ordering::Greater);
}

#[test]
fn clockwise() {
    let p1 = Point::from2d(3.0, 2.0);
    let p2 = Point::from2d(2.0, 3.0);

    assert!(Point::clockwise(&p1, &p2));
}

#[test]
fn anti_clockwise() {
    let p1 = Point::from2d(1.0, 3.0);
    let p2 = Point::from2d(3.0, 2.0);

    assert!(Point::clockwise(&p1, &p2) == false);
}

#[test]
fn make_right_turn_false() {
    let p1 = Point::from2d(0.0, 0.0);
    let p2 = Point::from2d(0.5, 0.5);
    let p3 = Point::from2d(0.0, 1.0);

    assert_eq!(Point::make_right_turn(&p1, &p2, &p3), false);
}

#[test]
fn make_right_turn_true() {
    let p1 = Point::from2d(0.0, 0.0);
    let p2 = Point::from2d(1.0, 1.0);
    let p3 = Point::from2d(2.0, 0.0);

    assert_eq!(Point::make_right_turn(&p1, &p2, &p3), true);
}

#[test]
fn sweep_line_cmp_points() {
    use std::cmp::Ordering;

    let p1 = Point::from2d(0.0, 0.0);
    let p2 = Point::from2d(1.0, 1.0);

    assert_eq!(p1.sweep_plane_cmp(&p2), Some(Ordering::Greater));
}

#[test]
fn f64_2_from_point() {
    let array_point: [f64; 2] = Point::from2d(2.0, 3.0).into();

    assert_eq!(array_point[0], 2.0);
    assert_eq!(array_point[1], 3.0);
}

#[test]
fn f64_3_from_point() {
    let array_point: [f64; 3] = Point::new(2.0, 3.0, 4.0).into();

    assert_eq!(array_point[0], 2.0);
    assert_eq!(array_point[1], 3.0);
    assert_eq!(array_point[2], 4.0);
}

#[test]
fn display_point() {
    let p = Point::from2d(2.0, 2.0);

    assert_eq!(format! {"{}", p}, "(2.0, 2.0)");
}

#[test]
fn point_eq() {
    let p1 = Point::new(1.2, 0.4, 0.0);
    let p2 = Point::new(1.2, 0.4, 0.0);

    assert!(p1 == p2);
}
