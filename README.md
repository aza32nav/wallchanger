# WallChanger Program

This program is for change wallpaper every (n) minutes.

This working in __Cinnamon desktop__(I still haven't checked it in gnome3).

I try to simulate the random wallpaper change configuration in __xfce4__.

Program written in __rust__.

### Warning: This is a personal project and is still under development.
### Needs many improvements and more error handling.

+ This only accept images("jpg", "jpeg", "png").
+ The arguments image adjustment is still inside the source code,
  so you have to configure them before compiling.
+ The directory path and minutes configuration is there in the environmental
  variables: _WALLCHANGER_PATH and WALLCHANGER_MINUTES_ add please in the
  bashrc, zshrc or profiles
+ The program works but to avoid possible errors check its source code, it is
  quite simple.
+ To launch the software, I use the ALT+F2 option and the program binary path,
  or you can add it to the system startup. This works with an infinite loop
  and changes the wallpaper of the desktop to the minutes that are configured
  in the environment variable WALLCHANGER_MINUTES. 
+ To finish the program you can use ```ps -aux | grep wallchanger``` and 
  then ```kill <Id-Process>```.
+ Use walkdir crate to traverse the directory and its subdirectories.

__TODO: Improvements__

+ Check a configuration file approach with (Minutes, image adjustment, and directory path).
+ Improve error handling.
+ Check a concurrent approach.
+ Improve, if possible, the generation of random numbers.
+ Check the posibility of the creation of a Restful Api or server with the
  ability of get the file name of the current image and change configuration values.

## License

This source code is released under MIT License.
