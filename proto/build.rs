fn main() {
    // let dir = env!("OUT_DIR");

    tonic_build::configure()
        // .out_dir(dir)
        .compile(&["src/service.proto"], &["src"])
        .expect("generate service proto failed");

    // let mut message_proto_config = prost_build::Config::default();
    // message_proto_config.out_dir(dir);

    prost_reflect_build::Builder::new()
        .compile_protos(&["src/message.proto"], &["."])
        .expect("generate message proto failed");
}
