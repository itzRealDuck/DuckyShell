use crossterm::{
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use io::Read;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{stdout, Write};
use std::path::Path;
use std::process;

fn cd(input: &mut String) {
    let _ = env::set_current_dir(
        //    "/home/".to_string()
        //      + &whoami::realname().to_owned().to_lowercase()
        //    + "/"
        &input,
    );

    if input == "/" {
        let _ = env::set_current_dir("/");
    }
}
mod ls {
    use std::error::Error;
    use std::fs;
    use std::io;
    use std::path::Path;

    pub fn list(dir: &Path) -> Result<(), Box<dyn Error>> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let file_name = entry
                    .file_name()
                    .into_string()
                    .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
                println!("{}", file_name);
            }
        }
        Ok(())
    }

    pub fn lsl() -> io::Result<()> {
        let path = fs::read_dir(".")?;

        for pathy in path {
            let entry = pathy?;
            let actualpath = entry.path();

            if actualpath.is_file() {
                let metadata = actualpath.metadata();

                if metadata.expect("jitty").permissions().readonly() == true {
                    println!("read {:?}", actualpath);
                } else {
                    println!("write {:?}", actualpath);
                }
            }
        }

        Ok(())
    }
    pub fn help() {
        println!(
            "Usage: ls || Options: 
     -l            Show Permissions
      More Coming Soon!"
        );
        println!("Made By ItzReakDuck, implemented and is for DuckyShell");
    }
}
mod grep {
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::Read;

    pub fn grepfind(wordsearch: &String) -> io::Result<()> {
        let path = fs::read_dir(".")?;
        for entry in path {
            let entry = entry?;
            let relentry = entry.path();

            if relentry.is_file() {
                let FileOpen = File::open(&relentry);
                let mut file_container = String::new();
                let _ = FileOpen
                    .expect("just an expect")
                    .read_to_string(&mut file_container);
                if file_container.contains(wordsearch) {
                    println!("This File {:?} Has Your words {}!", relentry, wordsearch);
                } else {
                }
            }
        }
        Ok(())
    }
    pub fn greptext(text: &String, file: &String) -> io::Result<()> {
        for lines in fs::read_to_string(file).unwrap().lines() {
            if lines.contains(text.trim()) {
                println!("{}", lines);
            }
        }
        Ok(())
    }
    pub fn help() {
        println!(
            "Usage: grep [word], Other Options: 
   soon to be added"
        );
        println!("Made By ItzReakDuck, implemented and is for DuckyShell");
    }
}

mod yes {

    pub fn yes(printhing: &String) {
        if printhing.is_empty() {
            loop {
                println!("y");
            }
        } else {
            loop {
                println!("{}", printhing);
            }
        }
    }

    pub fn help() {
        println!(
            "yes Or yes [words to spam], Arguments: 
        --version         Print version
        --help || -h      Print this help message
                \n "
        );
        println!("Made By ItzReakDuck, implemented and is for DuckyShell");
    }
    pub fn version() {
        println!("Rewritten coreutils, yes 1.0");
    }
}

mod find {
    use std::collections::VecDeque;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    pub fn find(arg1: &String) -> io::Result<()> {
        let list = fs::read_dir(".")?;

        for entry in list {
            let entry = entry?;
            let path = &String::from(entry.path().file_name().unwrap().to_str().unwrap());

            if path == arg1 {
                println!("{}", path);
            } else {
            }
        }

        Ok(())
    }
    pub fn help() {
        println!(
            "Usage: find [file to find], Other Options: 
    --help      Print this message
    --version   Print version 
    Other Options Coming Soon
"
        );
        println!("Made By ItzReakDuck, implemented and is for DuckyShell");
    }
    pub fn version() {
        println!("Rewritten coreutils, find 0.1");
    }

    pub fn iname(path: &PathBuf, pattern: &String) -> io::Result<()> {
        let mut iter = VecDeque::new();

        iter.push_back(path.to_path_buf());

        while let Some(sheez) = iter.pop_front() {
            let dirs = fs::read_dir(&sheez)?;
            for entry in dirs {
                let entry = entry?;
                let pathy = entry.path();

                if pathy.is_dir() {
                    iter.push_back(pathy);
                } else if pathy
                    .to_string_lossy()
                    .to_lowercase()
                    .contains(&pattern.to_lowercase())
                {
                    println!("{}", pathy.display());
                }
            }
        }

        Ok(())
    }
}

mod cp {

