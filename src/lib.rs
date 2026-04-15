extern crate pycld2_sys as ffi;

use libc::{c_double, c_int};
use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};

/// A language code, normally two letters for common languages.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Language {
    language_id: ffi::Language,
}

/// Options controlling Decoder behavior.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Options {
    /// A possible language for the text.
    pub language_hint: Option<Language>,

    /// Assume the input is HTML and skip tags.
    pub html: bool,

    /// Allow detection from very short fragments.
    pub best_efforts: bool,
}

/// Detailed information about how well the input text matched a specific
/// language.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct LanguageScore {
    /// The language matched.
    pub language: Option<Language>,

    /// The percentage of the text which appears to be in this language.
    /// Between 0 and 100.
    pub percent: u8,

    /// Scores near 1.0 indicate a "normal" text for this language.  Scores
    /// further away from 1.0 indicate strange or atypical texts.
    pub normalized_score: f64,
}

/// Detailed language detection results.
///
/// Note: Do not rely on this struct containing only the fields listed
/// below.  It may gain extra fields in the future.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct DetectionResult {
    /// The language detected.
    pub language: Option<Language>,

    /// The scores for the top 3 candidate languages.
    pub scores: [LanguageScore; 3],

    /// The number of bytes of actual text found, excluding punctuation, tags,
    /// etc.
    pub text_bytes: usize,

    /// Is this guess reliable?
    pub reliable: bool,
}

/// Detect the language of the input text and return detailed statistics.
///
/// ```
/// use std::default::Default;
/// use pycld2::{Language, Options, detect_language};
///
/// let text = "Sur le pont d'Avignon,
/// L'on y danse, l'on y danse,
/// Sur le pont d'Avignon
/// L'on y danse tous en rond.
///
/// Les belles dames font comme ça
/// Et puis encore comme ça.
/// Les messieurs font comme ça
/// Et puis encore comme ça.";
///
/// let detected = detect_language(text, &Default::default());
/// assert_eq!(Language::new("fr"), detected.language);
/// assert_eq!(true, detected.reliable);
/// assert_eq!(text.len(), detected.text_bytes + 5);
/// assert_eq!(Language::new("fr"), detected.scores[0].language);
/// assert_eq!(99, detected.scores[0].percent);
/// assert_eq!(None, detected.scores[1].language);
/// assert_eq!(None, detected.scores[2].language);
///
/// let detected = detect_language("test", &Default::default());
/// assert_eq!(None, detected.language);
/// assert_eq!(false, detected.reliable);
///
/// let mut options = Options::default();
/// options.best_efforts = true;
/// let detected = detect_language("test", &options);
/// assert_eq!(Language::new("en"), detected.language);
/// assert_eq!(true, detected.reliable);
/// assert_eq!(83, detected.scores[0].percent);
/// assert_eq!(None, detected.scores[1].language);
/// assert_eq!(None, detected.scores[2].language);
/// ```
pub fn detect_language(text: &str, options: &Options) -> DetectionResult {
    let language_hint = options
        .language_hint
        .map(|language| language.language_id)
        .unwrap_or(ffi::Language::UNKNOWN_LANGUAGE);
    let hints = ffi::CLDHints {
        content_language_hint: null(),
        tld_hint: null(),
        encoding_hint: ffi::Encoding::UNKNOWN_ENCODING as c_int,
        language_hint,
    };

    let flags = if options.best_efforts {
        ffi::kCLDFlagBestEffort
    } else {
        0
    };

    // Out-parameters passed as pointers
    let mut language3 = [
        ffi::Language::UNKNOWN_LANGUAGE,
        ffi::Language::UNKNOWN_LANGUAGE,
        ffi::Language::UNKNOWN_LANGUAGE,
    ];
    let mut percent3: [c_int; 3] = [0, 0, 0];
    let mut normalized_score3: [c_double; 3] = [0.0, 0.0, 0.0];
    let mut text_bytes: c_int = 0;
    let mut reliable: bool = false;

    let language_id = unsafe {
        ffi::CLD2_ExtDetectLanguageSummary4(
            text.as_ptr() as *const i8,
            text.len() as c_int,
            !options.html,
            &hints,
            flags,
            language3.as_mut_ptr(),
            percent3.as_mut_ptr(),
            normalized_score3.as_mut_ptr(),
            null_mut(),
            &mut text_bytes,
            &mut reliable,
        )
    };
    let score_n = |n: usize| LanguageScore {
        language: Language::from_id(language3[n]),
        percent: percent3[n] as u8,
        normalized_score: normalized_score3[n],
    };
    DetectionResult {
        language: Language::from_id(language_id),
        scores: [score_n(0), score_n(1), score_n(2)],
        text_bytes: text_bytes as usize,
        reliable,
    }
}

impl Language {
    pub fn new(language_code: &str) -> Option<Self> {
        let c_name = CString::new(language_code.as_bytes()).unwrap();
        let language_id = unsafe { ffi::CLD2_GetLanguageFromName(c_name.as_ptr()) };
        Self::from_id(language_id)
    }

    fn from_id(language_id: ffi::Language) -> Option<Self> {
        if language_id == ffi::Language::UNKNOWN_LANGUAGE {
            return None;
        }
        Some(Self { language_id })
    }

    pub fn code(&self) -> &'static str {
        // Safety: CLD2_LanguageCode return a pointer into a static string.
        let c_str = unsafe { CStr::from_ptr(ffi::CLD2_LanguageCode(self.language_id)) };
        // code is always ASCII
        std::str::from_utf8(c_str.to_bytes()).unwrap()
    }
}

#[test]
fn test_language_conversion() {
    let language = Language::new("en");
    assert_eq!(language, Language::from_id(ffi::Language::ENGLISH));
    assert_eq!("en", language.unwrap().code());
    assert_eq!(None, Language::new("bla-bla-bla"));
    assert_eq!(None, Language::new(""));
    assert_eq!(None, Language::from_id(ffi::Language::UNKNOWN_LANGUAGE));
}
