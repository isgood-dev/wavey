# Music Player
This Music player allows you to download music (or any audio really) from YouTube and play it! This is useful if you don't like ads. This Music Player was made as a project and I will be updating it in the future too.

All music that you download gets saved into the `/Audio bin/` directory.

Created by acatia#0001

## Latest changes
* Switched from pygame to VLC for playing audio files.
* Uses multithreading for downloading songs in the background.

Soon:
* Option to delete selected song index.
* Volume control.

## Setup
To run this youself, it's pretty simple. You will need to install all of the dependencies:
```
pip install --upgrade youtube-dl
pip install moviepy
pip install python-vlc
pip install youtube-search-python
pip install pafy
```
You will only have to do this once!

Once that is done, you will need to clone the GitHub repository:
```py
git clone https://github.com/acatiadroid/music-player
```

Then, you will need to run it:
```py
python mp.py
```

### How to add audio
You can either: 
* Search song by the name:

Example: `Numb linkin park`

![alt-text](https://cdn.discordapp.com/attachments/763535909433376788/848927783199178832/unknown.png)

* Use YouTube URL:

Example: `https://www.youtube.com/watch?v=kXYiU_JCYtU`

![alt-text](https://cdn.tixte.com/uploads/acatia.needs.rest/kpcp4as999a.png)

> ⚠️ NOTE: the larger the YouTube video is, the longer it is going to take to download. Killing the script whilst it's downloading a song will cause the song to not fully download, making it unplayable.

**PRO TIP: Use the word "lyrics" when searching for a song as song music videos have long intro's. This will also make it download quicker.**

### How to play the audio
Click on the song name in the black box in the middle of the screen and click the play button.

### How to change the volume
Coming soon.

### How to seek through the song
Coming soon.

> ⚠️ Warning: If the song title has characters that are illegal to file names (<>?|/\ . etc), this will break the downloading process. 