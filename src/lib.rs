/*!
 */

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::HashMap;
use std::fmt;

#[macro_use]
extern crate lazy_static;

//import re
//import tokenizer

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

lazy_static! {
    static ref UNITS: HashMap<&'static str, &'static str> = {
        hashmap![
            "cups" => "cup",
            "tablespoons" => "tablespoon",
            "teaspoons" => "teaspoon",
            "pounds"=> "pound",
            "ounces"=> "ounce",
            "cloves"=> "clove",
            "sprigs"=> "sprig",
            "pinches"=> "pinch",
            "bunches"=> "bunch",
            "slices"=> "slice",
            "grams"=> "gram",
            "heads"=> "head",
            "quarts"=> "quart",
            "stalks"=> "stalk",
            "pints"=> "pint",
            "pieces"=> "piece",
            "sticks"=> "stick",
            "dashes"=> "dash",
            "fillets"=> "fillet",
            "cans"=> "can",
            "ears"=> "ear",
            "packages"=> "package",
            "strips"=> "strip",
            "bulbs"=> "bulb",
            "bottles"=> "bottle"
        ]
    };
}

lazy_static! {
    static ref FRACTIONS: HashMap<&'static str, &'static str> = {
        hashmap![
        "\x215b" => " 1/8",
        "\x215c" => " 3/8",
        "\x215d" => " 5/8",
        "\x215e" => " 7/8",
        "\x2159" => " 1/6",
        "\x215a" => " 5/6",
        "\x2155" => " 1/5",
        "\x2156" => " 2/5",
        "\x2157" => " 3/5",
        "\x2158" => " 4/5",
        "\x00bc" => " 1/4",
        "\x00be" => " 3/4",
        "\x2153" => " 1/3",
        "\x2154" => " 2/3",
        "\x00bd" => " 1/2"
        ]
    };
}

/// Replace unicode fractions with ascii representation, preceded by a
/// space.
///
/// # Example
///
/// ```
/// # use ingredient_phrase_tagger::*;
/// assert_eq!(cleanUnicodeFractions("1\x215e"), "1 7/8");
/// ```
// TODO(markcol): replace &str with cow
pub fn cleanUnicodeFractions(s: &str) -> String {
    let mut out: String = s.to_string();
    for (f_unicode, f_ascii) in FRACTIONS.iter() {
        out = out.replace(f_unicode, f_ascii)
    }
    out.to_string()
}

/// Replaces $'s with spaces. The reverse of `clumpFractions`.
///
/// # Example
///
/// ```
/// # use ingredient_phrase_tagger::*;
/// assert_eq!(unclump("aaa 1$2/3 bbb"), "aaa 1 2/3 bbb");
/// ```
// TODO(markcol): replace &str with cow
pub fn unclump(s: &str) -> String {
    s.replace("$", " ")
}

// TODO(markcol): FIX THIS. We used to use the pattern.en package to singularize words, but
// in the name of simple deployments, we took it out. We should fix this at some
// point.
pub fn normalizeToken(s: &str) -> &str {
    singularize(s)
}

///  A poor replacement for the pattern.en singularize function, but ok for now.
///
/// #Examples
///
/// ```
/// # use ingredient_phrase_tagger::*;
/// assert_eq!(singularize("cups"), "cup");
/// assert_eq!(singularize("dashes"), "dash");
/// assert_eq!(singularize("ounces"), "ounce");
/// ```
pub fn singularize(word: &str) -> &str {
    if UNITS.contains_key(word) {
        UNITS[word]
    } else {
        word
    }
}

/// Joins list of words with spaces, but is smart about not adding spaces
/// before commas.
///
/// ```
/// # use ingredient_phrase_tagger::*;
/// assert_eq!(
///     smartJoin(&["aaa", "(", "foo", ",", "bar", ")", "baz"]),
///     "aaa (foo, bar) baz");
/// ```
// HACK(markcol): fix this
pub fn smartJoin(words: &[&str]) -> String {
    words
        .join(" ")
        .replace(" , ", ", ")
        .replace("( ", "(")
        .replace(" )", ")")
        .to_string()
}

/// Buckets the length of the ingredient into 6 buckets.
///
/// ```
/// # use ingredient_phrase_tagger::*;
/// assert_eq!(lengthGroup(1), "4");
/// assert_eq!(lengthGroup(7), "8");
/// assert_eq!(lengthGroup(12), "16");
/// assert_eq!(lengthGroup(21), "X");
/// ```
// TODO(markcol): replace return of "X" with Option<String> to signal
// no replacement. Requires changes at all call sites.
pub fn lengthGroup(actualLength: i32) -> String {
    for n in [4, 8, 12, 16, 20].iter() {
        if actualLength < *n {
            return n.to_string();
        }
    }
    return "X".to_string();
}

/// Returns true if the word is inside parenthesis in the phrase.
// pub fn isCapitalized(token: &str) -> bool {
//   return re.match(r'^[A-Z]', token) is not None
// }

/// Parse 'raw' ingredients lines into CRF-ready output.
// pub fn export_data(lines: &[&str]) -> &Vec<String> {
//     let mut output: Vec<String> = Vec::new();
//     for line in lines {
//         let line_clean = re.sub('<[^<]+?>', '', line);
//         let tokens = tokenizer.tokenize(line_clean);
//
//         for i, token in tokens.iter() {
//             let features = getFeatures(token, i + 1, tokens);
//             output.append(joinLine([token] + features));
//         }
//         output.append('\n');
//     }
//     output
// }

