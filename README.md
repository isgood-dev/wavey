# Music Player
This Music player allows you to download music (or any audio really) from YouTube and play it! This is useful if you don't like ads. This Music Player was made as a project and I will be updating it in the future too.

All music that you download gets saved into the `/Audio bin/` directory.

Created by acatia#5378

## Latest changes
Check [updates.mdown](updates.mdown) for the latest updates.


## Setup
To run this youself, it's pretty simple. You will need to install all of the dependencies:
```
pip install --upgrade youtube-dl
pip install -r requirements.txt
pip install -U git+https://github.com/acatiadroid/pafy
```
> NOTE: Using https://github.com/acatiadroid/pafy because pafy attempts to provide stats for YouTube video dislikes which is no longer offered by YouTube's API. The fork I made blocks the library from attempting to request that information from YouTube. (temporary fix until permanent fix is added to `pafy`.)

You will only have to do this once!

Once that is done, you will need to clone the GitHub repository:
```
git clone https://github.com/acatiadroid/music-player
```

Then, you will need to run it:
```py
python mp.py
```

## Help
Check out [help.mdown](help.mdown) for help on specific things.

## Roadmap
* Progress bar
* Remove Listbox and replace with individual buttons for aesthetics
