#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::{error, fmt};

use rand::seq::SliceRandom;

static UPPER_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/upper.txt"));
static LOWER_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/lower.txt"));
static LETTER_ALPHABET: &str = concat!(
    include_str!(concat!(env!("OUT_DIR"), "/lower.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/upper.txt")),
);
static DIGIT_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/digit.txt"));
static SYMBOL_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/symbol.txt"));
static WHITESPACE_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/whitespace.txt"));

/// Error Type of randstr
#[derive(Debug)]
pub enum Error {
    /// Error when no alphabet is specified before calling `try_build`
    NoAlphabet,
    /// Error when the generated string is too short to contain all mandatory characters.
    TooShort,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoAlphabet => write!(f, "No alphabet specified"),
            Error::TooShort => write!(
                f,
                "Length is too short to contain all mandatory alphabets"
            ),
        }
    }
}

/// Builder for generating random strings.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct RandStrBuilder {
    upper: bool,
    lower: bool,
    letter: bool,
    digit: bool,
    symbol: bool,
    whitespace: bool,
    custom: Option<String>,

    must_upper: bool,
    must_lower: bool,
    must_letter: bool,
    must_digit: bool,
    must_symbol: bool,
    must_whitespace: bool,
    must_custom: bool,

    len: usize,

    #[cfg_attr(feature = "serde", serde(skip))]
    rng: Option<rand::rngs::ThreadRng>,
}

