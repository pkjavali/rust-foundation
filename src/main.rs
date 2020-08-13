// testing a simple trait
trait MyNumberTrait {
    fn if_negative(&self) -> bool;
    fn print_sign(&self) {
        println!("The sign of number is {}", self.if_negative());
    }
}

impl MyNumberTrait for MyNumberStruct {
    fn if_negative(&self) -> bool {
        self.value > 0
    }
}

struct MyNumberStruct {
    sign: bool,
    value: i32,
}

impl MyNumberStruct {
    fn new() -> Self {
        MyNumberStruct {
            sign: false,
            value: 200,
        }
    }
}

fn main() {
    let mut num = MyNumberStruct::new();
    num.sign = false;
    num.value = -200;
    println! {"The sign of num.sign is {}", num.sign}; //original sign as stored in struct
    println! {"The sign of num.if_negative() is {}", num.if_negative()}; //sign computed by self trait
    num.sign = !num.sign;
    println! {"The sign of num.sign is {}", num.sign}; //struct member modified
    num.print_sign(); //sign as computed by self trait but being called by original trait other function
}
