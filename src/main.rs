fn main() {
    println!("Hello, world!");
    println!("We are going to build a program that determines if the data about steel falls within spec of the AS/NZS 1163 standard");
    println!("The name of AS/NZS 1163 is Cold-formed structural steel hollow section");
    println!("Specific version is AS/NZS 1163:2009");

    let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
}

#[derive(Debug)]
struct SHS {
    width: f32,
    height: f32,
    gauge: f32,
    length: f32,
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

    fn check_thickness(&self, reference_gauge: f32) -> bool {
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
}
