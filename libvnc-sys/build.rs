use std::env;
use std::path::PathBuf;

#[cfg(feature = "pkg")]
fn bindgen_vncserver() {
    let libvncserver =
        pkg_config::probe_library("libvncserver").expect("libvncserver package not found. Please install libvncserver/libvncserver-dev/libvncserver-devel with your package manager ");

    for link_path in libvncserver.link_paths {
        println!("cargo:rustc-link-search={}", link_path.display());
    }

    for lib in libvncserver.libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
    println!("cargo:rustc-link-lib=dylib=vncclient"); //There's no libvncclient , so we need to specify the vncclient manually
    let rfb_header = format!(
        "{}/{}",
        libvncserver.include_paths[0].to_str().unwrap(),
        "rfb/rfb.h"
    );
    let rfbclient_header = format!(
        "{}/{}",
        libvncserver.include_paths[0].to_str().unwrap(),
        "rfb/rfbclient.h"
    );
    let bindings = bindgen::Builder::default()
        .header(rfb_header)
        .header(rfbclient_header)
        .use_core()
        .clang_arg(format!(
            "-I{}",
            libvncserver.include_paths[0].to_str().unwrap(),
        ))
        .generate()
        .expect("unable to generate rfb bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("rfb.rs"))
        .expect("couldn't write bindings!");
}
#[cfg(not(feature = "pkg"))]
fn bindgen_vncserver() {
    use std::{
        env::current_dir,
        fs::{self, create_dir_all},
        process::Command,
        vec,
    };

    #[cfg(target_os = "android")]
    compile_error!("Unsupported Target Android");

    // https://github.com/LibVNC/libvncserver/issues/628 use cmake in plain command line to avoid compile error on windows
    // let mut config = cmake::Config::new("libvncserver");
    // let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    // let target_triple = env::var("TARGET").unwrap();
    // if target_triple != env::var("HOST").unwrap() {
    //     if !cfg!(target_os = "linux") {
    //         //cfg!(target_os) in build.rs means the host os that build script is running
    //         panic!("Cross-compilation on platforms other than linux is not supported")
    //     }
    //     if target_os == "windows" {
    //         config.define(
    //             "CMAKE_TOOLCHAIN_FILE",
    //             "../cmake/Toolchain-cross-mingw32-linux.cmake",
    //         );
    //     }
    // } else if target_os == "windows" {
    //     config.define(
    //         "CMAKE_TOOLCHAIN_FILE",
    //         which::which("vcpkg")
    //             .expect("Install vcpkg and make sure it can be found in current environment")
    //             .parent()
    //             .unwrap()
    //             .join("scripts/buildsystems/vcpkg.cmake"), //TODO BETTER toolchain path
    //     );
    //     config.define("WITH_OPENSSL", "OFF");
    //     config.define("WITH_GNUTLS", "OFF");
    //     config.define("WITH_GCRYPT", "OFF");
    // } else if target_os == "android" {
    //     panic!("unsupported build target {}", target_os)
    // }
    // //TODO In WSL, if QT is installed in Windows system, then the build process might fail on Qt example.
    // let dst = config.build();
    // >>> Manually build libvncserver with cmake
    let mut dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    dst.push("build");
    let libvncserver_path = current_dir()
        .unwrap()
        .join("libvncserver")
        .display()
        .to_string();
    let mut cmake_args = vec![];
    cmake_args.push(libvncserver_path.as_str());
    // let cmake_install_prefix_arg = format!(r#"-DCMAKE_INSTALL_PREFIX="{}""#, dst.display());
    // cmake_args.push(&cmake_install_prefix_arg);
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_triple = env::var("TARGET").unwrap();
    if target_triple != env::var("HOST").unwrap() {
        if !cfg!(target_os = "linux") {
            //cfg!(target_os) in build.rs means the host os that build script is running
            panic!("Cross-compilation on platforms other than linux is not supported")
        }
        if target_os == "windows" {
            cmake_args
                .push(r#"-DCMAKE_TOOLCHAIN_FILE="../cmake/Toolchain-cross-mingw32-linux.cmake""#);
        }
    } else if target_os == "windows" {
        // config.define(
        //     "CMAKE_TOOLCHAIN_FILE",
        //     which::which("vcpkg")
        //         .expect("Install vcpkg and make sure it can be found in current environment")
        //         .parent()
        //         .unwrap()
        //         .join("scripts/buildsystems/vcpkg.cmake"),
        // );
        cmake_args.push("-DWITH_OPENSSL=OFF");
        cmake_args.push("-DWITH_GNUTLS=OFF");
        cmake_args.push("-DWITH_GCRYPT=OFF");
    } else if target_os == "android" {
        panic!("unsupported build target {}", target_os)
    }
    //Make sure dst is cleaned up
    if !dst.is_dir() {
        let _ = fs::remove_dir_all(dst.as_path());
        let _ = create_dir_all(dst.as_path());
    }
    let mut command = Command::new("cmake");
    command
        .current_dir(dst.as_path())
        .args(cmake_args)
        .env("CMAKE_INSTALL_PREFIX", dst.display().to_string());
    let status = command.status().unwrap();
    if !status.success() {
        panic!("Failed to run {:?}", command)
    }

    let mut command = Command::new("cmake");
    command
        .current_dir(dst.as_path())
        .arg("--build")
        .arg(".")
        .arg("--target")
        .arg("install");
    let status = command.status().unwrap();
    println!("command : {:?} status:{:?}", command, status);
    if !status.success() {
        panic!("Failed to run {:?}", command)
    }
    // << Manually build libvncserver done
    println!("cargo:rustc-link-lib=dylib=vncserver");
    println!("cargo:rustc-link-lib=dylib=vncclient"); //There's no libvncclient , so we need to specify the vncclient manually
    println!("cargo:rustc-link-search={}", dst.display());
    let rfb_header = format!("{}/{}", dst.display(), "include/rfb/rfb.h");
    let rfbclient_header = format!("{}/{}", dst.display(), "include/rfb/rfbclient.h");
    let bindings = bindgen::Builder::default()
        .header(rfb_header)
        .header(rfbclient_header)
        .use_core()
        .clang_args([
            format!("-I{}/{}", dst.display(), "include"),
            #[cfg(target_os = "windows")]
            format!("-DWIN32"),
        ])
        .generate()
        .expect("unable to generate rfb bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("rfb.rs"))
        .expect("couldn't write bindings!");
}
fn main() {
    bindgen_vncserver();
}
