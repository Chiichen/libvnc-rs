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

fn main() {
    bindgen_vncserver();
}
