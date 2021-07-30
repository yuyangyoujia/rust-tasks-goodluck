  
#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light {
    fn time(&self) -> i32;
}

impl Light for TrafficLight{
    fn time(&self) -> i32 {
        return match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 10,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("If traffic light is {:?},it will last {:?} seconds",red, red.time());
    println!("If traffic light is {:?},it will last {:?} seconds",green, green.time());
    println!("If traffic light is {:?},it will last {:?} seconds",yellow, yellow.time());
}