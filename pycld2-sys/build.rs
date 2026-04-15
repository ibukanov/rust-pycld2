use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&manifest_dir);
    // Include only what pycld2 includes, see
    // https://github.com/aboSamoor/pycld2/blob/bc9d269f603dc79d4a6e64887065839e62ef3dde/setup.py#L25
    let internals = src.join("cld2").join("internal");
    let mut sources = [
        "cld2_generated_cjk_compatible.cc",
        "cld2_generated_deltaocta0122.cc",
        "cld2_generated_distinctocta0122.cc",
        "cld2_generated_quad0122.cc",
        "cld_generated_cjk_delta_bi_32.cc",
        "cld_generated_cjk_uni_prop_80.cc",
        "cld_generated_score_quad_octa_0122.cc",
        "cldutil.cc",
        "cldutil_shared.cc",
        "compact_lang_det.cc",
        "compact_lang_det_hint_code.cc",
        "compact_lang_det_impl.cc",
        "debug.cc",
        "fixunicodevalue.cc",
        "generated_distinct_bi_0.cc",
        "generated_entities.cc",
        "generated_language.cc",
        "generated_ulscript.cc",
        "getonescriptspan.cc",
        "lang_script.cc",
        "offsetmap.cc",
        "scoreonescriptspan.cc",
        "tote.cc",
        "utf8statetable.cc",
    ]
    .into_iter()
    .map(|name| internals.join(name))
    .collect::<Vec<_>>();
    sources.push(src.join("src").join("wrapper.cpp"));

    // Run the build.
    let mut build = cc::Build::new();
    build.cpp(true);
    let cxxflags = match env::var("CXXFLAGS") {
        Ok(val) => val + " -std=c++03",
        Err(..) => String::from("-std=c++03"),
    };
    env::set_var("CXXFLAGS", &cxxflags);
    build.include(Path::new("cld2/public"));
    build.include(Path::new("cld2/internal"));
    for f in sources.iter() {
        build.file(f);
    }
    build.compile("libpycld2.a");
}
