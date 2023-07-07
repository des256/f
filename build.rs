use {
    std::{
        env,
        fs,
        process,
        path,
    },
};

// define the possible main configuration branches of the code
pub enum System {
    Linux,
    Windows,
    Macos,
    Android,
    Ios,
    Web,
}

fn main() {

    // make current build available to source code
    println!("cargo:rustc-cfg=build={:?}",env::var("PROFILE").unwrap());

    // select which main branch to compile
#[cfg(target_os="linux")]
    let (system,system_name) = (System::Linux,"linux");  // Vulkan, Opengl, Gles
#[cfg(target_os="windows")]
    let (system,system_name) = (System::Windows,"windows");  // Vulkan, Opengl, Directx
#[cfg(target_os="macos")]
    let (system,system_name) = (System::Macos,"macos");  // Vulkan, Metal
#[cfg(target_os="android")]
    let (system,system_name) = (System::Android,"android");  // Vulkan, Gles
#[cfg(target_os="ios")]
    let (system,system_name) = (System::Ios,"ios");  // Vulkan, Metal
#[cfg(target_family="wasm")]
    let (system,system_name) = (System::Web,"web");  // Webgl, Webgpu

    // this enables #[cfg(system="...")] conditional compilation in the entire source code
    println!("cargo:rustc-cfg=system=\"{}\"",system_name);

    if let System::Web = system {

        // TBD: Web doesn't use FFI
    }
    
    else {

        // create header file that contain the includes to generate FFI bindings for
        // and print any cargo link settings to stdout
        let mut header = String::new();

        match system {

            System::Linux => {

                header.push_str("#include <sys/epoll.h>\n");
                header.push_str("#include <X11/Xlib.h>\n");
                header.push_str("#include <X11/Xlib-xcb.h>\n");
                header.push_str("#include <xcb/xcb.h>\n");
                println!("cargo:rustc-link-lib=X11");
                println!("cargo:rustc-link-lib=X11-xcb");
                println!("cargo:rustc-link-lib=xcb");

                // Vulkan
                println!("cargo:rustc-cfg=vulkan");
                header.push_str("#include <vulkan/vulkan.h>\n");
                header.push_str("#include <vulkan/vulkan_xcb.h>\n");
                println!("cargo:rustc-link-lib=vulkan");
            },

            System::Windows => {

                // Vulkan
                println!("cargo:rustc-cfg=vulkan");
                header.push_str("#include <vulkan/vulkan.h>\n");
                header.push_str("#include <vulkan/vulkan_win32.h>\n");
                println!("cargo:rustc-link-lib=vulkan");
            },

            System::Macos => {

                // Vulkan
                println!("cargo:rustc-cfg=vulkan");
                header.push_str("#include <vulkan/vulkan.h>\n");
                header.push_str("#include <vulkan/vulkan_macos.h>\n");
                println!("cargo:rustc-link-lib=vulkan");
            },

            System::Android => {

                // Vulkan
                println!("cargo:rustc-cfg=vulkan");
                header.push_str("#include <vulkan/vulkan.h>\n");
                header.push_str("#include <vulkan/vulkan_android.h>\n");
                println!("cargo:rustc-link-lib=vulkan");
            },

            System::Ios => {

                // Vulkan
                println!("cargo:rustc-cfg=vulkan");
                header.push_str("#include <vulkan/vulkan.h>\n");
                header.push_str("#include <vulkan/vulkan_ios.h>\n");
                println!("cargo:rustc-link-lib=vulkan");
            },

            System::Web => {

            },
        }

        // write the header file to sys.h
        let header_path = path::Path::new("src/sys/sys.h");
        fs::write(&header_path,header).expect("Unable to write header file");

        // generate sys.rs FFI bindings from the includes
        process::Command::new("bindgen")
            .args(&[
                &format!("src/sys/sys.h"),
                "-o",&format!("src/sys/sys.rs"),
                "--disable-nested-struct-naming",
                "--no-prepend-enum-name",
                "--no-layout-tests",
            ])
            .status()
            .expect("unable to generate system FFI bindings");

        // create mod.rs to complete the sys module with FFI bindings
        let mut sysmod = String::new();
        sysmod.push_str("#![allow(non_camel_case_types)]\n");
        sysmod.push_str("#![allow(non_upper_case_globals)]\n");
        sysmod.push_str("#![allow(non_snake_case)]\n");
        sysmod.push_str("#![allow(dead_code)]\n\n");
        sysmod.push_str("include!(\"sys.rs\");\n");

        let sysmod_path = path::Path::new("src/sys/mod.rs");
        fs::write(&sysmod_path,sysmod).expect("Unable to write module file");
    }

    // and indicate to rerun only if changed
    println!("cargo:rerun-if-changed=build.rs");
}
