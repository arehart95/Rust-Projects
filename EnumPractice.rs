
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown =>
                println!("This location is unknown."),
            Location::Anonymous =>
                println!("This location is anonymous."),
            Location::Known(lon, lat) =>
                println!("This location is at {}, {}", lon, lat)
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(34.38384, 153.37225);
    address.display();

 
}
