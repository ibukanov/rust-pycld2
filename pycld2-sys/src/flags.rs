use libc::c_int;

pub const kCLDFlagScoreAsQuads: c_int = 0x0100;
pub const kCLDFlagHtml: c_int = 0x0200;
pub const kCLDFlagCr: c_int = 0x0400;
pub const kCLDFlagVerbose: c_int = 0x0800;
pub const kCLDFlagQuiet: c_int = 0x1000;
pub const kCLDFlagEcho: c_int = 0x2000;
pub const kCLDFlagBestEffort: c_int = 0x4000;
