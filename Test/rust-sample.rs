

/// Available encoding character sets
pub enum CharacterSet
{
    /// The standard character set (uses `+` and `/`)
    Standard,
    /// The URL safe character set (uses `-` and `_`)
    UrlSafe
}

/// Contains configuration parameters for `to_base64`.
pub struct Config
{
    /// Character set to use
    priv char_set: CharacterSet,
    /// True to pad output with `=` characters
    priv pad: bool,
    /// `Some(len)` to wrap lines at `len`, `None` to disable line wrapping
    priv line_length: Option<uint>
}


/// Configuration for RFC 4648 standard base64 encoding
pub static STANDARD: Config = Config {char_set: Standard, pad: true, line_length: None};

/// A trait for converting a value to base64 encoding.
pub trait ToBase64
{
    /// Converts the value of `self` to a base64 value following the specified
    /// format configuration, returning the owned string.
    fn to_base64(&self, config: Config) -> ~str;
}


mod test
{
    use test::BenchHarness;
    use base64::*;

    #[test]
    fn test_to_base64_basic() {
        assert_eq!("".as_bytes().to_base64(STANDARD), ~"");
        assert_eq!("f".as_bytes().to_base64(STANDARD), ~"Zg==");
        assert_eq!("fo".as_bytes().to_base64(STANDARD), ~"Zm8=");
        assert_eq!("foo".as_bytes().to_base64(STANDARD), ~"Zm9v");
        assert_eq!("foob".as_bytes().to_base64(STANDARD), ~"Zm9vYg==");
        assert_eq!("fooba".as_bytes().to_base64(STANDARD), ~"Zm9vYmE=");
        assert_eq!("foobar".as_bytes().to_base64(STANDARD), ~"Zm9vYmFy");
    }
}


impl<'self> ToBase64 for &'self [u8] {
    /**
     * Turn a vector of `u8` bytes into a base64 string.
     *
     * # Example
     *
     * ```rust
     * extern mod extra;
     * use extra::base64::{ToBase64, STANDARD};
     *
     * fn main () {
     *     let str = [52,32].to_base64(STANDARD);
     *     println!("base 64 output: {}", str);
     * }
     * ```
     */
    fn to_base64(&self, config: Config) -> ~str {
        let bytes = match config.char_set {
            Standard => STANDARD_CHARS,
            UrlSafe => URLSAFE_CHARS
        };

        let mut v: ~[u8] = ~[];
    }
}

fn main()
{
    let nums = [1, 2];
    let noms = ["Tim", "Eston", "Aaron", "Ben"];

    let mut odds = nums.iter().map(|&x| x * 2 - 1);

    for num in odds
    {
        do spawn
        {
            println!("{:s} says hello from a lightweight thread!", noms[num]);
        }
    }
}
