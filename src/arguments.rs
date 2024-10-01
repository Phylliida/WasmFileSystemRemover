use clap::Parser;

// modified from https://github.com/wasm-forge/wasi2ic/tree/main
#[derive(Parser, Debug)]
#[command(version, about=format!("Wasi dependency removal V{}", env!("CARGO_PKG_VERSION")), long_about = None)]
pub struct WasmFsRemoverArgs {
    /// Quiet mode
    #[arg(long, short, default_value_t = false)]
    pub quiet: bool,

    /// Input file to process (*.wasm or *.wat).
    pub input_file: String,

    /// Output file to store the processed Wasm (*.wasm).
    #[arg(default_value_t = String::from("no_wasi.wasm"))]
    pub output_file: String,
}