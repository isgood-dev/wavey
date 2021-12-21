# Music Player
This Music player allows you to download music (or any audio really) from YouTube and play it! This is useful if you don't like ads. This Music Player was made as a project and I will be updating it in the future too.

All music that you download gets saved into the `/Audio bin/` directory.

Created by acatia#5378

## Latest changes
18/12/2021
* Complete UI redesign with a darker colour scheme.
* Seperate window for adding music.
* Ability to import music files from elsewhere on your computer into the Audio Bin directory.
* File extensions and non mp3 files are no longer shown on the song selection box.
* New bottom bar that holds the buttons and the name of the song that's playing.
* Redesigned play, pause & stop button and moved them to the bottom.
* Window is no longer resizable.
* New font — "Cascadia Mono"


## Setup
To run this youself, it's pretty simple. You will need to install all of the dependencies:
```
pip install --upgrade youtube-dl
pip install -r requirements.txt
pip install -U git+https://github.com/acatiadroid/pafy
```
> NOTE: Using https://github.com/acatiadroid/pafy because pafy attempts to provide stats for YouTube video dislikes which is no longer offered by YouTube's API. The fork I made blocks the library from attempting to request that information from YouTube. (temporary fix until permanent fix is added to `pafy`.)

You will only have to do this once!

FFMPEG is also required!

Once that is done, you will need to clone the GitHub repository:
```
git clone https://github.com/acatiadroid/music-player
```

Then, you will need to run it:
```py
python mp.py
```

### How to add audio
You can either: 
* Add songs by their name:

![img](https://acatia.wants-to.party/v9Gdb7ewL5.png)


* Add songs by the YouTube URL:

![img](https://acatia.wants-to.party/r6wdku777z.png)

> ⚠️ NOTE: the larger the YouTube video is, the longer it is going to take to download. Killing the script whilst it's downloading a song will cause the song to not fully download, making it unplayable.

**PRO TIP: Use the word "lyrics" when searching for a song as song music videos have long intro's. This will also make it download quicker.**

### How to play the audio
Click on the song name in the black box in the middle of the screen and click the play button.

> ⚠️ Warning: If the song title has characters that are illegal to file names (<>?|/\ . etc), this will break the downloading process. 


### Demo
A video to demonstrate how to add audio and play it.

[![](https://res.cloudinary.com/marcomontalbano/image/upload/v1640100353/video_to_markdown/images/video--c152991c1be81f929e8918da25a49863-c05b58ac6eb4c4700831b2b3070cd403.jpg)](https://acatia.wants-to.party/musicplayerdemo.mp4 "")