    use std::collections::VecDeque;
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    pub fn cp(src: &PathBuf, dst: &mut PathBuf) -> Result<(), io::Error> {
        let mut iterator = VecDeque::new();
        iterator.push_back(src.to_path_buf());
        *dst = dst.join(src.file_name().unwrap());
        iterator.push_back(dst.to_path_buf());

        while let Some(cheese) = iterator.pop_front() {
            if let Some(bread) = iterator.pop_front() {
                fs::create_dir_all(&bread)?;

                for entry in fs::read_dir(&cheese)? {
                    let path = &entry?.path();

                    if path.is_dir() {
                        //  *dst = dst.join(path.file_name().unwrap());
                        iterator.push_back(path.to_path_buf());
                        iterator.push_back(bread.join(&path.file_name().unwrap()));
                    } else if path.is_file() {
                        let _ = fs::copy(&path, bread.join(&path.file_name().unwrap()));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn help() {
        println!("Usage: cp [optional flags] [from] [to] || Options: 
        -v || --version      ->  shows version of the command 
        -h || --help         ->  shows this help message 
           -r                ->  instead of an iterative approach, it recrusivly copy files and directories
      More Coming Soon!");
        println!("Made By ItzReakDuck, implemented and is for DuckyShell");
    }
    pub fn version() {
        println!("Rewritten coreutils, cp 0.3");
    }
    pub fn recrusive(src: &PathBuf, dst: &PathBuf) -> Result<(), io::Error> {
        fs::create_dir_all(&dst)?;

        for entry in fs::read_dir(src)? {
            let path = entry?.path();

            if path.is_dir() {
                let _ = recrusive(&path, &mut dst.join(path.file_name().unwrap()));
            } else if path.is_file() {
                let _ = fs::copy(&path, dst.join(path.file_name().unwrap()));
            }
        }
        Ok(())
    }
}

fn main() {
    let mut input = String::new();
    println!(
        "Release 7.3 Alpha of RustyShell,Welcome {} ",
        whoami::realname()
    );

    loop {
        io::stdin().read_line(&mut input).unwrap();

        let words: Vec<&str> = input.split_whitespace().collect();
        match words {
            // Ls Command And Its Arguments: Start
            words if words[0].trim() == "ls".trim() && words.get(1).is_none() => {
                if let Err(ref e) = crate::ls::list(Path::new(".")) {
                    println!("{}", e);
                    process::exit(1);
                }
            }
            words
                if words[0].trim() == "ls".trim()
                    && words[1] == words[1]
                    && Path::new(words[1].trim()).exists() =>
            {
                let path = Path::new(words[1].trim());

                if path.exists() {
                    if let Err(ref e) = crate::ls::list(path) {
                        println!("{}", e);
                        process::exit(1);
                    }
                } else {
                    println!("path not exist ");
                }
            }
            words if words[0].trim() == "ls" && words[1].trim() == "-l" => {
                let _ = crate::ls::lsl();
            }
            words
                if words[0].trim() == "ls" && words[1].trim() == "-h"
                    || words[0].trim() == "ls" && words[1].trim() == "--help" =>
            {
                let _ = crate::ls::help();
            }

            // End

            // Cd Command And Its Arguments: Start
            words if words[0] == "cd".trim() && words.get(1).is_none() => {
                // let whoamiString = String::from(whoami::realname());

                let mut input23 = String::from(
                    "/home/".to_string() + &whoami::realname().to_owned().to_lowercase(),
                );
                cd(&mut input23);
            }
            words
                if words[0].trim() == "cd".trim()
                    && words[1] == words[1]
                    && Path::new(words[1].trim()).exists() =>
            {
                let mut inputy = String::from(words[1].trim());

                cd(&mut inputy);

                inputy.clear()
            }
            // End

            // Clear Command: Start
            words if words[0].trim() == "clear".trim() => {
                let mut out = stdout();
                out.queue(Hide).unwrap();
                out.queue(Clear(ClearType::All)).unwrap();
                out.queue(MoveTo(0, 0)).unwrap();
                out.flush().unwrap();
            }
            // End

            // Exit Command: Start
            words if words[0].trim() == "exit".trim() => {
                std::process::exit(0);
            }
            // End

            // Cat And Its Arguments: Start
            words
                if words[0].trim() == "cat".trim()
                    && words[1] == words[1]
                    && Path::new(words[1].trim()).exists() =>
            {
                let pathy = Path::new(words[1].trim());

                let file_open = File::open(pathy);

                let mut file_container = String::new();

                let _ = file_open
                    .expect("idk what dis does")
                    .read_to_string(&mut file_container);

                println!("{}", file_container);
            }
            // End

            // Echo Command: Start
            words if words[0].trim() == "echo" && words[1] == words[1] && words[1] != "$PATH" => {
                println!("{}", words[1]);
            }
            words if words[0].trim() == "echo" && words[1] == "$PATH" => {
                let path = match env::current_dir() {
                    Ok(y) => y,
                    Err(..) => panic!("nuh uh"),
                };

                println!("{}", path.display());
            }

            // End

            // Touch And Its Arguments: Start
            words if words[0].trim() == "touch" && words[1].trim() == words[1].trim() => {
                let _ = File::create(words[1].trim());
            }

            // End

            // Mkdir And Its Arguments: Start
            words if words[0].trim() == "mkdir" && words[1].trim() == words[1].trim() => {
                let _ = fs::create_dir(words[1].trim());
            }

            // End

            // Rm And Its Arguments: Start
            words
                if words[0].trim() == "rm"
                    && words[1].trim() == "-rf"
                    && words[2].trim() == words[2].trim()
                    && Path::new(words[2].trim()).exists()
                    || words[0].trim() == "rm"
                        && words[1].trim() == "-rf"
                        && words[2].trim() == words[2].trim() =>
            {
                // let _ = fs::remove_dir_all(words[2].trim());

                let filechecker = Path::new(words[2].trim());

                if filechecker.is_file() {
                    let _ = fs::remove_file(words[2].trim());
                } else {
                    let _ = fs::remove_dir_all(words[2].trim());
                }
            }

            words if words[0].trim() == "rm" && words[1].trim() == words[1].trim() => {
                let _ = fs::remove_file(words[1].trim());
            }
            // End
            // Grep Command And Its Arguments: Start
            words
                if words[0].trim() == "grep" && words[1].trim() == "-h"
                    || words[0].trim() == "grep" && words[1].trim() == "--help" =>
            {
                let _ = crate::grep::help();
            }
            words
                if words[0].trim() == "grep"
                    && words[1].trim() == "--text"
                    && words[2].trim() == words[2].trim()
                    && words[3].trim() == words[3].trim() =>
            {
                //               let words2_to_string = words[2].to_string();
                //             let words3_to_string = words[3].to_string();

                let _ = crate::grep::greptext(&words[2].to_string(), &words[3].to_string());
            }
            words if words[0].trim() == "grep" && words[1].trim() == words[1].trim() => {
                let _ = crate::grep::grepfind(&words[1].to_string());
            }

            // End

            // yes Command And Its Arguments: Start
            words if words[0].trim() == "yes" && words.get(1).is_none() => {
                let _ = crate::yes::yes(&String::new());
            }
            words
                if words[0].trim() == "yes" && words[1].trim() == "-h"
                    || words[0].trim() == "yes" && words[1].trim() == "--help" =>
            {
                let _ = crate::yes::help();
            }
            words if words[0].trim() == "yes" && words[1].trim() == "--version" => {
                let _ = crate::yes::version();
            }
            words if words[0].trim() == "yes" && words.get(1).is_some() => {
                let _ = crate::yes::yes(&words[1].to_string());
            }

            // End

            // find Command Its Arguments: Start
            words
                if words[0].trim() == "find"
                    && words[1].trim() == words[1].trim()
                    && Path::new(words[1].trim()).exists() =>
            {
                let _ = crate::find::find(&words[1].to_string());
            }
            words if words[0].trim() == "find" && words[1].trim() == "--help" => {
                let _ = crate::find::help();
            }
            words if words[0].trim() == "find" && words[1].trim() == "--version" => {
                let _ = crate::find::version();
            }
            words
                if words[0].trim() == "find"
                    && words[1].trim() == "-name"
                    && words[2].trim() == words[2].trim() =>
            {
                use std::path::PathBuf;
                let _ = crate::find::iname(&PathBuf::from("."), &words[2].to_string());
            }
            // End

            // Cp Command and its arguments
            words
                if words[0].trim() == "cp"
                    && words[1].trim() == "-r"
                    && Path::new(words[2].trim()).exists()
                    && Path::new(words[3].trim()).exists() =>
            {
                use std::path::PathBuf;
                let _ = crate::cp::recrusive(
                    &PathBuf::from(words[2].trim()),
                    &PathBuf::from(words[3].trim())
                        .join(PathBuf::from(words[2].trim()).file_name().unwrap()),
                );
            }
            words
                if words[0].trim() == "cp"
                    && Path::new(words[1].trim()).exists()
                    && Path::new(words[2].trim()).exists() =>
            {
                use std::path::PathBuf;
                let _ = crate::cp::cp(
                    &PathBuf::from(words[1].trim()),
                    &mut PathBuf::from(words[2].trim()),
                );
            }
            words
                if words[0].trim() == "cp" && words[1].trim() == "-v"
                    || words[0].trim() == "cp" && words[1].trim() == "--version" =>
            {
                let _ = crate::cp::version();
            }
            words
                if words[0].trim() == "cp" && words[1].trim() == "-h"
                    || words[0].trim() == "cp" && words[1].trim() == "--help" =>
            {
                let _ = crate::cp::help();
            }

            // End

            // Non-CoreUtil Apps Executer: Start
            words if words == words => {
                let mut child = process::Command::new(words[0].trim())
                    .args(&words[1..])
                    .spawn()
                    .expect("realest real");

                let _ = child.wait();
            }
            // End
            Vec { .. } => todo!(),
        }
        input.clear();
    }
}
