fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = tonic_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::configure()
        .include_file("mod.rs")
        .compile_protos_with_config(
            config,
            &[
                "proto/rpc/cc/arduino/cli/commands/v1/common.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/board.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/commands.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/compile.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/lib.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/upload.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/debug.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/monitor.proto",
                "proto/rpc/cc/arduino/cli/commands/v1/settings.proto",
                "polyfill/google/rpc/status.proto"
            ],
            &["proto/rpc/", "polyfill"],
        )?;
    // panic!();
    Ok(())
}
