use std::path::Path;

const ROOT: &str = "c_lib";

fn main() {
    build_lbc();
    build_lpeg();
    build_lrexlib();
    build_lsqlite3();
}

fn create_build() -> cc::Build {
    let mut build = cc::Build::new();
    build.include(Path::new(ROOT).join("lua"));
    build
}

fn build_lbc() {
    let dir = Path::new(ROOT).join("lbc");
    create_build()
        .file(dir.join("lbc.c"))
        .file(dir.join("src").join("number.c"))
        .include(dir.join("src"))
        .include(dir)
        .compile("lbc");
}

fn build_lpeg() {
    let dir = Path::new(ROOT).join("lpeg");
    create_build()
        .warnings(false)
        .file(dir.join("lpcap.c"))
        .file(dir.join("lpcode.c"))
        .file(dir.join("lpprint.c"))
        .file(dir.join("lptree.c"))
        .file(dir.join("lpvm.c"))
        .include(dir)
        .compile("lpeg");
}

fn build_lrexlib() {
    let dir = Path::new(ROOT).join("lrexlib").join("src");
    create_build()
        .define("PCRE2_CODE_UNIT_WIDTH", "8")
        .file(dir.join("common.c"))
        .file(dir.join("pcre2").join("lpcre2_f.c"))
        .file(dir.join("pcre2").join("lpcre2.c"))
        .include(dir)
        .include(Path::new(ROOT).join("pcre2-mirror").join("src"))
        .compile("lrexlib");
}

fn build_lsqlite3() {
    let dir = Path::new(ROOT).join("lsqlite3");
    create_build()
        .warnings(false)
        .file(dir.join("lsqlite3.c"))
        .file(dir.join("extras").join("extension-functions.c"))
        .include(dir)
        .include(Path::new(ROOT).join("sqlite-amalgamation"))
        .compile("lsqlite3");
}
