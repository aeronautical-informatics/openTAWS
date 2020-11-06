pub struct Envelope {
    points: Vec<(f64, f64)>,
    derivatives: Vec<f64>,
}

impl Envelope {
    /// points is orderd by x values
    /// points does not contain two values with the same x
    pub fn new(points: Vec<(f64, f64)>) -> Option<Self> {
        if points.len() < 2 {
            return None;
        }

        //points.sort_by(|p1, p2| p1.0.partial_cmp(&p2.0).unwrap());

        let minimum = points[0];
        let mut derivatives = (0..(points.len() - 1))
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

    pub fn contains(&self, x: f64, y: f64) -> bool {
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
        let vec_points = vec![
            (1600.0f64, 100.0),
            (1850.0, 300.0),
            (10100.0, 1958.0),
            (10101.0, 1958.0),
        ];
        Envelope::new(vec_points).unwrap()
    }

    #[test]
    fn left_lower_bound() {
        let evp = init_envelope();
        assert!(evp.contains(1600.0, 100.0));
    }

    #[test]
    fn y_below_envelope() {
        let evp = init_envelope();
        assert!(!evp.contains(1600.0, 99.9));
    }

    #[test]
    fn high_x_in_envelope() {
        let evp = init_envelope();
        assert!(evp.contains(1e100, 100.0));
    }

    #[test]
    fn y_above_envelope() {
        let evp = init_envelope();
        assert!(!evp.contains(10100.0, 1959.0));
    }
}
