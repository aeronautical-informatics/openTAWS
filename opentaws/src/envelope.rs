use core::ops::{Div, Mul};

use num::Num;

/// `Envelope` helps checking whether a 2D point is inside of a 2D Envelope - that is a polygon without vertical lines.
pub struct Envelope<X, Y, D, const N: usize> {
    points: [(X, Y); N],
    derivatives: [D; N], // TODO make this N-1
}

impl<X, Y, D, const N: usize> Envelope<X, Y, D, N>
where
    X: num::traits::Num + PartialOrd + Copy,
    Y: num::traits::Num + Div<X, Output = D> + PartialOrd + Copy,
    D: num::traits::Num + Mul<X, Output = Y> + PartialOrd + Copy,
{
    /// Creates an envelope from point pairs
    ///
    /// The point pairs must be orderd by x values in ascending order. There must be no two points
    /// with the same x value. The lower left bound is is the first point.
    ///
    /// Each section between two successive x points is interpolated by a linear function. The
    /// interpolation function of the last section is extendend to +∞. Use an additional point with
    /// the same y value as the prior point to cause an extrapolation parallel to the x axis for
    /// the slope after the last point.
    //
    // # Example
    //
    // ```
    // use opentaws::envelope::Envelope;
    // // The last point ensure that the slope is extended with a function parallel to the x axis
    // let points = vec![(1908, 150), (2050, 300), (10300, 1958), (10301, 1958)];
    // let envelope = Envelope::new(&points).expect("invalid points given to envelope");
    // ```
    pub const fn new(points: [(X, Y); N]) -> Self {
        if N < 2 {
            // List is too small!
            panic!("points does not contain at least 2 points");
        }

        let mut derivatives = [D::zero(); N];

        let mut i = 0;
        loop {
            // only process N-1 elements (with i in 1..N)
            i += 1;
            if i >= N {
                break;
            }

            let (x, y) = points[i - 1];
            let (x_, y_) = points[i];

            // require monotic increase of first dimension
            if x >= x_ {
                panic!("points first dimension is not strictly monotonic");
            }

            derivatives[i] = (y_ - y) / (x_ - x)
        }

        Self {
            points,
            derivatives,
        }
    }

    /// Checks wether a point is in the envelope
    pub fn contains(&self, x: X, y: Y) -> bool {
        let minimum = self.points[0];
        if x < minimum.0 || y < minimum.1 {
            return false;
        }

        let mut interval_index = (N - 1) - 1; // TODO make this `derivatives.len() - 1`
        for (i, p) in self.points[0..self.derivatives.len() - 1]
            .iter()
            .enumerate()
        {
            // TODO remove the -1
            let p_ = self.points[i + 1];
            if p.0 <= x && x <= p_.0 {
                interval_index = i;
                break;
            }
        }

        let fx = self.points[interval_index].1
            + self.derivatives[interval_index] * (x - self.points[interval_index].0);
        y <= fx
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_envelope() -> Envelope<4> {
        let points = [(1600, 100), (1850, 300), (10100, 1958), (10101, 1958)];
        Envelope::new(points).unwrap()
    }

    #[test]
    fn left_lower_bound() {
        let evp = init_envelope();
        assert!(evp.contains(1600, 100));
    }

    #[test]
    fn y_below_envelope() {
        let evp = init_envelope();
        assert!(!evp.contains(1600, 99.9));
    }

    #[test]
    fn high_x_in_envelope() {
        let evp = init_envelope();
        assert!(evp.contains(1e100, 100));
    }

    #[test]
    fn y_above_envelope() {
        let evp = init_envelope();
        assert!(!evp.contains(10100, 1959));
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn input_list_too_small() {
        let evp = Envelope::<1>::new([(0, 0)]).unwrap();
    }
}
