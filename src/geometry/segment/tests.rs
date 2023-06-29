#[cfg(test)]
use crate::geometry::point::Point;

#[cfg(test)]
use crate::geometry::segment::Segment;

#[test]
fn new_segment() {
    let s = Segment::new([2.0, 2.0], [4.0, 4.0]);

    assert_eq!(s.start, [4.0, 4.0].into());
    assert_eq!(s.end, [2.0, 2.0].into());
}

#[test]
fn new_segment_reverse() {
    let s = Segment::new([4.0, 4.0], [2.0, 2.0]);

    assert_eq!(s.start, [4.0, 4.0].into());
    assert_eq!(s.end, [2.0, 2.0].into());
}

#[test]
fn partial_eq_segment() {
    //todo!()
}

#[test]
fn segment_contains_point() {
    let s = Segment::new([0.0, 0.0], [2.0, 2.0]);
    let p = Point::from2d(2.0, 2.0);

    assert!(s.contains(&p));
}

#[test]
fn segment_constains_point_limits() {
    let s = Segment::new([0.0, 4.0], [2.0, 2.0]);
    let p = Point::from2d(2.0, 2.0);

    assert!(s.contains(&p));
}

#[test]
fn segment_doesnt_contains_point() {
    let s = Segment::new([0.0, 0.0], [2.0, 2.0]);
    let p = Point::from2d(3.0, 3.0);

    assert_eq!(s.contains(&p), false);
}

#[test]
fn find_interpolation() {
    let s1 = Segment::new([0.0, -2.0], [0.0, 2.0]);
    let s2 = Segment::new([-2.0, 0.0], [2.0, 0.0]);

    let opt_point = Segment::find_intersection(&s1, &s2);
    assert!(opt_point.is_some());

    let point = opt_point.unwrap();
    assert_eq!(point, Point::from2d(0.0, 0.0));
}

#[test]
fn find_interpolation_parallel() {
    let s1 = Segment::new([0.0, 0.0], [4.0, 0.0]);
    let s2 = Segment::new([2.0, 0.0], [4.0, 0.0]);

    assert_eq!(Segment::find_intersection(&s1, &s2), None)
}

#[test]
fn find_interpolation_incident_none() {
    let s1 = Segment::new([0.0, 0.0], [4.0, 4.0]);
    let s2 = Segment::new([0.0, 4.0], [1.0, 3.0]);

    assert_eq!(Segment::find_intersection(&s1, &s2), None)
}

#[test]
fn find_interpolation_incident() {
    let s1 = Segment::new([0.0, 0.0], [4.0, 4.0]);
    let s2 = Segment::new([0.0, 4.0], [2.0, 2.0]);

    assert!(s1.contains(&Point::from2d(2.0, 2.0)));
    assert!(s2.contains(&Point::from2d(2.0, 2.0)));
    assert_eq!(
        Segment::find_intersection(&s1, &s2).unwrap(),
        Point::from2d(2.0, 2.0)
    );
}

#[test]
fn display_segment() {
    let s = Segment::new([0.0, 0.0], [4.0, 4.0]);
    assert_eq!(format!("{}", s), "[(4.0, 4.0), (0.0, 0.0)]");
}

#[test]
fn segment_into_point_2() {
    let seg: [Point; 2] = Segment::new([2.0, 3.0], [4.0, 5.0]).into();

    assert_eq!(seg[0].x, 4.0);
    assert_eq!(seg[0].y, 5.0);

    assert_eq!(seg[1].x, 2.0);
    assert_eq!(seg[1].y, 3.0);
}
