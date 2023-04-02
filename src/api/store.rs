use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

static QUESTION_PATH: &str = "/.questions";

pub fn ensure_stored_questions_file() -> () {
    let path = Path::new(QUESTION_PATH);

    if !path.exists() {
        File::create(QUESTION_PATH).expect(&format!(
            "Couldn't create file for previous chat logs at {}",
            QUESTION_PATH
        ));
        println!(
            "Previous chat log file didn't exist at {} so we created it for you.",
            QUESTION_PATH
        );
    }
}

pub fn get_stored_questions() -> Result<Vec<String>, std::io::Error> {
    let handle = match File::open(QUESTION_PATH) {
        Ok(handle) => handle,
        Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
            let new_handle = File::create(QUESTION_PATH)
                .expect("Unable to create a file for questions history.");
            println!(
                "Questions file didn't exist but it has now been created at {}",
                QUESTION_PATH,
            );
            new_handle
        }
        Err(e) => panic!("{}", e),
    };

    let reader = BufReader::new(handle);

    let mut lines_vec: Vec<String> = Vec::with_capacity(40);

    for line in reader.lines() {
        lines_vec.push(line?);
    }

    if lines_vec.len() > 20 {
        let mut handle_to_overwrite = File::create(QUESTION_PATH).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Couldn't overwrite the contents of the existing questions file.",
            )
        })?;

        // We want to store maximum 20 previous questions.
        lines_vec = lines_vec.split_off(lines_vec.len() - 20);

        // Overwriting the contents of the question file with no more than 20 questions.
        for line in &lines_vec {
            handle_to_overwrite.write_all(line.as_bytes())?;
            handle_to_overwrite.write_all(b"\n")?;
        }
    }

    Ok(lines_vec)
}
