fn main() {
    println!("Hello, world!");
    println!("We are going to build a program that determines if the data about steel falls within spec of the AS/NZS 1163 standard");
    println!("The name of AS/NZS 1163 is Cold-formed structural steel hollow section");
    println!("Specific version is AS/NZS 1163:2009");

    let reference_shs = SHS::new(100.).length(8000.).gauge(5.).build();
    let shs_product = SHS::new(100.3)
        .height(98.8)
        .length(8110.2)
        .gauge(4.82)
        .build();

    dbg!(&reference_shs);
    dbg!(&shs_product);
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

    fn check_external_dimensions(&self) {}
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
