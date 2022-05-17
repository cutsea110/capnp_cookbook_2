extern crate capnp;

pub mod point_capnp {
    include!(concat!(env!("OUT_DIR"), "/point_capnp.rs"));
}

fn main() {
    let mut builder = capnp::message::Builder::new_default();

    {
        let mut point_msg = builder.init_root::<point_capnp::point::Builder>();
        point_msg.set_x(12);
        point_msg.set_y(14);
    }

    let mut buffer = Vec::new();

    capnp::serialize::write_message(&mut buffer, &builder).unwrap();

    let deserialised = capnp::serialize::read_message(
        &mut buffer.as_slice(),
        capnp::message::ReaderOptions::new(),
    )
    .unwrap();

    let point_reader = deserialised
        .get_root::<point_capnp::point::Reader>()
        .unwrap();

    assert_eq!(point_reader.get_x(), 12);
    assert_eq!(point_reader.get_y(), 14);
}
