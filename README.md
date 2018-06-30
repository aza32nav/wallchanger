# WallChanger Program

This program is for change wallpaper every (n) minutes.

This working only in gnome3.

I try to simulate the random wallpaper change configuration in xfce4.

Program written in rust.

## Warning: This is a personal project and is still under development.
## Needs many improvements and more error handling.

+ This only accept images("jpg", "jpeg", "png").
+ The arguments minutes, image adjustment, and the directory path is still
  inside the source code, so you have to configure them before compiling.
+ The program works but to avoid possible errors check its source code, it is quite
  simple.
+ To launch the software, I use the ALT+F2 option and the program binary path, or you
  can add it to the system startup. This works with an infinite loop and changes the 
  wallpaper of the desktop to the minutes that are configured in the const MINUTES. 
+ To finish the program you can use ```ps -aux | grep wallchanger``` and 
  then ```kill <Id-Process>```.
+ Use walkdir crate to traverse the directory and its subdirectories.

__TODO: Improvements__

+ Use a command-line arguments approach or use a configuration file approach with
  (Minutes, image adjustment, and directory path).
+ Improve error handling.
+ Check a concurrent approach.
+ Improve, if possible, the generation of random numbers.
+ Check the posibility of the creation of a Restful Api or server with the
  ability of get the file name of the current image and change configuration values.

## License

This source code is released under MIT License.
