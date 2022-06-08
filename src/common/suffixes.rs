const SUFFIX_DARK: &str = "_dark";
const SUFFIX_LIGHT: &str = "_light";

pub trait SuffixExt {
    fn light(&self) -> bool;
    fn dark(&self) -> bool;
    fn with_light_suffix(&self) -> Self;
    fn with_dark_suffix(&self) -> Self;
}

impl SuffixExt for String {
    // Returns `true` if the string has suffix `_light`.
    fn light(&self) -> bool {
        self.ends_with("_light")
    }

    // Returns `true` if the string has suffix `_dark`.
    fn dark(&self) -> bool {
        self.ends_with("_dark")
    }

    fn with_light_suffix(&self) -> Self {
        format!("{}{}", self, SUFFIX_LIGHT)
    }

    fn with_dark_suffix(&self) -> Self {
        format!("{}{}", self, SUFFIX_DARK)
    }
}
