use std::path::Path;

const ROOT: &str = "c_lib";

fn build_lua_libs() {
    let root = Path::new(ROOT);
    let mut lua_build = cc::Build::new();
    lua_build.include(root.join("lua"));

    let lbc = root.join("lbc");
    lua_build
        .clone()
        .file(lbc.join("lbc.c"))
        .file(lbc.join("src").join("number.c"))
        .include(lbc.join("src"))
        .include(lbc)
        .compile("lbc");

    let lpeg = root.join("lpeg");
    lua_build
        .clone()
        .warnings(false)
        .file(lpeg.join("lpcap.c"))
        .file(lpeg.join("lpcode.c"))
        .file(lpeg.join("lpprint.c"))
        .file(lpeg.join("lptree.c"))
        .file(lpeg.join("lpvm.c"))
        .include(lpeg)
        .compile("lpeg");

    let lrexlib = root.join("lrexlib").join("src");
    lua_build
        .clone()
        .define("PCRE2_CODE_UNIT_WIDTH", "8")
        .file(lrexlib.join("common.c"))
        .file(lrexlib.join("pcre2").join("lpcre2_f.c"))
        .file(lrexlib.join("pcre2").join("lpcre2.c"))
        .include(lrexlib)
        .include(root.join("pcre2-mirror").join("src"))
        .compile("lrexlib");

    let lsqlite3 = root.join("lsqlite3");
    lua_build
        .clone()
        .warnings(false)
        .file(lsqlite3.join("lsqlite3.c"))
        .file(lsqlite3.join("extras").join("extension-functions.c"))
        .include(lsqlite3)
        .include(root.join("sqlite-amalgamation"))
        .compile("lsqlite3");
}

fn main() {
    build_lua_libs();
    qt_ritual_build::add_resources(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/resources.qrc"
    ));
}