/// Returns true if the word is inside parenthesis in the phrase.
// pub fn insideParenthesis(token: &str, tokens: &[&str]) -> bool {
//    if token in ['(', ')'] {
//        return true;
//    }
//
//    let line = " ".join(tokens)
//    return re.match(r'.*(.*' + re.escape(token) + '.*).*', line) is not None
//}

/// Format a list of (tag, [tokens]) tuples as an HTML string for display.
///
/// ```nocompile
/// assert_eq!(
///   displayIngredient([("qty", ["1"]), ("name", ["cat", "pie"])]),
///    "<span class='qty'>1</span> <span class='name'>cat pie</span>");
/// ```
// pub fn displayIngredient(ingredient: &[&str]) -> &str {
//     return "".join([
//         format!("<span class='{}'>{}</span>", tag, tokens.join(" "))
//         for tag, tokens in ingredient
//     ])
// }

// pub fn joinLine(columns: &[&str]) -> &str {
//  columns.join('\t')
// }

/// Replaces the whitespace between the integer and fractional part of a quantity
/// with a dollar sign, so it's interpreted as a single token. The rest of the
/// string is left alone.
///
/// # Example
///
/// ```nocompile
/// # use ingredient_phrase_tagger::*;
/// assert_eq!(clumpFractions("aaa 1 2/3 bbb"), "aaa 1$2/3 bbb");
/// ```
//  pub fn clumpFractions(s: &str) -> &str {
//     re.sub(r"(\d+)\s+(\d)/(\d)", r"\1$\2/\3", s);
// }

/// Returns a list of features for a given token.
// pub fn getFeatures(token, index, tokens) {
//  let length = tokens.len();
//
//  return [("I%s" % index), ("L%s" % lengthGroup(length)),
//             ("Yes" if isCapitalized(token) else "No") + "CAP",
//             ("Yes" if insideParenthesis(token, tokens) else "No") + "PAREN"]
// }

/// iterate lines in the data file, which looks like:
///
///   # 0.511035
///   1/2       I1  L12  NoCAP  X  B-QTY/0.982850
///   teaspoon  I2  L12  NoCAP  X  B-UNIT/0.982200
///   fresh     I3  L12  NoCAP  X  B-COMMENT/0.716364
///   thyme     I4  L12  NoCAP  X  B-NAME/0.816803
///   leaves    I5  L12  NoCAP  X  I-NAME/0.960524
///   ,         I6  L12  NoCAP  X  B-COMMENT/0.772231
///   finely    I7  L12  NoCAP  X  I-COMMENT/0.825956
///   chopped   I8  L12  NoCAP  X  I-COMMENT/0.893379
///
///   # 0.505999
///   Black   I1  L8  YesCAP  X  B-NAME/0.765461
///   pepper  I2  L8  NoCAP   X  I-NAME/0.756614
///   ,       I3  L8  NoCAP   X  OTHER/0.798040
///   to      I4  L8  NoCAP   X  B-COMMENT/0.683089
///   taste   I5  L8  NoCAP   X  I-COMMENT/0.848617
///
/// i.e. the output of crf_test -v 1
///
/// This thing takes the output of CRF++ and turns it into an actual
/// data structure.
/* pub fn import_data(lines: &[&str]) -> Vec<String> {
    let data = [{}]
    let display = [[]]
    let prevTag = None
    for line in lines {
        // blank line starts a new ingredient
        if line in ('', '\n') {
            data.append({})
            display.append([])
            prevTag = None
        } else if line[0] == '#' {
            // ignore comments
            continue;
        } else {
            // otherwise it's a token
            // e.g.: potato \t I2 \t L5 \t NoCAP \t B-NAME/0.978253
            columns = re.split('\t', line.strip())
            token = columns[0].strip()

            // unclump fractions
            token = unclump(token)

            // turn B-NAME/123 back into "name"
            tag, confidence = re.split(r'/', columns[-1], 1)
            tag = re.sub('^[BI]\-', "", tag).lower()

            // ---- DISPLAY ----
            // build a structure which groups each token by its tag, so we can
            // rebuild the original display name later.

            if prevTag != tag {
                display[-1].append((tag, [token]))
                prevTag = tag
            } else {
                display[-1][-1][1].append(token)
                //              ^- token
                //           ^---- tag
                //       ^-------- ingredient
            }

            // ---- DATA ----
            // build a dict grouping tokens by their tag

            // initialize this attribute if this is the first token of its kind
            if tag not in data[-1] {
                data[-1][tag] = []
            }

            // HACK: If this token is a unit, singularize it so Scoop accepts it.
            if tag == "unit" {
                token = singularize(token);
            }

            data[-1][tag].append(token);
    }

    // reassemble the output into a list of dicts.
    output = [
        dict([(k, smartJoin(tokens))
              for k, tokens in ingredient.iteritems()])
        for ingredient in data
        if len(ingredient)
    ]
    // Add the marked-up display data
    for i, v in output.iter() {
        output[i]["display"] = displayIngredient(display[i]);
    }

    // Add the raw ingredient phrase
    for i, v in enumerate(output).iter() {
        output[i]["input"] = smartJoin([" ".join(tokens) for k, tokens in display[i]])
    }
    output
} 
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
