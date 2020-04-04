#[allow(dead_code)]
fn main() {
    let number = 42;
    println!("Tell me about {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime"), 
        13..=19 => println!("A teen"), // 13 - 19 inclusive
        y if y == 42 => println!("You've picked the meaning to life, the universe, and everything: {}", y), // break the integer our for giggles.
        _ => println!("anything else"), 
    }

    
    // match guards
    let pair = (2, 4);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("Twins!"),
        (x, y) if x+y == 0 => println!("Opposites"),
        (x, _) if x % 2 == 1 => println!("x is odd"),
        _ => println!("default"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    }; 

    println!("{} -> {}", boolean, binary);

    // match can be used to deconstruct tuples and enums and such...
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is 0 and y is {:?}", y),
        (x, 0) => println!("X is {:?} and last is 0", x),
        _      => println!("Something else"),
    }

    #[allow(dead_code)]
    enum Color {
        // unit types
        Red,
        Green,
        Blue,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(127, 12, 22);

    println!("What Color is it?");
    match color {
        Color::Red => println!("RED"),
        Color::Green => println!("GREEN"),
        Color::Blue => println!("BLUE"), 
        Color::RGB(r, g, b) => println!("rgb: {},{},{}", r, g, b),
        Color::HSV(h, s, v) => println!("hsv: {},{},{}", h, s, v),
        Color::HSL(h, s, l) => println!("hsl: {},{},{}", h, s, l),
        Color::CMY(c, m, y) => println!("cmy: {},{},{}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("cmyk: {},{},{},{}", c, m, y, k),
    }

    // assign a reference type of i32,
    // the & signifies there si a reference being assigned.
    let refer = &42; 

    // if refer is pattern matched against &val
    // it results in an comparison like:
    // &i32
    // &val
    // we see that if matching, &'s are dropped then the i32
    // should be assigned to val
    match refer { 
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // to avoid &, dereference before matching with *
    match *refer {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // what about if you don't start with a reference?
    // refer was a &
    // this is not
    let _not_refer = 21;
    // this is a forced reference assignement with `ref`
    let ref _is_refer = 42;

    let value = 5;
    let mut mut_val = 10;

    match value {
        ref r => println!("Got a reference to a value: {}", r),
    }

    match mut_val {
        ref mut m => {
            // got a reference, must deref it first 
            *m += 10;
            println!("We added 10, mut_val = {}", m);
        }, 
    }

    // structs
    
    struct Foo {
        x: (i32, i32),
        y: i32,
    }

    let foo = Foo { x: (1,2), y: 3 };

    match foo {
        Foo { x: (1,b), y } => println!("First of X is 1, b = {}, y = {}", b, y),
        Foo { y, .. } => println!("y = {}, x is anything", y),
    }

    struct Point {
        x: i32,
        y: i32, 
    }

    let origin = Point { x: 0, y: 0 };
    let dest = Point { x: 21, y: 42 };
    // use `:` to shadow the struct's vars
    match origin {
        Point { x, y } => println!("Oigin: {}, {}", x, y),
    }
    match dest {
        Point {x: x_d, y: y_d } => println!("Destination: {}, {}", x_d, y_d),
    }


    let taco: Result<i32, &'static str> = Err("I'm NOT a taco!");
    let chimichanga: Result<&'static str, &'static str> = Ok("I'm a chimichanga!");
    
    match taco {
        Ok(value) => println!("Ok: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match chimichanga {
        Ok(value) => println!("Ok, {}", value),
        Err(e) => println!("Error: {}", e), 
    }

    let t =  Some(3);
    let b = match t {
        Some(i) => Some(i+3),
        None => None,
    };

 
    let _seasons = ["Spring", "Summer", "Fall", "Winter"];
    let _weathers = ["Sunny", "Breezy", "Cloudy", "Spiders", "Rainy", "Snowy", "Stormy"];
    let _temps = ["Cold", "Cool", "Warm", "Hot"];
    enum Season {
        Spring = 0,
        Summer = 1,
        Fall = 2, 
        Winter = 3, 
    }

    let spring_time = Season::Spring as i32;

    match spring_time  { 
        0 => println!("spring"),
        1 => println!("summer"),
        2 => println!("fall"),
        3 => println!("winter", ),
        _ => {},
    }
    #[allow(dead_code)]
    enum Weather {
        Drought = 0,
        Sunny = 1,
        Breezy = 2,
        Cloudy = 3,
        Rainy = 4, 
        Snowy = 5,
        Stormy = 6,
    }
    #[allow(dead_code)]
    enum Temp {
        Cold = 0,
        Cool = 1,
        Warm = 2, 
        Hot = 3, 
    }

    let season = Season::Spring; 
    let weather = Weather::Breezy;
    let temp = Temp::Cool;

    match season {
        Season::Spring | Season::Fall => match temp {
            Temp::Cool => match weather {
                Weather::Drought => {},
                Weather::Sunny => {},
                Weather::Breezy => println!("People aren't buying much lemonade today :-("),
                Weather::Cloudy => {},
                Weather::Rainy => {},
                Weather::Snowy => {},
                Weather::Stormy => {},
            }
            Temp::Warm => {},
            _ => {},
        }
        Season::Summer => match temp {
            Temp::Warm => {},
            Temp::Hot => {},
            _ => {},
        }
        Season::Winter => match temp {
            Temp::Cool => {},
            Temp::Cold => {},
            _ => {},
        }
    }
}