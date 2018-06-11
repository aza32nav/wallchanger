use std::env;
use std::fs;
use std::process::Command;
use std::time::Duration;
use std::thread;

extern crate rand;
use rand::Rng;

const ADJUST: &'static str = "'scaled'";
const PATH: &'static str = "/Wallpapers/";
const MINUTES: u64 = 17;

fn main() {
    let path_dir = format!("{}{}",
                            env::var("HOME").unwrap(),
                            PATH);


    // println!("{}", path_dir);

    let files = fs::read_dir(path_dir).unwrap();

    let vec_files: Vec<_> = files.collect();

    // TODO: randomize the number

    loop {
    
        let rand_num =  rand::thread_rng().gen_range(1, vec_files.len());

        let file = match &vec_files[rand_num] {
            Ok(dir) => dir.path().to_str().unwrap().to_string(),
            Err(e) => format!("Error: {}", e),
        };
        // println!("{:?}", file);

        execute_command(file);

        thread::sleep(Duration::from_secs(60 * MINUTES));
    }
}

fn execute_command(img_file: String) {
    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-options")
        .arg(ADJUST)
        .spawn()
        .expect("The adjust command failed to start");

    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .arg(img_file)
        .spawn()
        .expect("The change wallpaper command fiiled to start");
}
