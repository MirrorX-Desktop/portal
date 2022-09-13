fn main() {
    tonic_build::configure()
        .out_dir("src")
        .compile(&["src/service.proto"], &["src"])
        .expect("generate service proto failed");

    let mut message_proto_config = prost_build::Config::default();
    message_proto_config.out_dir("src");

    prost_reflect_build::Builder::new()
        .compile_protos_with_config(message_proto_config, &["src/message.proto"], &["."])
        .expect("generate message proto failed");
}
