use std::env;

pub fn additional_clang_args(target: &str, arch: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    match arch {
        "i686" => args.push("-m32".into()),
        "x86_64" => args.push("-m64".into()),
        _ => {}
    }

    let ndk = ndk();

    args.push(format!("--sysroot={}/sysroot", ndk));
    args.push(format!("-I{}/sysroot/usr/include/{}", ndk, target));
    args.push(format!(
        "-isystem{}/sources/cxx-stl/llvm-libc++/include",
        ndk
    ));
    args.push(format!("--target={}", target));
    args
}

pub fn ndk() -> String {
    env::var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}
