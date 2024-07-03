use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use serde_json::{Result, Value};
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
         //   object.remove("sounds");
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





        // Check if the line contains the string "Latin Lemma"
        if line.contains("Latin lemmas") {
            // Write the line to the lemmas output file
            // Check if the line contains the string "Latin nouns"
            if line.contains("Latin nouns") {
                // Write the line to the nouns output file
          
                writeln!(output_nouns_file, "{}", modified_json)?;
            }
            // Check if the line contains the string "Latin verbs"
            if line.contains("Latin verbs") {
                // Write the line to the verbs output file
            
                writeln!(output_verbs_file, "{}", modified_json)?;
            }
            // Check if the line contains the string "Latin adjectives"
            if line.contains("Latin adjectives") {
                // Write the line to the adjectives output file
               
                writeln!(output_adjectives_file, "{}", modified_json)?;
            }
        }
    }

    Ok(())
}
