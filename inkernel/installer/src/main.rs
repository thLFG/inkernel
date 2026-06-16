fn main() {
    let kernel_path = "target/x86_64-unknown-none/debug/inkernel";
    let mut bootloader = bootloader::Bootlink::new(std::path::Path::new(kernel_path));
    bootloader
        .build_iso(std::path::Path::new("target/inkernel.iso"))
        .unwrap();
    println!("Holy moly!!1! This shit is work!");
}
