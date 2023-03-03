use std::process::Command;

#[path = "src/shaders/mod.rs"]
mod shaders;

fn main() {
    println!("cargo:rerun-if-changed=src/shaders");
    // println!("cargo:rustc-link-search=all=target/debug/");
    // compile_ios();
    // shaders::gen();
    compile_macos();
    // compile_air();
    // compile_lib();
}

fn compile_macos() {
    let output = Command::new("xcrun")
        .args(["-sdk", "macosx"])
        .arg("metal")
        .arg("-std=metal3.0")
        .arg("src/shaders/rolled.metal")
        .args(["-o", "src/shaders/rolled_macos.metallib"])
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

fn compile_ios() {
    let output = Command::new("xcrun")
        .args(["-sdk", "iphoneos"])
        .arg("metal")
        .arg("-std=metal3.0")
        .arg("src/shaders/rolled.metal")
        .args(["-o", "src/shaders/rolled_ios.metallib"])
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

// #[allow(dead_code)]
// fn compile_air() {
//     // xcrun -sdk macosx metal -gline-tables-only -frecord-sources shader.metal -c -o shader.air
//     let output = Command::new("xcrun")
//         // xcrun options
//         .args(["-sdk", "macosx"])
//         // tool name
//         .arg("metal")
//         // options not in Metal man page, but on Metal tool page
//         .arg("-frecord-sources")
//         // options in Metal man page
//         // State Selection Options
//         .arg("-c")
//         // Language Selection and Mode Options
//         // .args(["-x", ""])
//         .arg("-std=metal3.0")
//         // Target Selection Options
//         // .arg("")
//         // Code Generation Options
//         .arg("-O0")
//         .arg("-g")
//         // .arg("-gline-tables-only")
//         // Driver Options
//         .args(["-o", "shaders/shader.air"])
//         // filename
//         .arg("shaders/shader.metal")
//         .output()
//         .unwrap();
//     if !output.status.success() {
//         let stdout = String::from_utf8(output.stdout).unwrap();
//         let stderr = String::from_utf8(output.stderr).unwrap();
//         panic!(
//             r#"
// stdout: {}
// stderr: {}
// "#,
//             stdout, stderr
//         );
//     }
// }

// #[allow(dead_code)]
// fn compile_lib() {
//     // xcrun -sdk macosx metallib shader.air -o shader.metallib
//     let output = Command::new("xcrun")
//         // xcrun options
//         .args(["-sdk", "macosx"])
//         // tool name
//         .arg("metallib")
//         // options in metallib tool page
//         .args(["-o", "shaders/shader.metallib"])
//         // filename
//         .arg("shaders/shader.air")
//         .output()
//         .unwrap();
//     if !output.status.success() {
//         let stdout = String::from_utf8(output.stdout).unwrap();
//         let stderr = String::from_utf8(output.stderr).unwrap();
//         panic!(
//             r#"
// stdout: {}
// stderr: {}
// "#,
//             stdout, stderr
//         );
//     }
// }

// cc::Build::new()
//     .file("src/double.m")
//     .flag("-fobjc-arc")
//     .flag("-fmodules")
//     // .flag("-framework")
//     // .flag("CoreGraphics")
//     // .flag("-framework")
//     // .flag("Metal")
//     .compile("libdouble.a");
