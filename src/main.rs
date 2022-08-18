fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let tgn_latitude_degree: f64 =  41.12855889275647; 
    let tgn_longitude_degrees: f64= 1.2444100691709803;

    let gjn_latitude_degrees: f64 = 43.534658538404635;
    let gjn_longitude_degrees: f64 = -5.67706264426499;

    let tgn_latitude_radians: f64 = tgn_latitude_degree.to_radians();
    let gjn_latitude_radians:f64 = gjn_latitude_degrees.to_radians();

    let delta_latitude = (tgn_latitude_degree - gjn_latitude_degrees).to_radians();
    let delta_longitude = (tgn_longitude_degrees - gjn_longitude_degrees).to_radians();
  
    let inner_central_angle = f64::powi((delta_latitude/2.0).sin(), 2)
        +tgn_latitude_radians.cos()
        *gjn_latitude_radians.cos()
        *f64::powi((delta_longitude/2.0).sin(), 2);

    let central_angle=2.0 * inner_central_angle.sqrt().asin();

    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

    println!("The distance between the two points is {:.1} kilometers", distance);
}
