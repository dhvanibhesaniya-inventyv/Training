struct Area{
    length:f32,
    breath:f32,
    radius:f32
}

impl Area{
    fn area_circle(&self){
        println!("Area of circle is : {} ", 3.14 * self.radius * self.radius);
    }
    fn area_square(len:f32){
        println!("Area of square is : {} ", len * len);

    }
    fn area_rectangle(&self)-> f32{
        // println!("Area of rectangle is : {} ", self.length * self.breath);
        self.length * self.breath
    }
}


fn main(){


    let values = Area{
        length : 5.0 ,
        breath : 4.0 ,
        radius : 7.0

    };

values.area_circle();
Area::area_square(values.length);
println!("Area of rectangle is :{} ",values.area_rectangle());


}