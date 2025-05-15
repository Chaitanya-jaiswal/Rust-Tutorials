use std::io;

pub fn var_ass_mut(){
    let x : i32 = 10;
    let y = 11;
    println!("Values of x and y: {} and {}", x, y);

    let x = "a string?";
    println!("Value of x: {}", x);

    let mut z = 10;
    z = z+1;
    println!("Value of z: {}",z);

    const _TRUE : i32 = 1;
}
const _FALSE : i32 = 0;

pub fn vals_types(){
    let x : i32 = 10;
    let y : i64 = 20;
    println!("Value of +: {}", x+(y as i32));
    let z : f32 = 1.2;
    let u : f64 = 3.45;
    println!("Value of +: {}", (z as f64)+u);

    let t : bool = true;
    let f : bool = false;
    if t == f {
        println!("True is false");
    } else {
        println!("True is not false");
    }

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Some chars: {}, {}, and {}", c, z, heart_eyed_cat);

    let tuple : (i32,f64,char) = (1, 2.5, 'w');
    let (el1, _el2, _el3) = tuple;
    let (el01, _, _ ) = tuple;
    let first = tuple.0;
    let _last = tuple.2;
    println!("First element: {} = {} = {}", el1, first, el01);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
    let a = [3, 3, 3, 3, 3];
    let a1 = a[0];
    let a2 = a[1];
    println!("Array Elements: {} and {}", a1, a2);

    print!("Input element index to lookup:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(mut i) => {
            println!("Integer input: {}", i);
            if i>5 {
                i = 5;
            }
            let _element = a[(i as usize)];
            println!("This will not print without the if");
        }
        ,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

pub fn expressions(){
    let mut c = 0;
    loop {
        println!("This will never stop");
        c += 1;
        if c == 4 {
            break;
        }
    }
    while c != 0 {
        println!("Cycle with while {}!", c);
        c -= 1;
    }

    for n in 1..51 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Iteration loop: the value is: {}", element);
    }
}

//explain
#[cfg(test)]
mod testing {
    use crate::c01_basic::testfuns::{crapadd, okadd};

    #[test]
    fn test_crapadd() {
        assert_eq!(crapadd(1,2),2); 
    }
    #[test]
    fn test_okadd(){
        assert_eq!(okadd(1, 5), 6);
    }
}

pub mod testfuns{
    pub fn crapadd(x: i32,_y: i32) -> i32 {
        return x+x;
    }
    pub fn okadd(x: i32, y:i32) -> i32 {
        x+y
    }
}
