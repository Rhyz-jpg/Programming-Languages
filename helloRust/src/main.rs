




fn main() {

    let stack_string = "Hello World";//immutable
    // stack_string = "dadw";//error[E0384]: cannot assign twice to immutable variable `stack_string`

    let mut heap_string = "Goodbye World";//mutable
    heap_string = "Something New";//no error

    // println!(myString);

    let x = 100;
    println!("x is: {}", x);


    let mut s = String::from("My fav Number is: ");
    s = s + "1";// append 1 to the string
    println!("{}", s); // prints: My fav Number is: 1


    let mut m_1 = "String 1";
    let mut m_2 = m_1;
    drop(m_2);
    println!("{}", m_1);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1);

    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // let s1 = String::from("hello");//This allocates a mutable a string variable on the heap
    // let s2 = s1;//Changes owner of the data from s1 to s2
    // println!("{}, world!", s1);//causes error because s1 no longer owns the data


    // fn main() {
    //     let s = String::from("hello");  // s comes into scope
    //     takes_ownership(s); //s leaves the scope
    // }
    // fn takes_ownership(some_string: String) { // some_string comes into scope
    //     println!("{}", some_string);
    // } //value of s is freed

    let mut s = "hello";
    takes_ownership(s);
    println!("{}", s);
   
    s = "dwadwadw";
    println!("New s{}", s);


    //borowwing example: https://www.makeuseof.com/rust-borrowing/#:~:text=An%20Example%20of%20Borrowing%20in,the%20ampersand%20(%26)%20symbol.&text=Without%20borrowing%20by%20referencing%2C%20the,to%20the%20same%20memory%20location.
    let x = String::from("borowwing example"); // x owns "hello"
    let y = &x; // y references x, borrows "hello"
    println!("{}", x); 
    // y = y + " added";
    // println!("{}", y);



    
    let_example();
    scope_example();
    struct_exmaple();
    type_system_example();
    coercion_example();
    scope2_example();
    let a = 32;
    let b = 234.43;
    syntax_example(a, b);
    restricited_aliasing_example();
    println!("{}", return_example());
    println!("{}", return_example2());
    println!("{}", return_example3(0));

    
    let newInt = bad_return_example3(1,2,3);




}

fn takes_ownership(some_string: &str) {
    println!("{}", some_string);
}

fn let_example()
{
    let (x,y) = (1,2);
    println!("x={}\ny={}", x,y);
}

fn scope_example()
{
    let x = 10;
    {
        let x = 20;
        println!("x={}", x);

    }
    println!("x={}", x);
}


//https://doc.rust-lang.org/book/ch05-03-method-syntax.html
fn struct_exmaple()
{
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn type_system_example()
{
    let x = 1;
    let a: i32;
    a=20;
    let z:i32 = 33;
    let geuss: f32 = "117".parse().expect("REQUIRE INT");//from:https://doc.rust-lang.org/book/ch03-02-data-types.html
    println!("x={}", geuss);

}

fn coercion_example()
{
    let x = 1;
    let x = "awdada";
}

fn scope2_example()
{
    let mut x = 1;
    println!("x={}", x);

    {
        let x = 2;
        println!("x={}", x);
    }
    
    println!("x={}", x);
    
}


fn syntax_example(a:i64, b:f32){
    let c:i8 = 4;

}

fn restricited_aliasing_example(){
    let x = String::from("Hello, world!");
    let y = x;
    // println!("x={}", x);
    println!("y={}", y);


    println!("{}", i128::MAX);
}



fn return_example() -> i128{
   3 + 4
}
fn return_example2() -> i128{
    return (3 + 4)
}
fn return_example3(a: i128) -> i128{
    if(a > 0) { a + 1 }
    else { a - 1 }
}

fn bad_return_example3(a: i128,mut b:i128,c:i128) -> i128 {
    if(a == 3)
    {
        while (a < 100)
        {
            if(c < b)
            {
            }
            b = b + 1;
        }
        a+3
    }
    else
    {
        a + 2
    }
}

fn loop_example(){
    let mut x = 0;
    'Loop1: loop {
        'Loop2: loop {
            break 'Loop1;
        }
    }
}


// for x in 0..10 {
//     println!("x={}", x);
// }
