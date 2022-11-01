use core::fmt::Display;

use crate::prelude::*;

pub type Vector2 = nalgebra::Vector2<f64>;

#[derive(Debug)]
pub enum EnvelopeError {
    InvalidPolygon,
    InvalidLimits,
    PolygonNotWithinLimits,
    LimitViolation,
}

impl Display for EnvelopeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EnvelopeError::InvalidPolygon => f.write_fmt(format_args!("Invalid polygon.")),
            EnvelopeError::InvalidLimits => {
                f.write_fmt(format_args!("Envelope limits must be finite."))
            }
            EnvelopeError::PolygonNotWithinLimits => f.write_fmt(format_args!(
                "Polygon points must be within envelope limits."
            )),
            EnvelopeError::LimitViolation => {
                f.write_fmt(format_args!("Envelope is not defined for the given point."))
            }
        }
    }
}

impl TawsError for EnvelopeError {}

impl From<EnvelopeError> for &dyn TawsError {
    fn from(err: EnvelopeError) -> Self {
        match err {
            EnvelopeError::InvalidPolygon => &EnvelopeError::InvalidPolygon,
            EnvelopeError::InvalidLimits => &EnvelopeError::InvalidLimits,
            EnvelopeError::PolygonNotWithinLimits => &EnvelopeError::PolygonNotWithinLimits,
            EnvelopeError::LimitViolation => &EnvelopeError::LimitViolation,
        }
    }
}

pub struct Envelope<const N: usize> {
    limits: Rect, //ToDo: make const generic
    polygon: Polygon<N>,
}

impl<'a, const N: usize> Envelope<N> {
    pub fn new(limits: Rect, points: &'a [Vector2; N]) -> Result<Self, EnvelopeError> {
        let polygon = Polygon::new(points).map_err(|_| EnvelopeError::InvalidPolygon)?;

        if !Self::are_valid_limits(&limits) {
            return Err(EnvelopeError::InvalidLimits);
        }

        if !Self::is_polygon_within_limits(&limits, &polygon) {
            return Err(EnvelopeError::PolygonNotWithinLimits);
        }

        Ok(Self { limits, polygon })
    }

    fn are_valid_limits(limits: &Rect) -> bool {
        limits.min.x.is_finite()
            && limits.min.y.is_finite()
            && limits.max.x.is_finite()
            && limits.max.y.is_finite()
    }

    fn is_polygon_within_limits(limits: &Rect, polygon: &Polygon<N>) -> bool {
        polygon.points.iter().all(|point| limits.contains(*point))
    }

    pub fn contains(&self, x: f64, y: f64) -> Result<bool, EnvelopeError> {
        let point = Vector2::new(x, y);

        if !self.limits.contains(point) {
            return Err(EnvelopeError::LimitViolation);
        }

        Ok(self.polygon.contains(Vector2::new(x, y)))
    }
}

#[derive(Debug)]
pub enum PolygonError<'a> {
    TooFewPoints(usize),
    InvalidPoints(&'a [Vector2]),
}

impl<'a> Display for PolygonError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PolygonError::TooFewPoints(n) => f.write_fmt(format_args!(
                "At least two points are needed to form a valid polygon. {n} were given."
            )),
            PolygonError::InvalidPoints(points) => f.write_fmt(format_args!(
                "Polygon points must be finite. Points: {points:?}"
            )),
        }
    }
}

pub struct Polygon<const N: usize> {
    points: [Vector2; N],
}

impl<'a, const N: usize> Polygon<N> {
    pub fn new(points: &'a [Vector2; N]) -> Result<Self, PolygonError> {
        if N < 2 {
            return Err(PolygonError::TooFewPoints(N));
        }

        if !Self::are_points_valid(points) {
            return Err(PolygonError::InvalidPoints(points));
        }

        Ok(Self { points: *points })
    }

    fn are_points_valid(points: &[Vector2]) -> bool {
        points
            .iter()
            .all(|point| point.x.is_finite() && point.y.is_finite())
    }

    pub fn points(&self) -> &[Vector2] {
        &self.points
    }

    pub fn segments(&self) -> impl Iterator<Item = LineSegment> + '_ {
        (0..N).map(|i| (self.points[i], self.points[(i + 1) % N]).into())
    }

    /// determines whether the specfied point is within or on the polygon boundary.
    /// works for all kind of polygons: simple, non-convex and non-simple (self-intersecting).
    /// https://www.engr.colostate.edu/~dga/documents/papers/point_in_polygon.pdf
    fn contains(&self, point: Vector2) -> bool {
        let mut winding_num = 0.0;

        for segment in self.segments() {
            if segment.contains(point) {
                return true;
            }

            let (p1, p2) = segment.translate(-point).into();

            if p1.y * p2.y < 0.0 {
                let r = p1.x + (p1.y * (p2.x - p1.x)) / (p1.y - p2.y);

                if r > 0.0 {
                    if p1.y < 0.0 {
                        winding_num += 1.0;
                    } else {
                        winding_num -= 1.0;
                    }
                }
            } else if p1.y == 0.0 && p1.x > 0.0 {
                if p2.y > 0.0 {
                    winding_num += 0.5;
                } else {
                    winding_num -= 0.5;
                }
            } else if p2.y == 0.0 && p2.x > 0.0 {
                if p1.y < 0.0 {
                    winding_num += 0.5;
                } else {
                    winding_num -= 0.5;
                }
            }
        }

        winding_num != 0.0
    }
}

