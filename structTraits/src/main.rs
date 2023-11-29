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

trait vehicle{
    fn drive(&self) -> String;
}

impl vehicle for Car {
    fn drive(&self) -> String {
        return self.drive()
    }
}

fn driveVehicle(v:&dyn vehicle) {
    println!("from drive vehicle");
    v.drive();
}

fn main() {
    println!("Hello, world!");
    let c = Car::new("test", "number");
    println!("{}",c.drive());
    driveVehicle(&c)
}
