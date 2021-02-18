/// `Envelope` helps checking whether a 2D point is inside of a 2D Envelope - that is a polygon without vertical lines.
pub struct Envelope<const N: usize> {
    points: [(f64, f64); N],
    derivatives: [f64; N], // TODO make this N-1
}

impl<const N: usize> Envelope<N> {
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
    pub fn new<T, U>(points: [(T, U); N]) -> Option<Self>
    where
        T: Into<f64> + Copy,
        U: Into<f64> + Copy,
    {
        if N < 2 {
            // List is too small!
            return None;
        }
        let points_raw = points;

        let mut points: [(f64, f64); N] = [(points_raw[0].0.into(), points_raw[0].1.into()); N];
        for i in 1..N {
            points[i] = (points_raw[i].0.into(), points_raw[i].1.into());
            if points[i - 1].0 >= points[i].0 {
                // This means either the list is not sorted or two values are identical
                return None;
            }
        }

        // TODO port this to iter code once libcore allows to collect into arrays
        /*
        let derivatives = (0..(N - 1))
            .map(|i| {
                let (x, y) = points[i];
                let (x_, y_) = points[i + 1];
                (y_ - y) / (x_ - x)
            })
            .collect();
         */

        let mut derivatives = [0f64; N];
        for i in 0..(N - 1) {
            let (x, y) = points[i];
            let (x_, y_) = points[i + 1];
            derivatives[i] = (y_ - y) / (x_ - x)
        }

        Some(Self {
            points,
            derivatives,
        })
    }

    /// Checks wether a point is in the envelope
    pub fn contains<T, U>(&self, x: T, y: U) -> bool
    where
        T: Into<f64> + Copy,
        U: Into<f64> + Copy,
    {
        let (x, y) = (x.into(), y.into());
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
        let vec_points = [(1600, 100), (1850, 300), (10100, 1958), (10101, 1958)];
        Envelope::new(vec_points).unwrap()
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
}
