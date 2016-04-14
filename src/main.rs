extern crate uuid;
extern crate rand;

mod body;


fn main() {
    let mut body = body::Body::new();
    while true {
        body.run();
        println!("Body pose: ({}, {}, {})", body.pose.x, body.pose.y, body.pose.d);
    }
    println!("Hello, world!");
}
