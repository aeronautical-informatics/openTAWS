pub struct Envelope {
    points: Vec<(f64, f64)>,
    derivatives: Vec<f64>,
}

impl Envelope {
    /// Creates an envelope from point pairs
    ///
    /// The point pairs must be orderd by x values in ascending order. There must be no two points
    /// with the same x value. The lower left bound is is the first point.
    ///
    /// Each section between two successive x points is interpolated by a linear function. The
    /// interpolation function of the last section is extendend to +∞. Use an additional point with
    /// the same y value as the prior point to cause an extrapolation parallel to the x axis for
    /// the slope after the last point.
    ///
    /// # Example
    ///
    /// ```
    /// use otaws::envelope::Envelope;
    /// // The last point ensure that the slope is extended with a function parallel to the x axis
    /// let points = vec![(1908, 150), (2050, 300), (10300, 1958), (10301, 1958)];
    /// let envelope = Envelope::new(&points).expect("invalid points given to envelope");
    /// ```
    pub fn new<'a, I, T: 'a>(points: I) -> Option<Self>
    where
        I: IntoIterator<Item = &'a (T, T)>,
        T: Into<f64> + Copy,
    {
        let points: Vec<(f64, f64)> = points
            .into_iter()
            .map(|e| (e.0.into(), e.1.into()))
            .collect();

        if points.len() < 2 {
            // List is too small!
            return None;
        }

        for i in 1..points.len() {
            if points[i - 1].0 >= points[i].0 {
                // This means either the list is not sorted or two values are identical
                return None;
            }
        }

        let derivatives = (0..(points.len() - 1))
            .map(|i| {
                let (x, y) = points[i];
                let (x_, y_) = points[i + 1];
                (y_ - y) / (x_ - x)
            })
            .collect();

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
        let minium = self.points[0];
        if x < minium.0 || y < minium.1 {
            return false;
        }

        //let position = self.points.iter().position(|p| )
        let mut interval_index = self.derivatives.len() - 1;
        for (i, p) in self.points.iter().enumerate().take(self.derivatives.len()) {
            let p_ = self.points[i + 1];
            if p.0 <= x && x <= p_.0 {
                interval_index = i;
                break;
            }
        }

        let fx = self.points[interval_index].1
            + self.derivatives[interval_index] * (x - self.points[interval_index].0);
        println!("fx = {}", fx);
        y <= fx

        /*
        let mut p_iter = self.points.iter().zip(self.derivatives.iter().chain(iter::once(0.0))).peekable();

        while let ( (p, d), (p_,_)) = (p_iter.next(), p_iter.peek(){
            if p.0 <= x && x <= p_.0{
                return p < p.y +
            }
        }

        for p in self.points{
            if x <= p.0 && p.0 <=
        }
        if input.0< self.minimum.0 || input.1 <self.minimum.1{
            return false;
        }
        */
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_envelope() -> Envelope {
        let vec_points = vec![(1600, 100), (1850, 300), (10100, 1958), (10101, 1958)];
        Envelope::new(&vec_points).unwrap()
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
