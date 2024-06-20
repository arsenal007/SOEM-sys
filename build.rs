extern crate bindgen;

use std::{env, format, path::PathBuf};

fn main() {
    let soem_lib_path = "../SOEM_Project/build/";
    let soem_include_path = "../SOEM/";
    let soem_osal_include_path = "../SOEM/osal/"; 
    let soem_osal_def_include_path = "../SOEM_Project/source/osal/";
    let soem_oshw_include_path = "../SOEM_Project/source/oshw/";

    println!("cargo:rustc-link-search=native={}", soem_lib_path);
    println!("cargo:rustc-link-lib=static=soem");
    println!("cargo:include={}", soem_include_path);
    println!("cargo:osal_include={}", soem_osal_include_path);

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", soem_include_path))
        .clang_arg(format!("-I{}/soem", soem_include_path))
	.clang_arg(format!("-I{}", soem_osal_include_path))
        .clang_arg(format!("-I{}", soem_osal_def_include_path))
        .clang_arg(format!("-I{}", soem_oshw_include_path))
        .header("wrapper.h")
        .allowlist_function("ec(x?)_(.*)")
        .allowlist_type("ec_fmmu")
        .allowlist_type("ec_group")
        .allowlist_type("ec_slave")
        .allowlist_type("ec_sm")
        .allowlist_type("ec_state")
        .allowlist_type("ecx_contextt")
        .allowlist_type("ecx_portt")
        .allowlist_type("ecx_redportt")
        .opaque_type("ec_PDOassignt")
        .opaque_type("ec_PDOdesct")
        .opaque_type("ec_SMcommtypet")
        .opaque_type("ec_eepromFMMUt")
        .opaque_type("ec_eepromSMt")
        .opaque_type("ec_eringt")
        .opaque_type("ec_idxstackT")
        .opaque_type("ecx_portt")
        .opaque_type("ecx_redportt")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
