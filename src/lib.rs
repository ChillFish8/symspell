/*!

Spelling correction & Fuzzy search based on Symmetric Delete spelling correction algorithm.

# Basic Example

```
use symspell::{SymSpell, AsciiStringStrategy, Verbosity};

let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");

let suggestions = symspell.lookup("roket", Verbosity::Top, 2);
println!("{:?}", suggestions);

let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him";
let compound_suggestions = symspell.lookup_compound(sentence, 2);
println!("{:?}", compound_suggestions);
```
*/
#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate log;

mod composition;
mod edit_distance;
mod string_strategy;
mod suggestion;
mod symspell;

#[cfg(not(target_arch = "wasm32"))]
pub use string_strategy::AsciiStringStrategy;
pub use string_strategy::{StringStrategy, UnicodeStringStrategy, UnicodeiStringStrategy};
pub use suggestion::Suggestion;
pub use symspell::{SymSpell, SymSpellBuilder, Verbosity};