impl RandStrBuilder {
    fn new() -> Self {
        RandStrBuilder {
            upper: false,
            lower: false,
            letter: false,
            digit: false,
            symbol: false,
            whitespace: false,
            custom: None,
            must_upper: false,
            must_lower: false,
            must_letter: false,
            must_digit: false,
            must_symbol: false,
            must_whitespace: false,
            must_custom: false,
            rng: None,
            len: 0,
        }
    }
    /// Allows the generator to produce uppercase letters.
    pub fn upper(&mut self) -> &mut Self {
        self.upper = true;
        self
    }
    /// Allows the generator to produce lowercase letters.
    pub fn lower(&mut self) -> &mut Self {
        self.lower = true;
        self
    }
    /// Allows the product to produce uppercase and lowercase letters.
    pub fn letter(&mut self) -> &mut Self {
        self.letter = true;
        self
    }
    /// Allows the generator to produce whitespaces.
    pub fn whitespace(&mut self) -> &mut Self {
        self.whitespace = true;
        self
    }
    /// Allows the generator to produce digits.
    pub fn digit(&mut self) -> &mut Self {
        self.digit = true;
        self
    }
    /// Allows the generator to produce symbols.
    pub fn symbol(&mut self) -> &mut Self {
        self.symbol = true;
        self
    }
    /// Allows the generator to produce custom characters.
    pub fn custom(&mut self, custom: &str) -> &mut Self {
        self.custom = Some(custom.to_string());
        self
    }
    /// Allows the generator to produce all characters.
    pub fn all(&mut self) -> &mut Self {
        self.letter().digit().symbol()
    }
    /// Forces the generator to produce strings containing uppercase letters.
    pub fn must_upper(&mut self) -> &mut Self {
        self.must_upper = true;
        self.upper()
    }
    /// Forces the generator to produce strings containing lowercase letters.
    pub fn must_lower(&mut self) -> &mut Self {
        self.must_lower = true;
        self.lower()
    }
    /// Forces the generator to produce strings containing letters.
    pub fn must_letter(&mut self) -> &mut Self {
        self.must_letter = true;
        self.letter()
    }
    /// Forces the generator to produce strings containing digits.
    pub fn must_digit(&mut self) -> &mut Self {
        self.must_digit = true;
        self.digit()
    }
    /// Forces the generator to produce strings containing symbols.
    pub fn must_symbol(&mut self) -> &mut Self {
        self.must_symbol = true;
        self.symbol()
    }
    /// Forces the generator to produce strings containing whitespaces.
    pub fn must_whitespace(&mut self) -> &mut Self {
        self.must_whitespace = true;
        self.symbol()
    }
    /// Forces the generator to produce strings containing custom characters.
    pub fn must_custom(&mut self, custom: &str) -> &mut Self {
        self.must_custom = true;
        self.custom(custom)
    }
    /// Sets the random number generator to use.
    pub fn rng(&mut self, rng: rand::rngs::ThreadRng) -> &mut Self {
        self.rng = Some(rng);
        self
    }
    /// Sets the length of the generated string.
    pub fn len(&mut self, len: usize) -> &mut Self {
        self.len = len;
        self
    }
    /// Builds the random string producer.
    pub fn build(&self) -> RandStr {
        self.try_build().unwrap()
    }
    /// Builds the random string producer. Returns an error if the builder cannot be built.
    pub fn try_build(&self) -> Result<RandStr, Error> {
        let options = self;

        let custom = options.custom.as_deref();
        let mut alphabet: Vec<_> = [
            options.upper.then_some(UPPER_ALPHABET),
            options.lower.then_some(LOWER_ALPHABET),
            options.letter.then_some(LETTER_ALPHABET),
            options.digit.then_some(DIGIT_ALPHABET),
            options.symbol.then_some(SYMBOL_ALPHABET),
            options.whitespace.then_some(WHITESPACE_ALPHABET),
            custom,
        ]
        .into_iter()
        .flatten()
        .flat_map(|a| a.as_bytes().iter())
        .cloned()
        .collect();

        if alphabet.is_empty() {
            Err(Error::NoAlphabet)?;
        }

        alphabet.sort_unstable();
        alphabet.dedup();

        let must_alphabets: Vec<_> = [
            options.must_upper.then_some(UPPER_ALPHABET),
            options.must_lower.then_some(LOWER_ALPHABET),
            options.must_letter.then_some(LETTER_ALPHABET),
            options.must_digit.then_some(DIGIT_ALPHABET),
            options.must_whitespace.then_some(WHITESPACE_ALPHABET),
            options.must_symbol.then_some(SYMBOL_ALPHABET),
            options
                .must_custom
                .then(|| custom.expect("symbol alphabet is not set")),
        ]
        .into_iter()
        .flatten()
        .map(|a| a.as_bytes().to_vec())
        .collect();

        let len = options.len;
        if len < must_alphabets.len() {
            Err(Error::TooShort)?;
        }

        let rng = options.rng.clone().unwrap_or_else(rand::thread_rng);

        Ok(RandStr {
            alphabet,
            must_alphabets,
            len,
            rng,
        })
    }
}

/// The generator to produce new random strings.
pub struct RandStr {
    alphabet: Vec<u8>,
    must_alphabets: Vec<Vec<u8>>,
    len: usize,
    rng: rand::rngs::ThreadRng,
}

impl RandStr {
    /// Generates a new random string.
    pub fn generate(&mut self) -> String {
        let capacity = self.len + self.must_alphabets.len();

        let mut result = Vec::with_capacity(capacity);
        for _ in 0..self.len {
            result.push(*self.alphabet.choose(&mut self.rng).unwrap());
        }
        let mut rearrange = false;
        for alpha in &self.must_alphabets {
            if !alpha.iter().any(|&a| result.contains(&a)) {
                rearrange = true;
                result.push(*alpha.choose(&mut self.rng).unwrap());
            }
        }

        if rearrange {
            result = result.rchunks(self.len).next().unwrap().to_vec();
            result.shuffle(&mut self.rng);
        }

        String::from_utf8(result).unwrap()
    }
}

/// Creates a new RandStrBuilder.
///
/// # Examples
///
/// ```rust
/// use randstr::randstr;
///
/// let mut generator = randstr()
///   .len(10)
///   .all()
///   .build();
/// println!("{}", generator.generate())
/// ```
pub fn randstr() -> RandStrBuilder {
    RandStrBuilder::new()
}
