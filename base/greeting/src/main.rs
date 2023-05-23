/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let x = add(1, 2);
///
/// ```

// extern crate time;
// use time::*;

// fn main() {
    // let a = 12;
    // let x = 2.0;
    // let y : f32 = 3.0;
   
    // print!("a is {}, b is {0}\n", a);
    // print!("The value of x is: {},y is:{}\n",x,y);

    // let sum = 5 + 10; //加
    // let difference = 95.5 - 4.3; //减
    // let product = 4 * 30; //乘
    // let quotient = 56.7 / 32.2; //除
    // let remainder = 43 % 5;  //求余
    // println!("{}/{}/{}/{}/{}",sum,difference,product,quotient,remainder);

    //不支持自运算  不支持 ++ 和 --，
    //布尔型 bool true/false
    //字符型 char // Rust 中字符串和字符都必须使用 UTF-8 编码
    // 复合类型  元组 ()包括的一组数据，可以包含不同种类的数据：
    // 数组用一对 [ ] 包括的同类型数据。

    // let tup : (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1

    /*
    let (x, y, z) = tup;
    println!("{}/{}/{}",x,y,z);

    let a = [1,2,3,4,5,6];  // a 是一个长度为 5 的整型数组
    let b = ["January","February","March"];  // b 是一个长度为 3 的字符串数组
    let c: [i32;5] = [1,2,3,4,5];   // c 是一个长度为 5 的 i32 数组
    let d = [3;5];  // 等同于 let d = [3, 3, 3, 3, 3];

    let first = a[0]; // 数组访问
    let second = a[1];
    // println!("{}/{}",a,b);
    // println!("{}/{}/{}/{}",a,b,c,d);

    // let mut a = [1, 2, 3];
    // a[0] = 123;
    // a [0] = 4;
    */

    // 这是第一种注释方式

/* 这是第二种注释方式 */

/*
 * 多行注释
 * 多行注释
 * 多行注释
 */

// println!("{}",add(3, 4))
// }

//函数
// fn main() {
//     println!("{}",add(4, 9))
// }
// fn add(a:i32, b: i32) -> i32 {
//     return a + b;
// }
//fn <函数名> （<参数>） <函数体>

// fn main() {
//     println!("hello, world!");
//     another_function();
// }

// fn another_function() {
//     println!("Hello, runoob!");
// }

//函数参数
// fn main() {
//     another_function(5,6);
//     let a = 6;
//     // le a = (let b = 2);
    
// }

// fn another_function(x: i32, y: i32)  {
//     println!("x的值为 : {}", x);
//     println!("y的值为 : {}", y);
//     println!("x*y的值 : {}", x*y);
//     println!("x/y的值 : {}", x/y);
//     println!("x%y的值 : {}", x%y);
// }


//函数体的语句和表达式
// fn main() {
//     let x = 5;
//     let y = {
//         let x = 3;
//         x + 1   //注意：x + 1 之后没有分号，否则它将变成一条语句！一个合法的函数体
//     };
//     println!("x 的值为 : {}",x);
//     println!("y 的值为 : {}",y);
// }

//函数可以嵌套
// fn main() {
//     fn five() -> i32 {
//        let  x = 5;
//         let y = 6 + x;
//         return y
//     }
//     println!("five() 的值为: {}", five());
//     println!("add函数返回值: {}", add(3,6))
// }

// //函数返回值
// fn add(a: i32, b: i32) -> i32 {
//     return a + b; 
// }

// fn main() {
    // let number = 3;
    // if number < 5 {
    //     println!("true");
    // } else {
    //     println!("false");
    // }

    // let a = 12;
    // let b;
    // if a > 0 {
    //     b = 1;
    // }  
    // else if a < 0 {
    //     b = -1;
    // }  
    // else {
    //     b = 0;
    // }
    // println!("b is {}", b);

    // let number = 3; 
    // if number { //expected `bool`, found integer
    //     println!("Yes");
    // }

    // if <condition> { block 1 } else { block 2 } 
    // let a = 3;
    // let number = if a > 0 {2} else {-3};  //两个函数体表达式的类型必须一样！且必须有一个 else 及其后的表达式块。
    // println!("number 为 {}", number)
