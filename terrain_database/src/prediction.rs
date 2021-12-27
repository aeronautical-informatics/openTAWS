use num_traits::Zero;
use uom::si::acceleration::meter_per_second_squared;
use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::si::time::millisecond;
use uom::typenum::P2;

const REACTION_TIME_MS: f64 = 1000.0;
const MAX_G: f64 = 1.0;
const TARGET_CLIMB_RATE_DEGREE: f64 = 6.0;

/// Calculates a position based on a starting point, direction, speed and travel time
fn linear_calculation(
    start: &[Length; 2],
    dir: &[Ratio; 2],
    speed: Velocity,
    t: Time,
) -> [Length; 2] {
    let mut out = *start;

    out[0] += dir[0] * speed * t;
    out[1] += dir[1] * speed * t;
    out
}

pub struct HeightPrediction {
    start_height: Length,
    start_dir: [Ratio; 2],

    middle_pos: [Length; 2],
    middle_dir: [Ratio; 2],
    middle_time: Time,

    last_pos: [Length; 2],
    last_dir: [Ratio; 2],
    last_time: Time,

    speed: Velocity,
}

impl HeightPrediction {
    /// Returns the predicted height for the position reached after a specified travel time
    pub fn height_by_time(&self, time: Time) -> Length {
        if time <= self.middle_time {
            self.start_height + self.start_dir[1] * self.speed * time
        } else if time <= self.last_time {
            self.middle_pos[1] + self.middle_dir[1] * self.speed * time
        } else {
            self.last_pos[1] + self.last_dir[1] * self.speed * time
        }
    }

    /// Returns the predicted vertical distance cleared after a specific travel time
    pub fn distance_by_time(&self, time: Time) -> Length {
        if time <= self.middle_time {
            self.start_dir[0] * self.speed * time
        } else if time <= self.last_time {
            self.middle_pos[0] + self.middle_dir[0] * self.speed * time
        } else {
            self.last_pos[0] + self.last_dir[0] * self.speed * time
        }
    }

    /// Returns the predicted height after a specific vertical distance
    pub fn height_by_distance(&self, distance: Length) -> Length {
        if distance <= self.middle_pos[0] {
            let z = distance / self.start_dir[0];
            self.start_height + self.start_dir[1] * z
        } else if distance <= self.last_pos[0] {
            self.middle_pos[1]
        } else {
            let z = (distance - self.last_pos[0]) / self.last_dir[0];
            self.last_pos[1] + self.last_dir[1] * z
        }
    }
}

