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
    forms: Option<Vec<Form>>,
    head_templates: Option<Vec<AdjectiveHeadTemplate>>,

    word: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VerbEntry {
    forms: Option<Vec<Form>>,
    //  head_templates: Option<Vec<AdjectiveHeadTemplate>>,
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
             //   && !line.contains("Latin proper nouns")
              //  && !line.contains("Latin proper noun forms")
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

    {
        let mut word_set: HashSet<String> = HashSet::new();
        let mut input_set: HashSet<String> = HashSet::new();
        let input_file = File::open("latin_nouns.jsonl")?;
        let reader = BufReader::new(input_file);

        let mut writer = csv::Writer::from_path("nouns.csv")?;
        //do irregular nouns manually
        writer.write_record(&[
            "word", "nom_sg", "gen_sg", "dat_sg", "acc_sg", "abl_sg", "voc_sg", "loc_sg", "nom_pl",
            "gen_pl", "dat_pl", "acc_pl", "abl_pl", "voc_pl", "loc_pl", "gender",
        ])?;

        for line in reader.lines() {
            println!("{:#?}", line);
            let line = line?;
            println!("SERIALIZNG");
            let entry: NounEntry = serde_json::from_str(&line)?;
            println!("SERIALIZNG DONE");
            let word = entry.word.clone();
            let mut nom_sg = String::new();
            let mut gen_sg = String::new();
            let mut dat_sg = String::new();
            let mut acc_sg = String::new();
            let mut abl_sg = String::new();
            let mut voc_sg = String::new();
            let mut loc_sg = String::new();
            let mut nom_pl = String::new();
            let mut gen_pl = String::new();
            let mut dat_pl = String::new();
            let mut acc_pl = String::new();
            let mut abl_pl = String::new();
            let mut voc_pl = String::new();
            let mut loc_pl = String::new();

            let mut gender = String::new();

            let mut pluralia_tantum = String::from("fa");

            if !word.contains(" ") {
                for form in &entry.forms {
                    if let Some(tags) = &form.tags {
                        if tags.contains(&"nominative".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if nom_sg == "" {
                                nom_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"genitive".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if gen_sg == "" {
                                gen_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"dative".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if dat_sg == "" {
                                dat_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"accusative".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if acc_sg == "" {
                                acc_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"ablative".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if abl_sg == "" {
                                abl_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"vocative".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if voc_sg == "" {
                                voc_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"locative".to_string())
                            && tags.contains(&"singular".to_string())
                        {
                            if loc_sg == "" {
                                loc_sg = form.form.clone();
                            }
                        }
                        if tags.contains(&"nominative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if nom_pl == "" {
                                nom_pl = form.form.clone();
                            }
                        }
                        if tags.contains(&"genitive".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if gen_pl == "" {
                                gen_pl = form.form.clone();
                            }
                        }
                        if tags.contains(&"dative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if dat_pl == "" {
                                dat_pl = form.form.clone();
                            }
                        }
                        if tags.contains(&"accusative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if acc_pl == "" {
                                acc_pl = form.form.clone();
                            }
                        }
                        if tags.contains(&"ablative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if abl_pl == "" {
                                abl_pl = form.form.clone();
                            }
                        }
                        if tags.contains(&"vocative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if voc_pl == "" {
                                voc_pl = form.form.clone();
                            }
                        }
                        if tags.contains(&"locative".to_string())
                            && tags.contains(&"plural".to_string())
                        {
                            if loc_pl == "" {
                                loc_pl = form.form.clone();
                            }
                        }
                    }
                }

                nom_sg = diacritics::remove_diacritics(nom_sg.as_str());
                gen_sg = diacritics::remove_diacritics(gen_sg.as_str());
                dat_sg = diacritics::remove_diacritics(dat_sg.as_str());
                acc_sg = diacritics::remove_diacritics(acc_sg.as_str());
                abl_sg = diacritics::remove_diacritics(abl_sg.as_str());
                voc_sg = diacritics::remove_diacritics(voc_sg.as_str());
                loc_sg = diacritics::remove_diacritics(loc_sg.as_str());
                nom_pl = diacritics::remove_diacritics(nom_pl.as_str());
                gen_pl = diacritics::remove_diacritics(gen_pl.as_str());
                dat_pl = diacritics::remove_diacritics(dat_pl.as_str());
                acc_pl = diacritics::remove_diacritics(acc_pl.as_str());
                abl_pl = diacritics::remove_diacritics(abl_pl.as_str());
                voc_pl = diacritics::remove_diacritics(voc_pl.as_str());
                loc_pl = diacritics::remove_diacritics(loc_pl.as_str());

                if line.contains("Latin masculine nouns") {
                    gender.push_str("m");
                } else if line.contains("Latin feminine nouns") {
                    gender.push_str("f");
                } else if line.contains("Latin neuter nouns") {
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

                let real_id = format!("{}{}", nom_sg, gen_sg);
                let mut insert_id = format!("{}", nom_sg);

                if (nom_sg != "") && (gen_sg != "") && !insert_id.contains("-") {
                    let unique_word = word_set.insert(real_id.clone());
                    let unique_insert = input_set.insert(insert_id.clone());

                    if unique_word && unique_insert {
                        // i am removing all diacritics to avoid confusion because some words will be wrongly marked otherwise
                        writer.write_record(&[
                            diacritics::remove_diacritics(insert_id.as_str()),
                            nom_sg,
                            gen_sg,
                            dat_sg,
                            acc_sg,
                            abl_sg,
                            voc_sg,
                            loc_sg,
                            nom_pl,
                            gen_pl,
                            dat_pl,
                            acc_pl,
                            abl_pl,
                            voc_pl,
                            loc_pl,
                            gender,
                        ])?;
                    } else if unique_word && !unique_insert {
                        insert_id = format!("{}{}", nom_sg, 2);

                        if input_set.insert(insert_id.clone()) {
                            writer.write_record(&[
                                diacritics::remove_diacritics(insert_id.as_str()),
                                nom_sg,
                                gen_sg,
                                dat_sg,
                                acc_sg,
                                abl_sg,
                                voc_sg,
                                loc_sg,
                                nom_pl,
                                gen_pl,
                                dat_pl,
                                acc_pl,
                                abl_pl,
                                voc_pl,
                                loc_pl,
                                gender,
                            ])?;
                        } else {
                            panic!("THIRD");
                        }
                    }
                }
            }
        }
        writer.flush()?;
    }
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives
    //process adjectives

    {
        let input_file = File::open("latin_adjectives.jsonl")?;
        let reader = BufReader::new(input_file);
        let mut adj_set: HashSet<String> = HashSet::new();

        let mut writer = csv::Writer::from_path("adjectives.csv")?;
        writer.write_record(&[
            "word",
            "comparative",
            "superlative",
            "adverb",
            "nom_sg_masc",
            "gen_sg_masc",
            "dat_sg_masc",
            "acc_sg_masc",
            "abl_sg_masc",
            "nom_sg_fem",
            "gen_sg_fem",
            "dat_sg_fem",
            "acc_sg_fem",
            "abl_sg_fem",
            "nom_sg_neut",
            "gen_sg_neut",
            "dat_sg_neut",
            "acc_sg_neut",
            "abl_sg_neut",
            "nom_pl_masc",
            "gen_pl_masc",
            "dat_pl_masc",
            "acc_pl_masc",
            "abl_pl_masc",
            "nom_pl_fem",
            "gen_pl_fem",
            "dat_pl_fem",
            "acc_pl_fem",
            "abl_pl_fem",
            "nom_pl_neut",
            "gen_pl_neut",
            "dat_pl_neut",
            "acc_pl_neut",
            "abl_pl_neut",
        ])?;

        for line in reader.lines() {
            println!("{:#?}", line);
            let line = line?;
            println!("SERIALIZNG ADJECTIVE");
            let entry: AdjectiveEntry = serde_json::from_str(&line)?;
            println!("SERIALIZNG ADJECTIVE DONE");

            let mut word = entry.word.clone();

            let mut comparative = String::new();
            let mut superlative = String::new();
            let mut adverb = String::new();

            let mut nom_sg_masc = String::new();
            let mut gen_sg_masc = String::new();
            let mut dat_sg_masc = String::new();
            let mut acc_sg_masc = String::new();
            let mut abl_sg_masc = String::new();

            let mut nom_sg_fem = String::new();
            let mut gen_sg_fem = String::new();
            let mut dat_sg_fem = String::new();
            let mut acc_sg_fem = String::new();
            let mut abl_sg_fem = String::new();

            let mut nom_sg_neut = String::new();
            let mut gen_sg_neut = String::new();
            let mut dat_sg_neut = String::new();
            let mut acc_sg_neut = String::new();
            let mut abl_sg_neut = String::new();

            let mut nom_pl_masc = String::new();
            let mut gen_pl_masc = String::new();
            let mut dat_pl_masc = String::new();
            let mut acc_pl_masc = String::new();
            let mut abl_pl_masc = String::new();

            let mut nom_pl_fem = String::new();
            let mut gen_pl_fem = String::new();
            let mut dat_pl_fem = String::new();
            let mut acc_pl_fem = String::new();
            let mut abl_pl_fem = String::new();

            let mut nom_pl_neut = String::new();
            let mut gen_pl_neut = String::new();
            let mut dat_pl_neut = String::new();
            let mut acc_pl_neut = String::new();
            let mut abl_pl_neut = String::new();

            if let Some(forms) = entry.forms {
                for form in forms {
                    if let Some(source) = &form.source {
                        if source == "declension" || source == "inflection" {
                            if let Some(tags) = &form.tags {
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"nominative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    nom_sg_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"genitive".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    gen_sg_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"dative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    dat_sg_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"accusative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    acc_sg_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"ablative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    abl_sg_masc = form.form.clone();
                                }

                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"nominative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    nom_sg_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"genitive".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    gen_sg_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"dative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    dat_sg_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"accusative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    acc_sg_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"ablative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    abl_sg_fem = form.form.clone();
                                }

                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"nominative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    nom_sg_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"genitive".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    gen_sg_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"dative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    dat_sg_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"accusative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    acc_sg_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"ablative".to_string())
                                    && tags.contains(&"singular".to_string())
                                {
                                    abl_sg_neut = form.form.clone();
                                }

                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"nominative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    nom_pl_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"genitive".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    gen_pl_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"dative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    dat_pl_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"accusative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    acc_pl_masc = form.form.clone();
                                }
                                if tags.contains(&"masculine".to_string())
                                    && tags.contains(&"ablative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    abl_pl_masc = form.form.clone();
                                }

                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"nominative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    nom_pl_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"genitive".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    gen_pl_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"dative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    dat_pl_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"accusative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    acc_pl_fem = form.form.clone();
                                }
                                if tags.contains(&"feminine".to_string())
                                    && tags.contains(&"ablative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    abl_pl_fem = form.form.clone();
                                }

                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"nominative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    nom_pl_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"genitive".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    gen_pl_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"dative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    dat_pl_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"accusative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    acc_pl_neut = form.form.clone();
                                }
                                if tags.contains(&"neuter".to_string())
                                    && tags.contains(&"ablative".to_string())
                                    && tags.contains(&"plural".to_string())
                                {
                                    abl_pl_neut = form.form.clone();
                                }
                            }
                        }
                    }
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

                if nom_sg_fem != "" && nom_sg_neut != "" && gen_sg_masc != "" {
                    //gen stem

                    let original_gen = gen_sg_masc.clone();
                    word = diacritics::remove_diacritics(word.as_str());

                    comparative = diacritics::remove_diacritics(comparative.as_str());
                    superlative = diacritics::remove_diacritics(superlative.as_str());
                    adverb = diacritics::remove_diacritics(adverb.as_str());
                    nom_sg_masc = diacritics::remove_diacritics(nom_sg_masc.as_str());
                    gen_sg_masc = diacritics::remove_diacritics(gen_sg_masc.as_str());
                    dat_sg_masc = diacritics::remove_diacritics(dat_sg_masc.as_str());
                    acc_sg_masc = diacritics::remove_diacritics(acc_sg_masc.as_str());
                    abl_sg_masc = diacritics::remove_diacritics(abl_sg_masc.as_str());

                    nom_sg_fem = diacritics::remove_diacritics(nom_sg_fem.as_str());
                    gen_sg_fem = diacritics::remove_diacritics(gen_sg_fem.as_str());
                    dat_sg_fem = diacritics::remove_diacritics(dat_sg_fem.as_str());
                    acc_sg_fem = diacritics::remove_diacritics(acc_sg_fem.as_str());
                    abl_sg_fem = diacritics::remove_diacritics(abl_sg_fem.as_str());

                    nom_sg_neut = diacritics::remove_diacritics(nom_sg_neut.as_str());
                    gen_sg_neut = diacritics::remove_diacritics(gen_sg_neut.as_str());
                    dat_sg_neut = diacritics::remove_diacritics(dat_sg_neut.as_str());
                    acc_sg_neut = diacritics::remove_diacritics(acc_sg_neut.as_str());
                    abl_sg_neut = diacritics::remove_diacritics(abl_sg_neut.as_str());

                    nom_pl_masc = diacritics::remove_diacritics(nom_pl_masc.as_str());
                    gen_pl_masc = diacritics::remove_diacritics(gen_pl_masc.as_str());
                    dat_pl_masc = diacritics::remove_diacritics(dat_pl_masc.as_str());
                    acc_pl_masc = diacritics::remove_diacritics(acc_pl_masc.as_str());
                    abl_pl_masc = diacritics::remove_diacritics(abl_pl_masc.as_str());

                    nom_pl_fem = diacritics::remove_diacritics(nom_pl_fem.as_str());
                    gen_pl_fem = diacritics::remove_diacritics(gen_pl_fem.as_str());
                    dat_pl_fem = diacritics::remove_diacritics(dat_pl_fem.as_str());
                    acc_pl_fem = diacritics::remove_diacritics(acc_pl_fem.as_str());
                    abl_pl_fem = diacritics::remove_diacritics(abl_pl_fem.as_str());

                    nom_pl_neut = diacritics::remove_diacritics(nom_pl_neut.as_str());
                    gen_pl_neut = diacritics::remove_diacritics(gen_pl_neut.as_str());
                    dat_pl_neut = diacritics::remove_diacritics(dat_pl_neut.as_str());
                    acc_pl_neut = diacritics::remove_diacritics(acc_pl_neut.as_str());
                    abl_pl_neut = diacritics::remove_diacritics(abl_pl_neut.as_str());

                    let mut adj_stem = gen_sg_masc.clone();
                    println!("{:#?}", adj_stem);

                    if original_gen.ends_with("ī̆us") {
                        adj_stem.pop();
                        adj_stem.pop();
                        adj_stem.pop();
                    } else if gen_sg_masc.ends_with("i") {
                        adj_stem.pop();
                    } else if gen_sg_masc.ends_with("ae") {
                        adj_stem.pop();
                        adj_stem.pop();
                    } else if gen_sg_masc.ends_with("is") {
                        adj_stem.pop();
                        adj_stem.pop();
                    } else if gen_sg_masc.ends_with("us") {
                        adj_stem.pop();
                        adj_stem.pop();
                    } else if gen_sg_masc.ends_with("os") {
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
                            comparative,
                            superlative,
                            adverb,
                            nom_sg_masc,
                            gen_sg_masc,
                            dat_sg_masc,
                            acc_sg_masc,
                            abl_sg_masc,
                            nom_sg_fem,
                            gen_sg_fem,
                            dat_sg_fem,
                            acc_sg_fem,
                            abl_sg_fem,
                            nom_sg_neut,
                            gen_sg_neut,
                            dat_sg_neut,
                            acc_sg_neut,
                            abl_sg_neut,
                            nom_pl_masc,
                            gen_pl_masc,
                            dat_pl_masc,
                            acc_pl_masc,
                            abl_pl_masc,
                            nom_pl_fem,
                            gen_pl_fem,
                            dat_pl_fem,
                            acc_pl_fem,
                            abl_pl_fem,
                            nom_pl_neut,
                            gen_pl_neut,
                            dat_pl_neut,
                            acc_pl_neut,
                            abl_pl_neut,
                        ])?;
                    }
                }
            }
        }
    }

    //process verbs
    //process verbs
    //process verbs
    //process verbs
    //process verbs
    //process verbs
    //process verbs
    //process verbs
    {
        let mut verb_set: HashSet<String> = HashSet::new();
        let input_file = File::open("latin_verbs.jsonl")?;
        let verb_reader = BufReader::new(input_file);

        let mut verb_writer = csv::Writer::from_path("verbs.csv")?;
        //do irregular nouns manually
        verb_writer.write_record(&[
            "word",
            "canonical",
            "present_infinitive",
            "perfect_active",
            "supine",
            "conjugation",
            "irregular",
        ])?;

        for line in verb_reader.lines() {
            println!("{:#?}", line);
            let line = line?;
            println!("SERIALIZNG VERB");
            let entry: VerbEntry = serde_json::from_str(&line)?;
            println!("SERIALIZNG VERB DONE");
            let mut canonical = String::new();
            let word = entry.word.clone();
            let mut present_infinitive = String::new();
            let mut perfect_active = String::new();
            let mut supine = String::new();
            let mut conjugation = String::from("");
            let mut irregular = String::from("fa");

            if !canonical.contains(" ") {
                if let Some(forms) = entry.forms {
                    for form in forms {
                        if let Some(tags) = &form.tags {
                            if tags.contains(&"active".to_string())
                                && tags.contains(&"first-person".to_string())
                                && tags.contains(&"indicative".to_string())
                                && tags.contains(&"present".to_string())
                                && tags.contains(&"singular".to_string())
                            {
                                canonical = form.form.clone();
                            }
                            if tags.contains(&"infinitive".to_string())
                                && tags.contains(&"present".to_string())
                                && tags.contains(&"active".to_string())
                            {
                                present_infinitive = form.form.clone();
                            }
                            if tags.contains(&"active".to_string())
                                && tags.contains(&"perfect".to_string())
                                && tags.len() == 2
                            {
                                perfect_active = form.form.clone();
                            }
                            if tags.contains(&"supine".to_string()) && tags.len() == 1 {
                                supine = form.form.clone();
                            }
                        }
                    }
                }

                if line.contains("Latin first conjugation verbs") {
                    conjugation.push_str("1");
                } else if line.contains("Latin second conjugation verbs") {
                    conjugation.push_str("2");
                } else if line.contains("Latin third conjugation verbs") {
                    conjugation.push_str("3");
                } else if line.contains("Latin fourth conjugation verbs") {
                    conjugation.push_str("4");
                } else {
                    conjugation.push_str("i");
                }

                if line.contains("Latin irregular verbs") {
                    irregular = String::from("tr");
                }

                if line.contains("Latin verbs with missing supine stem") {
                    supine = String::from("MISSING");
                }
                if line.contains("Latin verbs with missing perfect stem") {
                    perfect_active = String::from("MISSING");
                }
                if supine == "" {
                    supine = String::from("MISSING");
                }
                if perfect_active == "" {
                    perfect_active = String::from("MISSING");
                }

                if (canonical != "")
             //   && (genitive != "")
             //   && (nominative != "-")
             //   && (genitive != "-")
                && !canonical.contains("-")
                && !canonical.contains(" ")
                {
                    if verb_set.insert(diacritics::remove_diacritics(word.as_str())) {
                        // i am removing all diacritics to avoid confusion because some words will be wrongly marked otherwise
                        verb_writer.write_record(&[
                            diacritics::remove_diacritics(word.as_str()),
                            diacritics::remove_diacritics(canonical.as_str()),
                            diacritics::remove_diacritics(present_infinitive.as_str()),
                            diacritics::remove_diacritics(perfect_active.as_str()),
                            diacritics::remove_diacritics(supine.as_str()),
                            conjugation,
                            irregular,
                        ])?;
                    }
                }
            }
        }
        verb_writer.flush()?;
    }

    Ok(())
}
