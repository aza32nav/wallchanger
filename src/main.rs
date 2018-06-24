use std::env;
use std::fs;
use std::process::Command;
use std::time::Duration;
use std::thread;
use std::io;

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

    loop {
        let image_file = get_image_path(&path_dir);
        let print_ima_path = &image_file.unwrap();

        // println!("the image is: {}", print_ima_path);
        execute_command(&print_ima_path);

        thread::sleep(Duration::from_secs(60 * MINUTES));
    }
}

fn get_image_path(path_dir: &str) -> io::Result<String>{

    let files = fs::read_dir(path_dir)?;

    let vec_files: Vec<_> = files.collect();

    let rand_num =  rand::thread_rng().gen_range(1, vec_files.len());

    let file = match &vec_files[rand_num] {
        Ok(dir) => dir.path().to_str().unwrap().to_string(),
        Err(e) => format!("Error: {}", e),
    };

    Ok(file)
}

fn execute_command(img_file: &str) {
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