/// Generates a path for an airplane, based on a starting height, pitch and velocity.
/// Results in 3 linear functions modelling the predicted height based on distance from the starting point or time from the starting point
pub fn gen_path(start_height: Length, pitch: Angle, speed: Velocity) -> HeightPrediction {
    //                      , - ~ ~ ~ - ,
    //       |\         , '               ' ,
    //       | \      ,                       ,
    //       |  \    ,                         ,    /|
    //       |   \  ,                           ,  / |
    //       |    \ ,                           , /  |
    //       |     \,                           ,/   |
    //       |      \,                         ,/    |
    //       |       \,                       ,/     |
    //       |        \ ,                  , '/      |
    //       |         \__'_-_,_______,__'___/       |
    //       |         |                     |       |
    //       |  part 1 |       part 2        | part 3|
    //      Pilot Delay    Start climbing     climbing flight

    // In the first part we just follow the current flight direction until we hit "the bottom" of the predicted climb circle
    let pos1: [Length; 2] = [Length::zero(), start_height];
    let dir1: [Ratio; 2] = [pitch.cos(), pitch.sin()];
    let reaction_time = Time::new::<millisecond>(REACTION_TIME_MS);
    let max_g = Acceleration::new::<meter_per_second_squared>(MAX_G * 9.80665);

    // Here we determine the radius of the circle we use for transitioning towards our target climb rate
    // It is calculated by our current flight speed squared and divided by our maximum g-force
    let climb_radius: Length = speed.powi(P2::new()) / max_g;
    let incoming_pitch: Angle = pitch;
    let outgoing_pitch: Angle = Angle::new::<degree>(TARGET_CLIMB_RATE_DEGREE);

    // Some variables in the coordinate system of the circle
    // (l1/h1) is the (x,y)-coordinate of the intersection of part 1 and the circle
    let l1: Length = climb_radius * incoming_pitch.sin() + climb_radius;
    let h1: Length = climb_radius * -incoming_pitch.cos() + climb_radius;
    // (l2/h2) is the (x,y)-coordinate of the intersection of part 3 and the circle
    let l2: Length = climb_radius * outgoing_pitch.sin() + climb_radius;
    let h2: Length = climb_radius * -outgoing_pitch.cos() + climb_radius;
    let dist_l1_l2: Length = l2 - l1;
    //                      , - ~ ~ ~ - ,        * (radius*2,radius*2)
    //        \         , '               ' ,      top right corner of
    //         \      ,                       ,    the circle coordinate system
    //          \    ,                         ,    /
    //           \  ,                           ,  /
    //            \ ,     dist_l1_l2            , /
    //     (l1,h1) *,---------------------------,* (l2,h2)
    // intersection \,                         ,/ intersection of
    // of part 1 and \,                       ,/  part 3 and the circle
    // the circle     \ ,                  , '/
    //       (0,0) *   \__'_-_,_______,__'___/
    //     origin of
    //     the circle
    //     coordinate system

    // Pos2 is the start position of the second part of the path
    let mut pos2 = linear_calculation(&pos1, &dir1, speed, reaction_time);
    // MaxT1 is the maximum time for the first function describing the first part
    let mut max_t1: Time = reaction_time;
    // We only need to account for additional Distance(for Pos2) and Time(for MaxT1) if the pitch is negative.
    // This would mean that we need to decent to "the bottom" of the circle
    if incoming_pitch < Angle::zero() {
        //How many times do we have to apply Dir1 to clear the height of the intersection of part 1 with the circle
        let mul = h1 / dir1[1].abs();
        // Apply mul with dir1 to pos2 for getting real start point of pos2
        pos2[0] += dir1[0] * mul;
        // No need to multiply, just subtract the height
        pos2[1] -= h1;
        // Likewise apply mul to speed for getting the additional time required to clear the distance
        max_t1 += mul / speed;
    }
    // Dir2 is always (1, 0)

    // Dir 3 is the direction of part 3 of the path.
    // For this we just generate ratios matching our outgoing pitch
    let dir3: [Ratio; 2] = [outgoing_pitch.cos(), outgoing_pitch.sin()];

    // Pos3 is the start position of part 3 of the path
    // Start with the position of part 1 where it intersects with the climb circle
    let mut pos3: [Length; 2] = linear_calculation(&pos1, &dir1, speed, reaction_time);
    // If l1 is equal or greater than l2, then we are already at the right position
    // This would also mean that we got no part 2!
    if l1 < l2 {
        // Calculate how many times we need to apply dir1 to pos3 for getting to the intersection of part 3 with the circle (by x-value)
        let mul: Length = dist_l1_l2 / dir1[0];
        // Apply this calculation to pos3
        pos3[0] += dir1[0] * mul;
        pos3[1] += h2 - h1;
        // For getting the real Pos3 we need to move Pos3 on its current linear function until its on the same height as Pos2
        // So we calculate how many times we need to apply the y-part of Dir3 for clearing the distance between the
        // y-part of Pos3 and the y-part of Pos2
        let mul: Length = (pos3[1] - pos2[1]) / dir3[1];
        //Finally we apply this to Pos3 for getting the real position of Pos3
        pos3[0] -= dir3[0] * mul;
        pos3[1] = pos2[1];
    }

    // MaxT2 is the maximum time for the second function describing the second part
    // For this we just check how long it takes to clear the distance between pos2 and pos3
    let max_t2: Time = (pos3[0] - pos2[0]) / speed;
    let max_t2 = max_t2.max(Time::zero());
    let max_t2 = max_t2 + max_t1;

    HeightPrediction {
        start_height,
        start_dir: dir1,
        middle_pos: pos2,
        middle_dir: [Ratio::new::<ratio>(1.0), Ratio::new::<ratio>(0.0)],
        middle_time: max_t1,
        last_pos: pos3,
        last_dir: dir3,
        last_time: max_t2,
        speed,
    }
}