/// Represents an axis-aligned rectangle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Rect {
    /// lower left corner
    min: Vector2,
    /// upper right corner
    max: Vector2,
}

impl Rect {
    /// Creates a rectanlge from two points.
    pub fn new(point1: Vector2, point2: Vector2) -> Rect {
        let min = Vector2::new(f64::min(point1.x, point2.x), f64::min(point1.y, point2.y));
        let max = Vector2::new(f64::max(point1.x, point2.x), f64::max(point1.y, point2.y));

        Rect { min, max }
    }

    /// Returns the four corner points starting from the lower left point going in clockwise direction.
    pub fn points(&self) -> [Vector2; 4] {
        let dy = Vector2::new(0.0, self.max.y - self.min.y);
        [self.min, self.min + dy, self.max, self.max - dy]
    }

    /// Iterates over the four sides of the rectangle starting from the left vertical segment in clockwise direction.
    pub fn segments(&self) -> impl Iterator<Item = LineSegment> + '_ {
        let points = self.points();
        (0..4).map(move |i| (points[i], points[(i + 1) % 4]).into())
    }

    /// Determines whether the specified point is within or on the reactangle.
    pub fn contains(&self, point: Vector2) -> bool {
        (self.min.x <= point.x && point.x <= self.max.x)
            && (self.min.y <= point.y && point.y <= self.max.y)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct LineSegment {
    point1: Vector2,
    point2: Vector2,
}

impl LineSegment {
    pub fn new(point1: Vector2, point2: Vector2) -> Self {
        Self { point1, point2 }
    }

    pub fn mid(&self) -> Vector2 {
        self.point1 + 0.5 * (self.point2 - self.point1)
    }

    pub fn contains(&self, point: Vector2) -> bool {
        let ab = self.point2 - self.point1;
        let ac = point - self.point1;

        if ab.magnitude() == 0.0 {
            return point == self.point1;
        }

        let cross_prod = (ab.x * ac.y) - (ac.x * ab.y);
        if cross_prod != 0.0 {
            return false;
        }

        let ab_dot_ac = (ab.x * ac.x) + (ab.y * ac.y);
        if ab_dot_ac < 0.0 {
            return false;
        } else if ab_dot_ac == 0.0 {
            return true;
        }

        let ab_dot_ab = (ab.x * ab.x) + (ab.y * ab.y);
        if ab_dot_ac > ab_dot_ab {
            return false;
        } else if ab_dot_ac == ab_dot_ab {
            return true;
        }

        true
    }

    pub fn translate(&self, offset: Vector2) -> LineSegment {
        (self.point1 + offset, self.point2 + offset).into()
    }
}

impl From<(Vector2, Vector2)> for LineSegment {
    fn from(segment: (Vector2, Vector2)) -> Self {
        Self::new(segment.0, segment.1)
    }
}

impl From<LineSegment> for (Vector2, Vector2) {
    fn from(segment: LineSegment) -> Self {
        (segment.point1, segment.point2)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;

    use super::*;

    #[test]
    fn test_segment_mid() {
        let l: LineSegment = (Vector2::zeros(), Vector2::zeros()).into();
        assert!(l.mid() == Vector2::zeros());

        let l: LineSegment = (Vector2::new(100.0, 100.0), Vector2::new(100.0, 100.0)).into();
        assert!(l.mid() == Vector2::new(100.0, 100.0));

        let l: LineSegment = (Vector2::zeros(), Vector2::new(100.0, 100.0)).into();
        assert!(l.mid() == Vector2::new(50.0, 50.0));
    }

    #[test]
    fn test_segment_contains() {
        let l: LineSegment = (Vector2::zeros(), Vector2::new(100.0, 100.0)).into();
        assert!(l.contains(Vector2::zeros()));
        assert!(l.contains(Vector2::new(50.0, 50.0)));
        assert!(!l.contains(Vector2::new(101.0, 101.0)));

        let l: LineSegment = (Vector2::new(10.0, 10.0), Vector2::new(10.0, 10.0)).into();
        assert!(l.contains(Vector2::new(10.0, 10.0)));
        assert!(!l.contains(Vector2::new(9.0, 9.0)));
        assert!(!l.contains(Vector2::zeros()))
    }

    #[test]
    fn test_segment_translate() {
        let l: LineSegment = (Vector2::zeros(), Vector2::zeros()).into();
        let l = l.translate(Vector2::new(100.0, -100.0));
        assert!(l.point1 == Vector2::new(100.0, -100.0) && l.point2 == Vector2::new(100.0, -100.0));

        let l: LineSegment = (Vector2::zeros(), Vector2::new(100.0, 100.0)).into();
        let l = l.translate(Vector2::new(100.0, -100.0));
        assert!(l.point1 == Vector2::new(100.0, -100.0) && l.point2 == Vector2::new(200.0, 0.0));
    }

    #[test]
    fn test_rect_points() {
        let r = Rect::new(Vector2::zeros(), Vector2::zeros());
        assert!(r.points().iter().all(|&p| p == Vector2::zeros()));

        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        assert!(r.points().iter().enumerate().all(|(i, &p)| match i {
            0 => p == Vector2::zeros(),
            1 => p == Vector2::new(0.0, 100.0),
            2 => p == Vector2::new(100.0, 100.0),
            3 => p == Vector2::new(100.0, 0.0),
            _ => false,
        }));
    }

    #[test]
    fn test_rect_segments() {
        let r = Rect::new(Vector2::zeros(), Vector2::zeros());
        assert!(r
            .segments()
            .all(|seg| seg == LineSegment::new(Vector2::zeros(), Vector2::zeros())));

        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        assert!(r.segments().enumerate().all(|(i, seg)| match i {
            0 => seg == LineSegment::new(Vector2::zeros(), Vector2::new(0.0, 100.0)),
            1 => seg == LineSegment::new(Vector2::new(0.0, 100.0), Vector2::new(100.0, 100.0)),
            2 => seg == LineSegment::new(Vector2::new(100.0, 100.0), Vector2::new(100.0, 0.0)),
            3 => seg == LineSegment::new(Vector2::new(100.0, 0.0), Vector2::new(0.0, 0.0)),
            _ => false,
        }));
    }

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(Vector2::zeros(), Vector2::zeros());
        assert!(r.contains(Vector2::zeros()));
        assert!(!r.contains(Vector2::new(1.0, 1.0)));

        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        assert!(r.points().iter().all(|&p| r.contains(p)));
        assert!(r.contains(Vector2::new(50.0, 50.0)));
        assert!(!r.contains(Vector2::new(101.0, 101.0)));
    }

    #[test]
    fn test_polygon_new() {
        assert!(matches!(
            Polygon::new(&[Vector2::zeros()]),
            Err(PolygonError::TooFewPoints(1))
        ));

        assert!(matches!(
            Polygon::new(&[
                Vector2::new(100.0, 100.0),
                Vector2::new(f64::INFINITY, 150.0),
            ]),
            Err(PolygonError::InvalidPoints(_))
        ));

        assert!(matches!(
            Polygon::new(&[Vector2::new(100.0, 100.0), Vector2::new(f64::NAN, 150.0),]),
            Err(PolygonError::InvalidPoints(_))
        ))
    }

    #[test]
    fn test_polygon_points() {
        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        let p = Polygon::new(&r.points()).unwrap();

        assert!(p
            .points()
            .iter()
            .zip(r.points().iter())
            .all(|(p1, p2)| p1 == p2))
    }

    #[test]
    fn test_polygon_segments() {
        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        let p = Polygon::new(&r.points()).unwrap();

        assert!(p
            .segments()
            .zip(r.segments())
            .all(|(seg1, seg2)| seg1 == seg2))
    }

    #[test]
    fn test_polygon_contains() {
        let points = [
            Vector2::zeros(),
            Vector2::new(0.0, 100.0),
            Vector2::new(100.0, 100.0),
            Vector2::new(100.0, 0.0),
        ];
        let poly = Polygon::new(&points).unwrap();

        assert!(points.iter().all(|point| poly.contains(*point)));
        assert!(poly
            .segments()
            .map(|segment| segment.mid())
            .all(|point| poly.contains(point)));

        assert!(!poly.contains(Vector2::new(-0.01, -0.01)));
        assert!(!poly.contains(Vector2::new(-0.01, 100.01)));
        assert!(!poly.contains(Vector2::new(100.01, 100.01)));
        assert!(!poly.contains(Vector2::new(100.01, -0.01)));

        let points = [
            Vector2::new(100.0, 100.0),
            Vector2::new(100.0, 200.0),
            Vector2::new(150.0, 200.0),
            Vector2::new(200.0, 50.0),
            Vector2::new(250.0, 200.0),
            Vector2::new(300.0, 200.0),
            Vector2::new(300.0, 100.0),
        ];
        let poly = Polygon::new(&points).unwrap();

        assert!(points.iter().all(|point| poly.contains(*point)));
        assert!(poly
            .segments()
            .map(|seg| seg.mid())
            .all(|point| poly.contains(point)));

        assert!(!poly.contains(Vector2::new(200.0, 150.0)));
        assert!(poly.contains(Vector2::new(200.0, 75.0)));

        let points = [
            Vector2::new(10.0, 10.0),
            Vector2::new(10.0, 10.0),
            Vector2::new(10.0, 10.0),
            Vector2::new(10.0, 10.0),
        ];

        let poly = Polygon::new(&points).unwrap();
        assert!(points.iter().all(|point| poly.contains(*point)));
        assert!(poly
            .segments()
            .map(|seg| seg.mid())
            .all(|point| poly.contains(point)));

        assert!(!poly.contains(Vector2::new(9.0, 9.0)));
        assert!(!poly.contains(Vector2::zeros()));
    }

    #[test]
    fn test_envelope_new() {
        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        let p = [Vector2::zeros()];
        let e = Envelope::new(r, &p);
        assert!(matches!(e, Err(EnvelopeError::InvalidPolygon)));

        let r = Rect::new(Vector2::zeros(), Vector2::new(f64::INFINITY, 100.0));
        let p = [
            Vector2::zeros(),
            Vector2::new(0.0, 100.0),
            Vector2::new(100.0, 100.0),
            Vector2::new(0.0, 100.0),
        ];
        let e = Envelope::new(r, &p);
        assert!(matches!(e, Err(EnvelopeError::InvalidLimits)));

        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        let p = [
            Vector2::zeros(),
            Vector2::new(0.0, 100.0),
            Vector2::new(101.0, 101.0),
            Vector2::new(0.0, 100.0),
        ];
        let e = Envelope::new(r, &p);
        assert!(matches!(e, Err(EnvelopeError::PolygonNotWithinLimits)));

        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        let p = [
            Vector2::zeros(),
            Vector2::new(100.0, 0.0),
            Vector2::new(100.0, 100.0),
        ];
        let e = Envelope::new(r, &p);
        assert!(matches!(e, Ok(_)));
    }

    #[test]
    fn test_envelope_contains() {
        let r = Rect::new(Vector2::zeros(), Vector2::new(100.0, 100.0));
        let e = Envelope::new(
            r,
            &[
                Vector2::new(100.0, 100.0),
                Vector2::new(100.0, 0.0),
                Vector2::new(0.0, 0.0),
            ],
        )
        .unwrap();

        assert!(matches!(e.contains(0.0, 0.0), Ok(true)));
        assert!(matches!(e.contains(100.0, 0.0), Ok(true)));
        assert!(matches!(e.contains(100.0, 100.0), Ok(true)));
        assert!(matches!(e.contains(50.0, 50.0), Ok(true)));
        assert!(matches!(e.contains(50.0, 50.0), Ok(true)));
        assert!(matches!(e.contains(75.0, 50.0), Ok(true)));

        assert!(matches!(e.contains(0.0, 100.0), Ok(false)));
        assert!(matches!(e.contains(0.0, 50.0), Ok(false)));
        assert!(matches!(e.contains(25.0, 50.0), Ok(false)));

        assert!(matches!(
            e.contains(-1.0, 50.0),
            Err(EnvelopeError::LimitViolation)
        ));

        assert!(matches!(
            e.contains(101.0, 50.0),
            Err(EnvelopeError::LimitViolation)
        ));

        assert!(matches!(
            e.contains(50.0, -1.0),
            Err(EnvelopeError::LimitViolation)
        ));

        assert!(matches!(
            e.contains(50.0, 101.0),
            Err(EnvelopeError::LimitViolation)
        ));
    }
}
