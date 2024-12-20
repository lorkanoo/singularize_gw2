use regex::{Match, Regex};

const UNCOUNTABLE: &[&'static str] = &[
    "equipment",
    "information",
    "rice",
    "money",
    "species",
    "series",
    "fish",
    "sheep",
    "jeans",
    "police",
    "milk",
    "salt",
    "time",
    "water",
    "paper",
    "food",
    "art",
    "cash",
    "music",
    "help",
    "luck",
    "news",
    "oil",
    "progress",
    "rain",
    "research",
    "shopping",
    "software",
    "traffic",
];
const IRREGULAR: &[(&'static str, &'static str)] = &[
    ("man", "men"),
    ("child", "children"),
    ("sex", "sexes"),
    ("ombie", "ombies"),
    ("goose", "geese"),
    ("moose", "moose"),
    ("foot", "feet"),
    ("tooth", "teeth"),
    ("mouse", "mice"),
    ("thief", "thieves"),
    //
    ("alias", "aliases"),
    ("status", "statuses"),
    ("campus", "campuses"),
    ("bus", "buses"),
    ("oaf", "oaves"),
    ("leaf", "leaves"),
    ("nife", "nives"),
    ("life", "lives"),
    ("wife", "wives"),
    //
    ("osis", "oses"),
    ("psis", "pses"),
];
const WORDS: &[(&'static str, &'static str)] = &[("testis", "testes"), ("ox", "oxen")];
const ENDINGS: &[(&'static str, &'static str)] = &[
    ("vertex", "vertices"),
    ("matrix", "matrices"),
    ("index", "indices"),
    ("buffalo", "buffaloes"),
    ("quiz", "quizzes"),
    ("potato", "potatoes"),
    ("hero", "heroes"),
    ("ango", "angoes"),
    //
    ("x", "xes"),
    ("ch", "ches"),
    ("ss", "sses"),
    ("sh", "shes"),
    //
    ("ay", "ays"),
    ("ey", "eys"),
    ("iy", "iys"),
    ("oy", "oys"),
    ("uy", "uys"),
    ("y", "ies"),
    //
    ("ffe", "ffes"),
    // ("fe", "ves"),
    ("lf", "lves"),
    //
    ("'s", "'s"),
    ("s", "ses"),
    //
    ("rse", "rses"),
    ("use", "uses"),
];

fn word_match(w: &str, end: &str) -> bool {
    if w == end {
        return true;
    }
    if let Some(prefix) = w.strip_suffix(end) {
        match *prefix.as_bytes().last().unwrap() {
            b'-' | b' ' | b'.' | b',' | b'_' => true,
            _ => false,
        }
    } else {
        false
    }
}
fn concat(a: &str, b: &str) -> String {
    let mut out = String::with_capacity(a.len() + b.len());
    out.push_str(a);
    out.push_str(b);
    out
}

pub fn singularize_item_name(item_tag: &String) -> String {
    let re_remove_item_count = Regex::new(r"^\d+ ").unwrap();
    let mut result = item_tag.clone();
    let matched_item_quantity: Vec<Match> = re_remove_item_count.find_iter(&item_tag).collect();
    let has_quantity = matched_item_quantity.len() > 0;
    for i in matched_item_quantity {
        result = result.replace(i.as_str(), "");
    }
    if !has_quantity {
        result
    } else {
        let mut words = result.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut singularized = false;
        if let Some(word) = words.first_mut() {
            singularize_word(&mut singularized, word);
        }
        if singularized {
            return words.join(" ")
        }

        if let Some(word) = words.last_mut() {
            singularize_word(&mut singularized, word);
        }

        if singularized {
            return words.join(" ")
        }

        if words.len() > 2 {
            for i in 1..words.len() - 2 {
                if let Some(word) = words.get_mut(i) {
                    singularize_word(&mut singularized, word);
                }
            }
        }

        words.join(" ")
    }
}

fn singularize_word(singularized: &mut bool, word: &mut String) {
    let word_singularized = singularize(word);
    if word_singularized != *word {
        *word = word_singularized.to_string();
        *singularized = true;
    }
}

/// Convert a word from plural to singular
pub fn singularize(word: &str) -> String {
    for &w in UNCOUNTABLE.iter() {
        if word_match(word, w) {
            return word.into();
        }
    }
    for &(singular, plural) in IRREGULAR.iter() {
        if let Some(pref) = word.strip_suffix(plural) {
            return concat(pref, singular);
        }
    }
    for &(singular, plural) in WORDS.iter() {
        if word == plural {
            return singular.to_string();
        }
    }
    for &(singular, plural) in ENDINGS.iter().rev() {
        if let Some(pref) = word.strip_suffix(plural) {
            return concat(pref, singular);
        }
    }
    if let Some(word) = word.strip_suffix("s") {
        word.to_string()
    } else {
        word.to_string()
    }
}

/// Convert a word from singular to plural
pub fn pluralize(word: &str) -> String {
    for &w in UNCOUNTABLE.iter() {
        if word_match(word, w) {
            return word.into();
        }
    }
    for &(singular, plural) in IRREGULAR.iter() {
        if let Some(pref) = word.strip_suffix(singular) {
            return concat(pref, plural);
        }
    }
    for &(singular, plural) in WORDS.iter() {
        if word == singular {
            return plural.to_string();
        }
    }
    for &(singular, plural) in ENDINGS.iter() {
        if let Some(pref) = word.strip_suffix(singular) {
            return concat(pref, plural);
        }
    }
    format!("{}s", word)
}

#[cfg(test)]
mod test;
