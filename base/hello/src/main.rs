// use std::process::Command;

// fn main() {
//     // let output = Command::new("/data/rust/xc/hello").output().expect("ERROR");

//     let output = Command::new("python3")
//           .arg("/data/rust/hello/src/hello")
//           .output()
//           .unwrap();
//     let out = String::from_utf8(output.stdout).unwrap();
//     println!("{}", out);
    

// }

// macro_rules! say_hello {
//     () => (
//         println!("Hello!")
//     )
// }

// fn main() {
//     say_hello!()

// }

// macro_rules! create_function {
//     ($func_name: ident) => (
//         fn $func_name() {
//             println!("You called {:?}()", stringify!($func_name))
//         }
//     )   
// }

// create_function!(foo);
// create_function!(bar);

// macro_rules! print_result {
//     ($expression:expr) => (
//         println!("{:?} = {:?}", stringify!($expression), $expression)
//     )
// }

// fn main() {
//     foo();
//     bar();

//     print_result!(1u32 + 1);
//     print_result!({
//         let x = 1u32;
//         x * x + 2 * x - 1
//     });
// }
// macro_rules! test {
//     ($left:expr; and $right:expr) => {
//         println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right)
//     };
//     ($left:expr; or $right:expr) => (
//         println!("{:?} or {:?} is {:?}",
//         stringify!($left),
//         stringify!($right),
//         $left || $right)
//     );
// }

// fn main() {
//     test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
//     test!(true; or false); 
// }

// // `min!` 将求出任意数量的参数的最小值。
// macro_rules! find_min {
//     ($x:expr) =>($x);
//     ($x:expr, $($y:expr), +) => (
//         std::cmp::max($x, find_min!($($y), +))
//     ) 
// }

// fn main() {
//     println!("{}", find_min!(1u32));
//     println!("{}", find_min!(1u32 + 2, 2u32));
//     println!("{}", find_min!(5u32, 2u32 * 3, 4u32, 52u32, 2u32));
// }


// use std::ops::{Add, Mul, Sub};

// macro_rules! assert_equal_len {
//     ($a: ident, $b: ident, $func: ident, $op: tt) => (
//         assert!($a.len() == $b.len(), 
//                 "{:?}: dimension mismatch: {:?} {:?} {:?}",
//                 stringify!($func),
//                 ($a.len(),),
//                 stringify!($op),
//                 ($b.len(),));
//             )
// }

// macro_rules! op {
//     ($func: ident, $bound: ident, $op:tt, $method:ident) => (
//         fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
//             assert_equal_len!(xs, ys, $func, $op);
//             for (x, y) in xs.iter_mut().zip(ys.iter()) {
//                 *x = $bound::$method(*x, *y);
//                 // *x = x.$method(*y);
//             }
//         }
//     )
// }

// op!(add_assign, Add, +=, add);
// op!(mul_assign, Mul, *=, mul);
// op!(sub_assign, Sub, -=, sub);


// mod test {
//     use std::iter;
//     macro_rules! test {
//         ($func:ident, $x:expr, $y:expr, $z:expr) => {
//             #[test]
//             fn $func() {
//                 for size in 0usize..10 {
//                     let mut x: Vec<_> = iter::repeat($x).take(size).collect();
//                     let y: Vec<_> = iter::repeat($y).take(size).collect();
//                     let z: Vec<_> = iter::repeat($z).take(size).collect();

//                     super::$func(&mut x, &y);
//                     assert_eq!(x, z);
//                 }
//             }
//         }
//     }
//     test!(add_assign, 1u32, 2u32, 3u32);
//     test!(mul_assign, 2u32, 3u32, 6u32);
//     test!(sub_assign, 3u32, 2u32, 1u32);
// }

// macro_rules! calculate {
//     (eval $e:expr) => { {
//         {
//             let val: usize = $e;
//             println!("{} = {}", stringify!{$e}, val);
//         }
//     }};
// }


// fn main() {
//     calculate!{
//         eval 1 + 2
//     }
//     calculate! {
//         eval (1 + 2) * (3 / 4) 
//     }
// }

// macro_rules! calculate {
//     (eval $e:expr) => {{
//         {
//             let val: usize = $e;
//             println!("{} = {}",stringify!{$e}, val);
//         }
//     }};

//     (eval $e:expr, $(eval $es:expr), +) => {{
//         calculate! {eval $e}
//         calculate! { $(eval $es), + }
//     }};
// }

// fn  main() {
//     calculate! {
//         eval 1 + 2,
//         eval 3 + 4,
//         eval (1 + 2) * (3 / 4) 
//     }
// }

