pub fn main()
{
    draw_shape(1);
}

fn draw_shape(position: usize)
{
    enum ShapesEnum
    {
        Square(fn(i32) -> i32),
        Triangle(fn(u32) -> u32)
    }

    let shapes = [ShapesEnum::Square(square), ShapesEnum::Triangle(triangle)];

    match shapes[position]
    {
        ShapesEnum::Square(f)           => println!("sqr: {}", f(10)),
        ShapesEnum::Triangle(f)           => println!("trig: {}", f(10)),
    }
    
    //let shapes = [square, triangle];

    fn square(x: i32) -> i32
    {
        10i32
    }
    fn triangle(x: u32) -> u32
    {
        20u32
    }

}