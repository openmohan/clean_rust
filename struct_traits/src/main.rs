struct Car{
    name: String,
    number: String
}

impl Car {
    fn new(name: &str, number: &str) -> Car {
        return Car {
            name: String::from(name),
            number: String::from(number),
        }
    }

    fn drive(&self) -> String {
        println!("car driving print");
        return String::from("car driving")
    }
}

trait Vehicle{
    fn drive(&self) -> String;
}

impl Vehicle for Car {
    fn drive(&self) -> String {
        return self.drive()
    }
}

fn drive_vehicle(v:&dyn Vehicle) {
    println!("from drive vehicle");
    v.drive();
}

fn main() {
    println!("Hello, world!");
    let c = Car::new("test", "number");
    println!("{}",c.drive());
    drive_vehicle(&c)
}