// fn write() {
//         let cpe_mode = vec!("watsons", "nexus", "watsons_ha");
//         let  mut mode = String::new();
//         for cpe in cpe_mode {
//             if cpe.len() != 0 {
//                 mode = format!("{}/", cpe);

//             } else {
//                 mode = cpe.to_string();
//             }
//         }
//         println!("{}", mode);
// }
// fn main() {
//     write()
// }
// #![allow(dead_code)]
// enum WebEvent {
//     PageLoad,
//     PageUnload,
//     KeyPress(char),
//     Paste(String),
//     Click { x: i64, y: i64 },

// }


// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad   => println!("page loaded"),
//         WebEvent::PageUnload => println!("page unloaded"),
//         WebEvent::KeyPress(c)   => println!("pressed '{}'.",c),
//         WebEvent::Paste(s)    => println!("pasted \"{}\"", s),
//         WebEvent::Click { x, y } => {
//             println!("clicked at x={}, y={}.", x, y);
//         }
//     }
// }

// fn main() {

//     let pressed = WebEvent::KeyPress('x');
//     let pasted  = WebEvent::Paste("my text".to_owned());
//     let clik    = WebEvent::Click { x: 20, y: 30 };
//     let load    = WebEvent::PageLoad;
//     let unload  = WebEvent::PageUnload;

//     inspect(pressed);
//     inspect(pasted);
//     inspect(clik);
//     inspect(load);
//     inspect(unload);
// }

// enum VeryVerboseEnum {
//     Add,
//     Subtract,
// }

// impl VeryVerboseEnum {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
//             Self::Add => x + y,
//             Self::Subtract => x - y,
//         }
//     }
// }

// fn main() {
    
// }

// #![allow(dead_code)]

// enum Status {
//     Rich,
//     Poor,
// }

// enum Work {
//     Civilian,
//     Soldier,
// }


// fn main() {
//     use Status::{Poor, Rich};
//     use Work::*;

//     let status = Poor;
//     let work = Civilian;

//     match status {
//         Rich => println!("The rich have lots of money!"),
//         Poor => println!("The poor have no money..."),
//     }

//     match work {
//         Civilian => println!("Civilians work!"),
//         Soldier  => println!("Soldiers fight!"),
//     }
// }


// #![allow(dead_code)]

// enum Number {
//     Zero,
//     One,
//     Two,
//     Seven,
// }

// enum Color {
//     Red   = 0xff0000,
//     Green = 0x00ff00,
//     Blue  = 0x0000ff, 
// }


// fn main() {
//     println!("Zer is {}", Number::Zero as i32);
//     println!("One is {}", Number::One as i32);
//     println!("Seven is {}", Number::Seven as i32);

//     println!("roses are #{:06x}", Color::Red as i32);
//     println!("violets are #{:06x}", Color::Blue as i32);
// }


// use List::*;

// enum List {
//     Cons(u32, Box<List>),
//     Nil,
// }

// impl List {
    
//     fn new() -> List {
//         Nil
//     }

//     fn pressed(self, elem: u32) -> List {
//         Cons(elem, Box::new(self))
//     }

//     fn len(&self) -> u32 {
//         match *self {
//             Cons(_, ref tail) => 1 + tail.len(),
//             Nil => 0,
//         }
//     }


//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 format!("{}, {}", head, tail.stringify())
//             },
//             Nil  => {
//                 format!("Nil")
//             },
//         }
//     }
// }

// fn main() {

//     let mut list = List::new();
//     list = list.pressed(1);
//     list = list.pressed(2);
//     list = list.pressed(3);


//     println!("linked list has length: {}", list.len());
//     println!("{}", list.stringify());
// }

// static LANGUAGE: &'static str = "Rust";
// const  THRESHOLD: i32 = 10;

// fn is_big(n: i32) -> bool {
//     n >  THRESHOLD
// }


// fn main() {
//     let  n = 16;
//     println!("This is {}", LANGUAGE);
//     println!("The threshold is {}", THRESHOLD);
//     println!("{} is {}", n, if is_big(n) { "big" } else { "small" })
// } 


// fn main() {
//     let an_integer = 1u32;
//     let a_boolean = true;
//     let unit = ();

//     let copied_integer = an_integer;

//     println!("An integer: {:?}", copied_integer);
//     println!("A boolean: {:?}", a_boolean);
//     println!("Meet the unit value: {:?}", unit);


//     let _unused_variable = 3u32;
//     let _noisy_unused_variable = 2u32;

//     let mut _h1 = String::from("Hello world");
// }


fn main() {
    let _mutable_binding = 1;
    let mut mutable_binding = 1;
    println!("{}", mutable_binding);

    mutable_binding += 1;
    println!("{}", mutable_binding);
}