type Vector2 = nalgebra::Vector2<f64>;

pub const INVALID_ENVELOPE: &str = "Invalid Envelope!";

/// Represents a TAWS Envelope which defines under what conditions an alert should and should not be emitted.
pub struct Envelope<const N: usize> {
    /*/// Defines the operating limits of this envelope. <br/>
    /// For states outside these limits the envelope is not defined.
    limits: Rect, */
    /// Defines the conditions under which an alert should or should not be emitted.
    polygon: Polygon<N>,
}

impl<const N: usize> Envelope<N> {
    /// Creates a new Envelope from the given envelope limits and polygon points.
    /// # Arguments
    /// * `points` - the polygon points which define the envelope. At least 3 points are nessecary. All points must be finite.
    /// ToDO (maybe): Augment the polygon to the envelope limits by adding a new start point and new end points to close the polygon with the envelope limits.
    pub fn try_new(/* limits: Rect, */ points: [Vector2; N]) -> Result<Self, ()> {
        /*if !Self::are_valid_limits(&limits) {
            return Err(());
        }*/

        if !Self::are_points_valid(/* &limits, */ &points) {
            return Err(());
        }

        Ok(Self {
            //limits,
            polygon: Polygon::try_new(points)?,
        })
    }

    /* /// checks wether the given limits are sufficent for an envelope.
    fn are_valid_limits(limits: &Rect) -> bool {
        limits.min.x.is_finite()
            && limits.min.y.is_finite()
            && limits.max.x.is_finite()
            && limits.max.y.is_finite()
    } */

    /// checks whether the given polygon points are sufficent for an envelope.
    /// the polygon must start and end on the envelope limits. all in-between points must be within the envelope limit.
    fn are_points_valid(/* limits: &Rect, */ _points: &[Vector2; N]) -> bool {
        if N < 3 {
            return false;
        }

        /* let first_point = points.first().unwrap();
        let last_point = points.last().unwrap();
        if !limits.is_point_on_boundary(*first_point) || !limits.is_point_on_boundary(*last_point) {
            return false;
        }

        let mid_points = &points[1..N - 1];
        if !mid_points.iter().all(|point| limits.contains(*point)) {
            return false;
        } */

        true
    }

    /* /// Returns whether the envelope is defined for the specified point.
    /// # Arguments
    /// * `point` - The point which is tested
    /// # Returns
    /// * `true` if the given point is within the envelope limits; otherwise `false`.
    pub fn is_within_limits(&self, point: Vector2) -> bool {
        self.limits.contains(point)
    } */

    /// Determines whether the specified (x, y) state is within the envelope.
    /// # Arguments
    /// * `x` - The first state component.
    /// * `y` - The second state compoenent.
    /// # Returns
    /// * `Ok(bool)` - Indicates whether the given state is within the envelope.
    pub fn contains(&self, x: f64, y: f64) -> Result<bool, ()> {
        /* if !self.is_within_limits(point) {
            return Err(());
        } */

        Ok(self.polygon.contains(Vector2::new(x, y)))
    }
}

/// Represents an Polygon.
struct Polygon<const N: usize> {
    /// Points which define the polygon.
    /// the last segement from point[-1] to point[0] is implied.
    points: [Vector2; N],
}

impl<const N: usize> Polygon<N> {
    /// tries to create a new polygon from the given points.
    /// at least 2 points are nessecary. all points must be finite.
    fn try_new(points: [Vector2; N]) -> Result<Self, ()> {
        if N < 2 {
            return Err(());
        }

        if !Self::are_points_valid(&points) {
            return Err(());
        }

        Ok(Self { points })
    }

    /// check given points.
    fn are_points_valid(points: &[Vector2; N]) -> bool {
        points
            .iter()
            .all(|point| point.x.is_finite() && point.y.is_finite())
    }

    /// returns the points of the polygon.
    fn points(&self) -> &[Vector2; N] {
        &self.points
    }

    /* /// returns the axis-aligned bounding-box for the polygon.
    fn bounding_box(&self) -> Rect {
        let mut min = Vector2::new(f64::INFINITY, f64::INFINITY);
        let mut max = Vector2::new(f64::NEG_INFINITY, f64::NEG_INFINITY);

        for p in self.points {
            if p.x < min.x {
                min.x = p.x;
            } else if p.x > max.x {
                max.x = p.x;
            }

            if p.y < min.y {
                min.y = p.y;
            } else if p.y > max.y {
                max.y = p.y;
            }
        }

        Rect::new(min, max)
    } */

