fn main() {
    println!("cargo:rerun-if-changed=src/proto/service.proto");
    tonic_build::configure()
    .build_server(true)
    .compile(&["src/proto/service.proto"], &["src/proto", "deps/googleapis"])
    .unwrap();
}
