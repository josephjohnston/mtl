use std::process::Command;

fn main() {
    // println!("cargo:rerun-if-changed=shader.metal");

    // compile_air();
    // compile_lib();
}

#[allow(dead_code)]
fn compile_air() {
    // xcrun -sdk macosx metal -gline-tables-only -frecord-sources shader.metal -c -o shader.air
    let output = Command::new("xcrun")
        // xcrun options
        .args(["-sdk", "macosx"])
        // tool name
        .arg("metal")
        // options not in Metal man page, but on Metal tool page
        .arg("-frecord-sources")
        // options in Metal man page
        // State Selection Options
        .arg("-c")
        // Language Selection and Mode Options
        // .args(["-x", ""])
        .arg("-std=metal3.0")
        // Target Selection Options
        // .arg("")
        // Code Generation Options
        .arg("-O0")
        .arg("-g")
        // .arg("-gline-tables-only")
        // Driver Options
        .args(["-o", "src/shader.air"])
        // filename
        .arg("src/shader.metal")
        .output()
        .unwrap();
    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!(
            r#"
stdout: {}
stderr: {}
"#,
            stdout, stderr
        );
    }
}

#[allow(dead_code)]
fn compile_lib() {
    // xcrun -sdk macosx metallib shader.air -o shader.metallib
    let output = Command::new("xcrun")
        // xcrun options
        .args(["-sdk", "macosx"])
        // tool name
        .arg("metallib")
        // options in metallib tool page
        .args(["-o", "src/shader.metallib"])
        // filename
        .arg("src/shader.air")
        .output()
        .unwrap();
    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!(
            r#"
stdout: {}
stderr: {}
"#,
            stdout, stderr
        );
    }
}

// cc::Build::new()
//     .file("src/double.m")
//     .flag("-fobjc-arc")
//     .flag("-fmodules")
//     // .flag("-framework")
//     // .flag("CoreGraphics")
//     // .flag("-framework")
//     // .flag("Metal")
//     .compile("libdouble.a");