// }

// fn main() {
//     let num = -5;
//     if num > 0 {
//         println!("number is greater than 0");
//     } else if  num < 0 {
//         println!("number is less than 0");
//     } else {
//         println!("number is not equal to 0");
//     }
// }

// fn main() {
//     let a = 5;
//     let b = 5;
//     if a != b {
//         if a > b {
//             println!("a is greater than b");
//         } else {
//             println!("a is less than b");
//         }
//     } else {
//         println!("a is equal to b");
//     }
// }


// fn main() {
//   let a = if true {
//         1
//     } else {
//         2
//     };
//     println!("value of a is {}", a);
// }

fn main() {
    let a = if false {
        "bc"
    } else {
       "ad"
    };
    println!("value of a is: {}", a);
}




//循环 wilhe
// fn main() {
    // let mut number = 1;
    // while number != 4 {
    //     println!("{}", number);
    //     number += 1;
    // }
    // println!("EXIT");

    // let mut i = 0;
    // while i < 10 {
    //     i += 1;
    //     println!("{}",i);
    // }

    // let a = [10, 20, 30, 40, 50];
    // for i in a.iter() {
    //     println!("值为 : {}", i);
    // }

    // for i in 0..5 {
    //     println!("a[{}] = {}", i, a[i]);
    // }

    //loop 循环
//     let s = ['R','U','N','O','B'];
//     let mut i = 0;
//     loop {
//         let ch = s[i];
//         if ch == 'O' {
//             break;
//         }
//         println!("\'{}\'", ch);
//         i += 1;
//     }
//     index()
// }


// fn index() {
//     let s =  ['R','U','N','O','B'];
//     let mut i = 0;
//     let location = loop {
//         let ch = s[i];
//         if ch == 'O' {
//             break i;
//         }
//         i += 1;
//     };
//     println!("\'O\' 的索引为 {}", location);
// }

// fn main() {
//     let s1 = String::from("hello");
//     // let s2 = s1;
//     let s2 = s1.clone();
//     println!("{}",s1);
//     println!("{}",s2);
// }

//所有权
// fn main() {
//     let s1 = String::from("hello");
//     println!("{}", s1);
//     take_own(s1);
//     // println!("{}", s1);

//     let x = 5;
//     can_copy(x);
//     println!("{}",x);
//     // println!("{}", s1);
   

// }



// fn take_own(s: String) {
//     println!("{}", s);
// }

// fn can_copy(i: i32) {
//     println!("{}", i);
// }

// 返回值与作用域
// 返回值也可以转移所有权
// fn main() {
//     let s1 = String::from("hello");

//     let (lenght, s2) = len(s1);

//     println!("{}", lenght);
//     println!("{}",s2);
// }

// fn len(s: String) -> (usize, String) {
//     return (s.len(), s)
// }

// 引用与借用
/*
fn main() {
    let s1 = String::from("hello");
    let lenght = len(&s1);

    println!("{}",lenght);
    println!("{}",s1);
}

fn len(s: &String) -> usize
{
    return s.len()
}
*/

//可变引用
/*
fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);

    println!("{}", s1)

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{},{}",r1,r2)
}

fn change(s: &mut String) {
    s.push_str(", world")
}
*/

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{},{}",r1,r2)
// }


// fn main() {
//    let  now = time::now();
//     let mut i:i64 = 0;
//     while i < 10 {
//         println!("{}",fib(i));
//         i = i + 1;
//     }
//     let end = time::now(); 
//     println!("{}",end-now);
    
// }


// fn fib(n: i64) -> i64  {
//     if n == 0 || n == 1 {
//         return n;
//     }
//     return fib(n-1) + fib(n-2);
// }


