fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/greeter.proto"], &["proto"])
        .unwrap_or_else(|e| {
            eprintln!("\n========================================");
            eprintln!("ERROR: Failed to compile protobuf files");
            eprintln!("========================================\n");
            eprintln!(
                "This example requires 'protoc' (Protocol Buffers compiler) to be installed.\n"
            );
            eprintln!("Installation instructions:");
            eprintln!(
                "  - Windows: Download from https://github.com/protocolbuffers/protobuf/releases"
            );
            eprintln!("           Add protoc.exe to your PATH");
            eprintln!("  - macOS: brew install protobuf");
            eprintln!("  - Linux: sudo apt-get install protobuf-compiler (Debian/Ubuntu)");
            eprintln!("          or yum install protobuf-compiler (RHEL/CentOS)");
            eprintln!("\nOriginal error: {}\n", e);
            std::process::exit(1);
        });
}
