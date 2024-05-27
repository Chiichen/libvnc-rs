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
    #[cfg(target_os = "android")]
    compile_error!("Unsupported Target Android");

    let mut config = cmake::Config::new("libvncserver");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_triple = env::var("TARGET").unwrap();
    if target_triple != env::var("HOST").unwrap() {
        if !cfg!(target_os = "linux") {
            //cfg!(target_os) in build.rs means the host os that build script is running
            panic!("Cross-compilation on platforms other than linux is not supported")
        }
        if target_os == "windows" {
            config.define(
                "CMAKE_TOOLCHAIN_FILE",
                "../cmake/Toolchain-cross-mingw32-linux.cmake",
            );
        }
    } else {
        if target_os == "windows" {
            config.define(
                "CMAKE_TOOLCHAIN_FILE",
                "C:/vcpkg/scripts/buildsystems/vcpkg.cmake", //TODO BETTER toolchain path
            );
            config.define("WITH_OPENSSL", "OFF");
            config.define("WITH_GNUTLS", "OFF");
            config.define("WITH_GCRYPT", "OFF");
        }
    }

    //TODO In WSL, if QT is installed in Windows system, then the build process might fail on Qt example.
    let dst = config.build();
    println!("cargo:rustc-link-lib=dylib=vncserver");
    println!("cargo:rustc-link-lib=dylib=vncclient"); //There's no libvncclient , so we need to specify the vncclient manually
    println!("cargo:rustc-link-search={}/build", dst.display(),);
    let rfb_header = format!("{}/{}", dst.display(), "include/rfb/rfb.h");
    let rfbclient_header = format!("{}/{}", dst.display(), "include/rfb/rfbclient.h");
    let bindings = bindgen::Builder::default()
        .header(rfb_header)
        .header(rfbclient_header)
        .use_core()
        .clang_arg(format!("-I{}/{}", dst.display(), "include"))
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
