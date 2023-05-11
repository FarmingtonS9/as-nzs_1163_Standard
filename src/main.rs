fn main() {
    println!("Hello, world!");
    println!("We are going to build a program that determines if the data about steel falls within spec of the AS/NZS 1163 standard");
    println!("The name of AS/NZS 1163 is Cold-formed structural steel hollow section");
    println!("Specific version is AS/NZS 1163:2009");

    let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
}

//Add struct for reference SHS, similar to SHS struct
//Easier for end user to build an explicitly separate SHS product
//Future idea: Add reference SHS to SHS product struct so it can be internally accessed (if None, warn user)

#[derive(Debug)]
struct SHS {
    width: f32,
    height: f32,
    gauge: f32,
    length: f32,
    //shs_reference: SHSRef
}

impl SHS {
    fn new(width: f32) -> SHSBuilder {
        SHSBuilder {
            width,
            height: width,
            gauge: 0.,
            length: 0.,
        }
    }

    //TODO: Rethink logic
    fn check_external_dimensions(&self, reference_width: &f32, reference_height: &f32) -> bool {
        let allowed_tolerance_width = 0.01 * reference_width;
        let allowed_tolerance_height = 0.01 * reference_height;

        println!(
            "Allowed tolerance for width: {}mm and height: {}mm",
            allowed_tolerance_width, allowed_tolerance_height
        );

        let is_width_tolerance_ok = self.check_width_tolerance(*reference_width);
        let is_height_tolerance_ok = self.check_height_tolerance(*reference_height);

        dbg!(&is_width_tolerance_ok);
        dbg!(&is_height_tolerance_ok);

        if is_width_tolerance_ok == true && is_height_tolerance_ok == true {
            true
        } else {
            false
        }
    }

    fn check_width_tolerance(&self, reference_width: f32) -> bool {
        let min_tolerance = 0.5;
        let allowed_tolerance_width = 0.01 * reference_width;
        let width_difference = self.width - reference_width;

        //Checking for tolerances of width
        if -min_tolerance < width_difference && min_tolerance > width_difference {
            println!(
                "Failed width within {}mm - {}mm of tolerance. Calculated result {}",
                -min_tolerance, min_tolerance, width_difference
            );
            false
        } else {
            println!("Minimum width tolerance: Passed!");
            if -allowed_tolerance_width <= width_difference
                && allowed_tolerance_width >= width_difference
            {
                println!("Required width tolerance: Passed!");
                true
            } else {
                println!("Required width tolerance: Failed!");
                false
            }
        }
    }

    fn check_height_tolerance(&self, reference_height: f32) -> bool {
        let min_tolerance = 0.5;
        let allowed_tolerance_height = 0.01 * reference_height;
        let height_difference = self.height - reference_height;

        //Check tolerance for height
        if -min_tolerance < height_difference && min_tolerance > height_difference {
            println!(
                "Failed height within {}mm - {}mm of tolerance. Calculated result {}",
                -min_tolerance,
                min_tolerance,
                self.height - reference_height
            );
            false
        } else {
            println!("Minimum tolerance: Passed!");
            if -allowed_tolerance_height <= self.height - reference_height
                && allowed_tolerance_height >= self.height - reference_height
            {
                println!("Required height tolerance: Passed!");
                true
            } else {
                println!("Required height tolerance: Failed!");
                false
            }
        }
    }

    fn check_thickness(&self, reference_gauge: &f32) -> bool {
        let allowed_tolerance_gauge = 0.01 * reference_gauge;
        let gauge_difference = self.gauge - reference_gauge;

        println!(
            "Allowed tolerance is {}mm - {}mm",
            -allowed_tolerance_gauge, allowed_tolerance_gauge
        );

        if -allowed_tolerance_gauge < gauge_difference && allowed_tolerance_gauge > gauge_difference
        {
            println!("Required tolerance: Passed!");
            true
        } else {
            println!("Required tolerance: Failed");
            false
        }
    }

