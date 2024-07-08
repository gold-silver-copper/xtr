use serde_json::{Result, Value};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Form {
    form: String,
    tags: Option<Vec<String>>,
    source: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NounEntry {
    forms: Vec<Form>,

    word: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AdjectiveHeadTemplate {
    args: Value,
    expansion: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AdjectiveEntry {
    // etymology_templates: Vec<EtymologyTemplate>,
    // etymology_text: String,
    forms: Option<Vec<Form>>,
    head_templates: Option<Vec<AdjectiveHeadTemplate>>,
    //  inflection_templates: Vec<InflectionTemplate>,
    //  lang: String,
    //  lang_code: String,
    //  pos: String,
    //   senses: Vec<Sense>,
    word: String,
}

fn main() -> io::Result<()> {
    // Define the input and output file paths
    let input_file_path = "meow.json";
    let output_nouns_path = "latin_nouns.jsonl";
    let output_verbs_path = "latin_verbs.jsonl";
    let output_adjectives_path = "latin_adjectives.jsonl";

    // Open the input file
    let input_file = File::open(input_file_path)?;
    let reader = BufReader::new(input_file);

    // Create or open the output files
    let mut output_nouns_file = File::create(output_nouns_path)?;
    let mut output_verbs_file = File::create(output_verbs_path)?;
    let mut output_adjectives_file = File::create(output_adjectives_path)?;

    // Iterate through each line of the input file
    for line in reader.lines() {
        let line = line?;

        let mut json_value: Value = match serde_json::from_str(&line) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
                continue;
            }
        };

        // Remove the "descendants" section
        if let Some(object) = json_value.as_object_mut() {
            object.remove("descendants");
            object.remove("etymology_number");
            //  object.remove("etymology_templates");
            // object.remove("head_templates");
            //  object.remove("inflection_templates");
            //   object.remove("categories");
            //     object.remove("lang_code");
            // object.remove("lang");
            object.remove("sounds");
            //     object.remove("pos");
        }

        // Serialize the modified JSON object to a string
        let modified_json = match serde_json::to_string(&json_value) {
            Ok(json_str) => json_str,
            Err(err) => {
                eprintln!("Failed to serialize JSON: {}", err);
                continue;
            }
        };

        let adj_check = r#""adj""#;
        let verb_check = r#""verb""#;
        let noun_check = r#""noun""#;

        // Check if the line contains the string "Latin Lemma"
        if line.contains("Latin lemmas") {
            // Write the line to the lemmas output file
            // Check if the line contains the string "Latin nouns"
            if line.contains("Latin nouns")
                && !line.contains(adj_check)
                && !line.contains(verb_check)
                && !line.contains("Latin cardinal numbers")
                && line.contains("nominative")
                && line.contains("genitive")
                && line.contains("dative")
                && line.contains("accusative")
            // && !(line.contains("masculine") && line.contains("feminine") && line.contains("neuter"))
            {
                // Write the line to the nouns output file

                writeln!(output_nouns_file, "{}", modified_json)?;
            }
            // Check if the line contains the string "Latin verbs"
            if line.contains("Latin verbs") {
                // Write the line to the verbs output file

                writeln!(output_verbs_file, "{}", modified_json)?;
            }
            // Check if the line contains the string "Latin adjectives"
            if line.contains("Latin adjectives")
                && !line.contains("Latin abbreviations")
                && !line.contains("Latin pronouns")
                && !line.contains("Latin indeclinable adjectives")
                && !line.contains("Latin terms suffixed with -libet")
                && !line.contains("with an indeclinable portion")
            //     && !line.contains(noun_check)
            //      && !line.contains(verb_check)
            {
                // Write the line to the adjectives output file

                writeln!(output_adjectives_file, "{}", modified_json)?;
            }
        }
    }

    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file
    //now process the resulting nouns file

    let mut word_set: HashSet<String> = HashSet::new();
    let input_file = File::open("latin_nouns.jsonl")?;
    let reader = BufReader::new(input_file);

    let mut writer = csv::Writer::from_path("nouns.csv")?;
    //do irregular nouns manually
    writer.write_record(&[
        "word",
        "nominative",
        "genitive",
        "gender",
        "irregular",
        "pluralia_tantum",
    ])?;

    for line in reader.lines() {
        println!("{:#?}", line);
        let line = line?;
        println!("SERIALIZNG");
        let entry: NounEntry = serde_json::from_str(&line)?;
        println!("SERIALIZNG DONE");
        let word = entry.word.clone();
        let mut nominative = String::new();
        let mut genitive = String::new();
        let mut gender = String::new();

        let mut irregular = String::from("fa");
        let mut pluralia_tantum = String::from("fa");

        if !word.contains(" ") {
            for form in &entry.forms {
                if let Some(tags) = &form.tags {
                    if tags.contains(&"nominative".to_string())
                        && tags.contains(&"singular".to_string())
                    {
                        if nominative == "" {
                            nominative = form.form.clone();
                        }
                    }
                    if tags.contains(&"genitive".to_string())
                        && tags.contains(&"singular".to_string())
                    {
                        if genitive == "" {
                            genitive = form.form.clone();
                        }
                    }
                }
            }

            if (nominative == "") && (genitive == "") {
                pluralia_tantum = "tr".into();

                for form in &entry.forms {
                    if let Some(tags) = &form.tags {
                        if tags.contains(&"nominative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if nominative == "" {
                                nominative = form.form.clone();
                            }
                        }
                        if tags.contains(&"genitive".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if genitive == "" {
                                genitive = form.form.clone();
                            }
                        }
                    }
                }
            }

            if line.contains("Latin masculine nouns") {
                gender.push_str("m");
            }
            if line.contains("Latin feminine nouns") {
                gender.push_str("f");
            }
            if line.contains("Latin neuter nouns") {
                gender.push_str("n");
            }

            if gender == "" {
                //for nouns with uncertain gender

                if word.ends_with("a") {
                    gender.push_str("f");
                } else if word.ends_with("n") {
                    gender.push_str("n");
                } else {
                    gender.push_str("m");
                }
            }

            if line.contains("Latin irregular nouns") {
                irregular = "tr".into();
            }

            let plain_gen = diacritics::remove_diacritics(genitive.as_str());

            let real_id = format!("{}_{}", word, plain_gen);

            if (nominative != "")
                && (genitive != "")
                && (nominative != "-")
                && (genitive != "-")
                && !word.contains("-")
            {
                if word_set.insert(real_id.clone()) {
                    // i am removing all diacritics to avoid confusion because some words will be wrongly marked otherwise
                    writer.write_record(&[
                        diacritics::remove_diacritics(real_id.as_str()),
                        diacritics::remove_diacritics(nominative.as_str()),
                        diacritics::remove_diacritics(genitive.as_str()),
                        gender,
                        irregular,
                        pluralia_tantum,
                    ])?;
                }
            }
        }
    }
    writer.flush()?;
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives

    let input_file = File::open("latin_adjectives.jsonl")?;
    let reader = BufReader::new(input_file);
    let mut adj_set: HashSet<String> = HashSet::new();

    let mut writer = csv::Writer::from_path("adjectives.csv")?;
    writer.write_record(&[
        "word",
        "feminine",
        "neuter",
        "comparative",
        "superlative",
        "adverb",
        "declension",
        "adj_stem",
    ])?;

    for line in reader.lines() {
        println!("{:#?}", line);
        let line = line?;
        println!("SERIALIZNG ADJECTIVE");
        let entry: AdjectiveEntry = serde_json::from_str(&line)?;
        println!("SERIALIZNG ADJECTIVE DONE");

        let mut word = entry.word.clone();
        let mut genitive = String::new();
        let mut declension = String::new();
        let mut feminine = String::new();
        let mut neuter = String::new();
        let mut comparative = String::new();
        let mut superlative = String::new();
        let mut adverb = String::new();

        if let Some(forms) = entry.forms {
            for form in forms {
                if let Some(source) = &form.source {
                    if source == "declension" || source == "inflection" {
                        if let Some(tags) = &form.tags {
                            if tags.contains(&"feminine".to_string())
                                && tags.contains(&"nominative".to_string())
                                && tags.contains(&"singular".to_string())
                            {
                                feminine = form.form.clone();
                            }
                            if tags.contains(&"neuter".to_string())
                                && tags.contains(&"nominative".to_string())
                                && tags.contains(&"singular".to_string())
                            {
                                neuter = form.form.clone();
                            }
                            if tags.contains(&"masculine".to_string())
                                && tags.contains(&"genitive".to_string())
                                && tags.contains(&"singular".to_string())
                            {
                                genitive = form.form.clone();
                            }
                        }
                    }
                } else {
                    if let Some(tags) = &form.tags {
                        if tags.contains(&"feminine".to_string()) && tags.len() == 1 {
                            feminine = form.form.clone();
                        }
                        if tags.contains(&"neuter".to_string()) && tags.len() == 1 {
                            neuter = form.form.clone();
                        }
                    }
                }
            }

            if line.contains("Latin third declension adjectives of one termination") {
                feminine = word.clone();
                neuter = word.clone();
            }

            if line.contains("Latin third declension adjectives of two terminations") {
                feminine = word.clone();
            }

            if line.contains("Latin third declension adjectives") {
                declension = "3".to_string();
            } else if line.contains("Latin first declension adjectives") {
                declension = "1".to_string();
            } else if line.contains("Latin second declension adjectives") {
                declension = "2".to_string();
            } else {
                declension = "12".to_string();
            }

            if let Some(ht) = entry.head_templates {
                if let Some(head_template) = ht.get(0) {
                    if let Some(c) = head_template.args.get("comp") {
                        comparative = c.as_str().unwrap_or("").to_string();
                    }
                    if let Some(s) = head_template.args.get("sup") {
                        superlative = s.as_str().unwrap_or("").to_string();
                    }
                    if let Some(a) = head_template.args.get("adv") {
                        adverb = a.as_str().unwrap_or("").to_string();
                    }
                }
            }

            if feminine != "" && neuter != "" && genitive != "" {
                //gen stem

                let original_gen = genitive.clone();
                word = diacritics::remove_diacritics(word.as_str());
                genitive = diacritics::remove_diacritics(genitive.as_str());
                feminine = diacritics::remove_diacritics(feminine.as_str());
                neuter = diacritics::remove_diacritics(neuter.as_str());
                comparative = diacritics::remove_diacritics(comparative.as_str());
                superlative = diacritics::remove_diacritics(superlative.as_str());
                adverb = diacritics::remove_diacritics(adverb.as_str());
                let mut adj_stem = genitive.clone();
                println!("{:#?}", adj_stem);

                if original_gen.ends_with("ī̆us") {
                    adj_stem.pop();
                    adj_stem.pop();
                    adj_stem.pop();
                } else if genitive.ends_with("i") {
                    adj_stem.pop();
                } else if genitive.ends_with("ae") {
                    adj_stem.pop();
                    adj_stem.pop();
                } else if genitive.ends_with("is") {
                    adj_stem.pop();
                    adj_stem.pop();
                } else if genitive.ends_with("us") {
                    adj_stem.pop();
                    adj_stem.pop();
                } else if genitive.ends_with("os") {
                    adj_stem.pop();
                    adj_stem.pop();
                } else {
                    panic!("COULDNT GET ADH STEM");
                }

                if comparative == "" {
                    comparative = format!("{}{}", adj_stem, "ior")
                }

                if superlative == "" {
                    if word.ends_with("er") {
                        superlative = format!("{}{}", word, "rimus")
                    } else {
                        superlative = format!("{}{}", adj_stem, "issimus")
                    }
                }
                if adverb == "" {
                    //group A
                    if word.ends_with("er") || word.ends_with("us") {
                        adverb = format!("{}{}", adj_stem, "e")
                    } else if adj_stem.ends_with("nt") {
                        adverb = format!("{}{}", adj_stem, "ter")
                    } else {
                        adverb = format!("{}{}", adj_stem, "iter")
                    }
                }
                if adj_set.insert(word.clone()) {
                    writer.write_record(&[
                        word,
                        feminine,
                        neuter,
                        comparative,
                        superlative,
                        adverb,
                        declension,
                        adj_stem,
                    ])?;
                }
            }
        }
    }

    Ok(())
}
