use std::env;
use std::process::Command;
use std::time::Duration;
use std::thread;

extern crate rand;
use rand::Rng;

extern crate walkdir;
use walkdir::WalkDir;

const ADJUST: &'static str = "'scaled'";
const PATH: &'static str = "/Imágenes/Wallpapers";
const MINUTES: u64 = 17;

fn main() {
    let path_dir = format!("{}{}",
                            env::var("HOME")
                                .expect("Fail: variable HOME was not found "),
                            PATH);

    loop {
        let image_file = get_image_path(&path_dir);

        // println!("Image: {}", image_file);
        execute_command(&image_file);

        thread::sleep(Duration::from_secs(60 * MINUTES));
    }
}

fn get_image_path(path: &str) -> String {

    let walker: Vec<_> = WalkDir::new(path).into_iter().collect();
    
    loop {
        let rand_num =  rand::thread_rng().gen_range(1, walker.len());
        // println!("{:?} de {:?}",rand_num, walker.len()); 
        let img = &walker[rand_num];
        if let Ok(img_path) = img {
            if img_path.file_type().is_file() { // avoid directories
                let path = img_path.path().to_str();
                let path_string = path
                    .expect("Failed returning the path");

                // Validate image file in progress    
                let path_split: Vec<_> = path_string.split('/').collect();
                let filename = path_split[path_split.len() - 1];
                // println!("filename: {:?}", filename);
                if !filename.starts_with('.') {
                    // println!("is hidden file: {:?}", filename.starts_with('.'));
                    let filename_split: Vec<_> = filename.split('.').collect();
                    // println!("{:?}", filename_split);
                    let extension = filename_split[filename_split.len() - 1];
                    // println!("extension: {:?}", extension);
                    match extension {
                        "jpg" | "JPG" | "jpeg" | "JPEG" | "png" |
                        "PNG" => return path_string.to_string(),
                        _ => continue,
                    }
                } else {
                    // println!("Is a hidden file");
                    continue;
                }

            } else {
                // println!("Fail: is not a file: {:?}", img_path.path());
                continue;
            }
        }
    }
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
