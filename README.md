<h1 align="center">Music Player</h1>
<h3 align="center">A simple music player created using Python and Tkinter to play music (or any audio) from YouTube or your computer!

*Created by acatia#5378*

⚠️ **This has only been tested and built around Windows. Other operating systems may not work for this.**
</h3>

# Requirements & Dependancies
### Using the Python script directly:
* [Python 3.6](https://www.python.org/downloads/) or later
* [requirements.txt](requirements.txt) dependancies
* Other dependancies:
```
pip install --upgrade youtube-dl
pip install -r requirements.txt
pip install -U git+https://github.com/acatiadroid/pafy
```
> Note: Using https://github.com/acatiadroid/pafy as the Pafy on Pypi has not accounted for the dislikes being removed from the YouTube API which causes errors. This fork created by me prevents dislikes from being fetched entirely.

### Using the executable:
* 64 bit architecture
* Administrator privilages
* 55MB free disk space (⚠️ This **does not** account for disk space used by MP3's for the audio.)

Python **does not** need to be installed for the executable to work. The Python interpreter has been embedded into the exe file.

# Recommended
### Some other things I recommend you installing/downloading to get the best out of it:
* [Cascadia Mono](misc/CascadiaMono.ttf) font
     - To install this, go into the `misc` folder and install the font from the [CascadiaMono.ttf](misc/CascadiaMono.ttf) TrueType font file.
* [Git](https://git-scm.com/) (for git cloning -- otherwise download as ZIP from homepage and extract)

# Installation/Setup
### Using the Python script:
* Ensure [all dependancies](#requirements--dependancies) have been installed.
* Clone the repository:
```
git clone https://github.com/acatiadroid/music-player
```
* CD into it:
```
cd music-player
```
* Run the [run.py](run.py) file:
```
python run.py
```

### Using the executable:
* Download the latest release from https://github.com/acatiadroid/music-player/releases (select the `exe` file)

* Once downloaded, run the script using Administrator privilages.
    - This requires Administrator since it creates the music player files into the directory you have specified. If you don't want to grant it these privilages, use the Python script method instead.
* Once that's done that, you can delete the exe file and go into the folder that's just been made.
* Find the `exe` file that's inside of that and run that file.
    - If you intend on using the music player often, I'd recommend creating a Desktop shortcut for the executable. It will save you a lot of time.


