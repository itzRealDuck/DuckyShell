#[macro_use]
extern crate structopt;

use crossterm::{
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use io::Read;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{stdout, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn cd(input: &mut String) {
    env::set_current_dir(
        //    "/home/".to_string()
        //      + &whoami::realname().to_owned().to_lowercase()
        //    + "/"
        &input,
    );

    if input == "/" {
        env::set_current_dir("/");
    }
}

fn ls(dir: &Path) -> Result<(), Box<dyn Error>> {
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

fn GetFilePerms() -> io::Result<()>{

let path = fs::read_dir(".")?; 


 for pathy in path { 

let entry = pathy?;  
let actualpath = entry.path();

   if actualpath.is_file() { 

    let metadata = actualpath.metadata();

        if metadata.expect("jitty").permissions().readonly() == true { 

        println!("read {:?}", actualpath);


            }    
            else { 
            println!("write {:?}", actualpath);

            }



        }     
    }  

    Ok(())

}

fn main() {
    let mut opt = Opt::from_args();
    let mut input = String::new();
    let mut newstr = String::new();
    println!(
        "Release 0.1 Alpha of RustyShell,Welcome {} ",
        whoami::realname()
    );

    loop {
        io::stdin().read_line(&mut input).unwrap();

        let words: Vec<&str> = input.split_whitespace().collect();
        match words {
            (words) if words[0].trim() == "ls".trim() && words.get(1).is_none() => {
                if let Err(ref e) = ls(Path::new(".")) {
                    println!("{}", e);
                    process::exit(1);
                }
            }
            (words)
                if words[0].trim() == "ls".trim()
                    && words[1] == words[1]
                    && Path::new(words[1].trim()).exists() =>
            {
                let mut path = Path::new(words[1].trim());

                if path.exists() {
                    if let Err(ref e) = ls(path) {
                        println!("{}", e);
                        process::exit(1);
                    }
                } else {
                    println!("path not exist ");
                }
            }
            (words) if words[0] == "cd".trim() && words.get(1).is_none() => {
                // let whoamiString = String::from(whoami::realname());

                let mut input23 = String::from(
                    "/home/".to_string() + &whoami::realname().to_owned().to_lowercase(),
                );
                cd(&mut input23);
            }
            (words)
                if words[0].trim() == "cd".trim()
                    && words[1] == words[1]
                    && Path::new(words[1].trim()).exists() =>
            {
                let mut inputy = String::from(words[1].trim());

                cd(&mut inputy);

                inputy.clear()
            }
            (words) if words[0].trim() == "clear".trim() => {
                let mut out = stdout();
                out.queue(Hide).unwrap();
                out.queue(Clear(ClearType::All)).unwrap();
                out.queue(MoveTo(0, 0)).unwrap();
                out.flush().unwrap();
            }
            (words) if words[0].trim() == "exit".trim() => {
                std::process::exit(0);
            }
            (words)
                if words[0].trim() == "cat".trim()
                    && words[1] == words[1]
                    && Path::new(words[1].trim()).exists() =>
            {
                let mut pathy = Path::new(words[1].trim());

                let mut file_open = File::open(pathy);

                let mut file_container = String::new();

                file_open
                    .expect("idk what dis does")
                    .read_to_string(&mut file_container);

                println!("{}", file_container);
            }
            (words) if words[0].trim() == "echo" && words[1] == words[1] && words[1] != "$PATH" => {
                println!("{}", words[1]);
            }
            (words) if words[0].trim() == "echo" && words[1] == "$PATH" => {
                let path = match env::current_dir() {
                    Ok(y) => y,
                    Err(y) => panic!("nuh uh"),
                };
            

                println!("{}", path.display());
            }

            (words) if words[0].trim() == "ls" && words[1] == "-l" => { 

            GetFilePerms();

            }

            Vec { .. } => todo!(),
        }
        input.clear();
    }
}
