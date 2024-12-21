fn main() -> Result<(),Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(true).build_client(false).out_dir("stubs").compile(&["proto/wamr_sdk.proto"], &["."])?;
    Ok(())
}