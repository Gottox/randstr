use rand::seq::SliceRandom;

static UPPER_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/upper.txt"));
static LOWER_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/lower.txt"));
static DIGIT_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/digit.txt"));
static SYMBOL_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/symbol.txt"));
static WHITESPACE_ALPHABET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/whitespace.txt"));

/// Builder for generating random strings.
pub struct RandStrBuilder {
    upper: Option<&'static str>,
    lower: Option<&'static str>,
    digit: Option<&'static str>,
    symbol: Option<&'static str>,
    whitespace: Option<&'static str>,
    custom: Option<String>,

    must_upper: bool,
    must_lower: bool,
    must_digit: bool,
    must_symbol: bool,
    must_whitespace: bool,
    must_custom: bool,

    rng: Option<rand::rngs::ThreadRng>,

    len: usize,
}

impl RandStrBuilder {
    fn new() -> Self {
        RandStrBuilder {
            upper: None,
            lower: None,
            digit: None,
            symbol: None,
            whitespace: None,
            custom: None,
            must_upper: false,
            must_lower: false,
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
        self.upper = Some(UPPER_ALPHABET);
        self
    }
    /// Allows the generator to produce lowercase letters.
    pub fn lower(&mut self) -> &mut Self {
        self.lower = Some(LOWER_ALPHABET);
        self
    }
    /// Allows the product to produce uppercase and lowercase letters.
    pub fn letter(&mut self) -> &mut Self {
        self.upper().lower()
    }
    /// Allows the generator to produce whitespaces.
    pub fn whitespace(&mut self) -> &mut Self {
        self.whitespace = Some(WHITESPACE_ALPHABET);
        self
    }
    /// Allows the generator to produce digits.
    pub fn digit(&mut self) -> &mut Self {
        self.digit = Some(DIGIT_ALPHABET);
        self
    }
    /// Allows the generator to produce symbols.
    pub fn symbol(&mut self) -> &mut Self {
        self.symbol = Some(SYMBOL_ALPHABET);
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
    /// Forces the generator to produce strings containing uppercase and lowercase letters.
    pub fn must_upper_lower(&mut self) -> &mut Self {
        self.must_upper().must_lower()
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
    /// Forces the generator to produce strings containing symbols.
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
        let options = self;

        let custom = options.custom.as_deref();
        let alphabet: Vec<_> = [
            options.upper,
            options.lower,
            options.digit,
            options.symbol,
            options.whitespace,
            custom,
        ]
        .into_iter()
        .flatten()
        .flat_map(|a| a.as_bytes().iter())
        .cloned()
        .collect();

        if alphabet.is_empty() {
            panic!("No alphabet specified");
        }

        let must_alphabets: Vec<_> = [
            options
                .must_upper
                .then(|| options.upper.expect("upper alphabet is not set")),
            options
                .must_lower
                .then(|| options.lower.expect("lower alphabet is not set")),
            options
                .must_lower
                .then(|| options.lower.expect("lower alphabet is not set")),
            options
                .must_digit
                .then(|| options.digit.expect("digit alphabet is not set")),
            options.must_whitespace.then(|| {
                options.whitespace.expect("whitespace alphabet is not set")
            }),
            options
                .must_symbol
                .then(|| options.symbol.expect("symbol alphabet is not set")),
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
            panic!("Length is too short to contain all mandatory alphabets");
        }

        let rng = options.rng.clone().unwrap_or_else(rand::thread_rng);

        RandStr {
            alphabet,
            must_alphabets,
            len,
            rng,
        }
    }
}

pub struct RandStr {
    alphabet: Vec<u8>,
    must_alphabets: Vec<Vec<u8>>,
    len: usize,
    rng: rand::rngs::ThreadRng,
}

impl RandStr {
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
                break;
            }
        }

        if rearrange {
            result = result.rchunks(self.len).next().unwrap().to_vec();
            result.shuffle(&mut self.rng);
        }

        String::from_utf8(result).unwrap()
    }
}

pub fn randstr() -> RandStrBuilder {
    RandStrBuilder::new()
}