    /// returns an iterator over
    fn segments(&self) -> impl Iterator<Item = LineSegment> + '_ {
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

/* /// Represents an axis-aligned rectangle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
struct Rect {
    /// lower left corner
    min: Vector2,
    /// upper right corner
    max: Vector2,
}

impl Rect {
    /// Creates a rectanlge from two points.
    fn new(p1: Vector2, p2: Vector2) -> Rect {
        let min = Vector2::new(f64::min(p1.x, p2.x), f64::min(p1.y, p2.y));
        let max = Vector2::new(f64::max(p1.x, p2.x), f64::max(p1.y, p2.y));

        Rect { min, max }
    }

    /// Returns the four corner points starting from the lower left point going in clockwise direction.
    fn points(&self) -> [Vector2; 4] {
        let dy = Vector2::new(0.0, self.max.y - self.min.y);
        [self.min, self.min + dy, self.max, self.max - dy]
    }

    /// Iterates over the four sides of the rectangle starting from the left vertical segment in clockwise direction.
    fn segments(&self) -> impl Iterator<Item = LineSegment> + '_ {
        let points = self.points();
        (0..4).map(move |i| (points[i], points[(i + 1) % 4]).into())
    }

    /// Determines whether the specified point is within or on the reactangle.
    fn contains(&self, point: Vector2) -> bool {
        (self.min.x <= point.x && point.x <= self.max.x)
            && (self.min.y <= point.y && point.y <= self.max.y)
    }

    /// Determines whether the specified point is on the rectangle boundary.
    fn is_point_on_boundary(&self, point: Vector2) -> bool {
        if !self.contains(point) {
            return false;
        }

        point.x == self.min.x
            || point.x == self.max.y
            || point.y == self.min.y
            || point.y == self.max.y
    }
} */

/// Represents a line segment.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct LineSegment {
    /// First point which defines the line segment.
    point1: Vector2,
    /// Second point which defines the line segment.
    point2: Vector2,
}

impl LineSegment {
    /// Create a new line segment.
    pub fn new(p1: Vector2, p2: Vector2) -> Self {
        Self {
            point1: p1,
            point2: p2,
        }
    }

    /// Returns the mid point of the line segment.
    pub fn mid(&self) -> Vector2 {
        self.point1 + 0.5 * (self.point2 - self.point1)
    }

    /// Determines whether the specified point is on the line segment.
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

    /// Translates the line segment by the specified offset.
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
    fn test_polygon_contains() {
        let points = [
            Vector2::zeros(),
            Vector2::new(0.0, 100.0),
            Vector2::new(100.0, 100.0),
            Vector2::new(100.0, 0.0),
        ];
        let poly = Polygon::try_new(points).unwrap();

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
        let poly = Polygon::try_new(points).unwrap();

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

        let poly = Polygon::try_new(points).unwrap();
        assert!(points.iter().all(|point| poly.contains(*point)));
        assert!(poly
            .segments()
            .map(|seg| seg.mid())
            .all(|point| poly.contains(point)));

        assert!(!poly.contains(Vector2::new(9.0, 9.0)));
        assert!(!poly.contains(Vector2::zeros()));
    }

    #[test]
    fn test_envelope() {
        /* let limits = Rect {
            min: Vector2::zeros(),
            max: Vector2::new(100.0, 100.0),
        }; */

        let points = [
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 75.0),
            Vector2::new(100.0, 75.0),
        ];

        let envelope = Envelope::try_new(/* limits, */ points).unwrap();

        /* assert!(envelope.is_within_limits(Vector2::zeros()));
        assert!(envelope.is_within_limits(Vector2::new(100.0, 100.0)));
        assert!(envelope.is_within_limits(Vector2::new(0.0, 100.0)));
        assert!(envelope.is_within_limits(Vector2::new(100.0, 0.0)));
        assert!(!envelope.is_within_limits(Vector2::new(150.0, 150.0))); */

        assert!(!envelope.contains(0.0, 0.0).unwrap());
        assert!(!envelope.contains(100.0, 100.0).unwrap());
        assert!(envelope.contains(50.0, 50.0).unwrap());
        assert!(envelope.contains(150.0, 150.0) == Err(()));
    }
}