    //TODO: Fix checks for concavity and convexity.
    //Meant to be checking for percentage as well.
    fn check_concavity_or_convexity(&self, deviation: f32, reference_width: &f32) -> bool {
        if deviation < 0. {
            println!("Concavity");
            self.check_concavity(deviation, reference_width)

        //0 will evaluate to "convexity"
        //Rethink control logic, if we want to
        } else {
            println!("Convexity");
            self.check_convexity(deviation, reference_width)
        }
    }

    fn check_concavity(&self, concavity: f32, reference_width: &f32) -> bool {
        let max_tolerance = 0.5;
        let calculated_tolerance = 0.008 * reference_width;

        if calculated_tolerance > max_tolerance {
            if calculated_tolerance > concavity {
                println!("Within maximum tolerance!");
                true
            } else {
                println!("Failured convexity tolerance");
                false
            }
        } else {
            if max_tolerance > concavity {
                println!("Within maximum tolerance!");
                true
            } else {
                println!("Failured convexity tolerance");
                false
            }
        }
    }

    fn check_convexity(&self, convexity: f32, reference_width: &f32) -> bool {
        let max_tolerance = 0.5;
        let calculated_tolerance = 0.008 * reference_width;

        if calculated_tolerance > max_tolerance {
            if calculated_tolerance > convexity {
                println!("Within maximum tolerance!");
                true
            } else {
                println!("Failured convexity tolerance");
                false
            }
        } else {
            if max_tolerance > convexity {
                println!("Within maximum tolerance!");
                true
            } else {
                println!("Failured convexity tolerance");
                false
            }
        }
    }

    ///Squaredness refers to how perpendicular the intersecting sides are.
    /// 90 degrees is perfectly square, and the Standard dictates squaredness to be not greater than 1 or -1 of 90.
    fn check_squaredness_of_sides(&self, angle: f32) -> bool {
        if angle < 89. || angle > 91. {
            false
        } else {
            true
        }
    }

    //Don't like this code block.
    //Look to improve it
    fn check_external_corner_profile(
        &self,
        radius_gauge: f32,
        corner_1: f32,
        corner_2: f32,
    ) -> bool {
        //Checking for external dimensions
        //50x50 or less
        if self.width <= 50. && self.height <= 50. {
            if corner_1 >= 1.5 * self.gauge
                && corner_1 <= 3.0 * self.gauge
                && corner_2 >= 1.5 * self.gauge
                && corner_2 <= 3.0 * self.gauge
            {
                true
            } else {
                false
            }
        //Greater than 50x50
        } else {
            if corner_1 >= 1.8 * self.gauge
                && corner_1 <= 3.0 * self.gauge
                && corner_2 >= 1.8 * self.gauge
                && corner_2 <= 3.0 * self.gauge
            {
                true
            } else {
                false
            }
        }
    }

    fn check_twist(&self, measured_twist: f32) -> bool {
        let max_twist_tolerance = 2. + (0.5 * (self.length / 1000.));

        if measured_twist <= max_twist_tolerance {
            true
        } else {
            false
        }
    }

    fn check_straightness(&self, deviation: f32) -> bool {
        let max_deviation = (self.length / 1000.) * 0.15;
        if deviation <= max_deviation {
            true
        } else {
            false
        }
    }

    fn check_mass(&self, reference_weight: f32, measured_weight: f32) -> bool {
        let weight_ratio = measured_weight / reference_weight;
        if weight_ratio > 0.96 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct SHSBuilder {
    width: f32,
    height: f32,
    gauge: f32,
    length: f32,
}

impl SHSBuilder {
    fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }
    fn length(&mut self, length: f32) -> &mut Self {
        self.length = length;
        self
    }

    fn gauge(&mut self, gauge: f32) -> &mut Self {
        self.gauge = gauge;
        self
    }

    fn build(&mut self) -> SHS {
        SHS {
            width: self.width,
            height: self.height,
            gauge: self.gauge,
            length: self.length,
        }
    }
}

#[cfg(test)]
mod shape_and_mass_test {
    use super::*;

