extern crate zmq;

fn main() {
    let context = zmq::Context::new();
    let _poll_item = {
        let socket = context.socket(zmq::PAIR).unwrap();
        socket.as_poll_item(0)
    }; //~ ERROR `socket` does not live long enough
}
