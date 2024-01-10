struct Area {
    length: f32,
    breath: f32,
}

impl Area {
    fn area_rectangle(&self) {
        println!("Area of rectangle is : {}", self.length * self.breath);
    }

    fn area_square(&self) {
        let side = if self.length >= self.breath {
            self.breath
        } else {
            self.length
        };

        println!("Area of square is : {}", side * side);
    }

    fn area_circle(&self) {
        let radius = self.length.min(self.breath) / 2.0;

        println!("Area of circle is : {}", 3.14 * radius * radius);
    }
}

fn main() {
    let values = Area {
        length: 5.0,
        breath: 5.0,
    };

    values.area_rectangle();
    values.area_square();
    values.area_circle();
}
