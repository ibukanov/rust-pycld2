# Rust cld2 like Python pycld2

This is a fork of [rust-cld2](https://github.com/emk/rust-cld2) Rust bindings
for [cld2](https://github.com/CLD2Owners/cld2) C++ library that includes patched
cld2 sources from [pycld2](https://github.com/aboSamoor/pycld2) Python bindings
for cld2 instead of the original sources to support detecting 165 languages.

Compared with the original rust-cld2 it exposes minimal API sufficient to
implement what is available via polyglot.detect.Detector Python class including
support best efforts option that allows for the detector to work on short
strings.

## Example

Example of language detection that uses best efforts and return some language
even when the detection is unreliable:

```rust
fn detect_language(text: &str) -> Option<&'static str> {
    let mut options = Default::default();
    let mut detection_result = pycld2::detect_language(text, &options);
    if !detection_result.reliable {
        options.best_efforts = true;
        detection_result = pycld2::detect_language(text, &options);
    }
    detection_result.language.map(|l| l.code())
}
```

## License

The original cld2 library is distributed under the Apache License Version
2.0.  This also covers much of the code in `cld2-sys/src/wrapper.h`.  All
of the new code is released into the public domain as described by the
Unlicense.