    #[test]
    fn failed_within_width_tolerance() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.).height(100.).length(8000.).gauge(5.).build();

        assert_eq!(
            shs_product_1.check_external_dimensions(&reference_shs.width, &reference_shs.height),
            false
        );
    }

    #[test]
    fn passed_test_for_tolerance() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.5).height(99.5).length(8000.).gauge(5.).build();

        assert_eq!(
            shs_product_1.check_external_dimensions(&reference_shs.width, &reference_shs.height),
            true
        )
    }

    #[test]
    fn failed_width_tolerance() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(98.9).height(100.5).length(8000.).gauge(5.).build();

        assert_eq!(
            shs_product_1.check_external_dimensions(&reference_shs.width, &reference_shs.height),
            false
        );
    }

    #[test]
    fn failed_height_tolerance() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.).height(101.1).length(8000.).gauge(5.).build();

        assert_eq!(
            shs_product_1.check_external_dimensions(&reference_shs.width, &reference_shs.height),
            false
        );
    }

    #[test]
    fn passed_thickness_tolerance() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.).length(8000.).gauge(5.04).build();

        assert_eq!(shs_product_1.check_thickness(&reference_shs.gauge), true)
    }
    #[test]
    fn failed_thickness_tolerance() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.).length(8000.).gauge(4.8).build();

        assert_eq!(shs_product_1.check_thickness(&reference_shs.gauge), false)
    }

    #[test]
    fn pass_concavity() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.4)
            .height(99.8)
            .length(8000.)
            .gauge(4.8)
            .build();

        assert_eq!(
            shs_product_1.check_concavity(0.5, &reference_shs.width),
            true
        );
    }

    #[test]
    fn fail_concavity() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.9)
            .height(99.8)
            .length(8000.)
            .gauge(4.8)
            .build();

        assert_eq!(
            shs_product_1.check_concavity(0.9, &reference_shs.width),
            false
        );
    }

    #[test]
    fn pass_convexity() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.4)
            .height(99.8)
            .length(8000.)
            .gauge(4.8)
            .build();

        assert_eq!(
            shs_product_1.check_convexity(0.2, &reference_shs.width),
            true
        );
    }

    #[test]
    fn fail_convexity() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.4)
            .height(99.8)
            .length(8000.)
            .gauge(4.8)
            .build();

        assert_eq!(
            shs_product_1.check_concavity(-0.5, &reference_shs.width),
            true
        );
    }

    #[test]
    fn test_concavity() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.4)
            .height(99.8)
            .length(8000.)
            .gauge(4.8)
            .build();

        assert_eq!(
            shs_product_1.check_concavity_or_convexity(-0.5, &reference_shs.width),
            true
        );
    }

    #[test]
    fn test_convexity() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        let shs_product_1 = SHS::new(100.4)
            .height(99.8)
            .length(8000.)
            .gauge(4.8)
            .build();

        assert_eq!(
            shs_product_1.check_concavity_or_convexity(0.5, &reference_shs.width),
            true
        );
    }

    #[test]
    fn fail_squaredness() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_squaredness_of_sides(88.9), false)
    }

    #[test]
    fn pass_squaredness() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_squaredness_of_sides(90.1), true)
    }

    #[test]
    fn pass_twist() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_twist(0.9), true)
    }

    #[test]
    fn fail_twist() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_twist(6.5), false)
    }

    #[test]
    fn pass_straightness() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_straightness(0.7), true);
    }

    #[test]
    fn fail_straightness() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_straightness(1.3), false)
    }

    #[test]
    fn pass_weight() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_mass(113.92, 112.5), true)
    }

    #[test]
    fn fail_weight() {
        let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
        assert_eq!(reference_shs.check_mass(113.92, 108.4), false)
    }
}
