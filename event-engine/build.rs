use std::io::Result;

fn main() -> Result<()> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    prost_build::compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
    Ok(())
}
