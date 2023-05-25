/// A library that uses Hugging Face to Translate Text
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io::Read;

//build a function that reads a file and returns a string
pub fn read_file(path: String) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//build a function that reads a file and returns an array of the lines of the file
pub fn read_file_array(path: String) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let array = contents.lines().map(|s| s.to_string()).collect();
    Ok(array)
}

//build a function that reads a file and translates it
pub fn translate_file(path: String) -> anyhow::Result<()> {
    let source_languages = vec![Language::German];
    let target_language = Language::English;
    let model = TranslationModelBuilder::new()
        .with_source_languages(source_languages)
        .with_target_languages(vec![target_language])
        .create_model()?;
    let text = read_file_array(path)?;
    //pass in the text to the model
    let output = model.translate(&text, None, target_language)?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}
