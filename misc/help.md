# Help
This file is to help you on specific functions on the music.

## Adding Music
* Click the "Add Music" button on the mid-left of the window.

![](https://user-images.githubusercontent.com/69216256/147894176-a8288055-e8b5-4c4e-9fe2-122ece569211.png)

* You can choose which download method you'd like to use:

![](https://user-images.githubusercontent.com/69216256/147894210-7b11a4f2-075b-4dc9-89f6-9634bc102176.png)

* To download a song by the name/title, just enter the name into the "Add song by name" box and click Download.

* To download a song by the YouTube URL, just enter the full URL into the "Add song by YT URL" box and click Download.


* To import music from your computer, click the "Import Music from PC" button, find the MP3 file on your computer, click it and click "Open". The MP3 file will then get copied into the `./data/audio/` directory.

## Play audio
* Click the song you want to play in the list box.

![image](https://user-images.githubusercontent.com/69216256/147937111-7c80cbdd-2cd9-4422-a5d9-362549403702.png)

* Click the play icon on the bottom middle of the screen.

![image](https://user-images.githubusercontent.com/69216256/147937243-817435ab-0453-48ec-a48a-c7d49bfcbd19.png)

Boom! You have music.

## Music Controls
### Play:
Play can be used to play music as well as resume music if music has been paused.

### Pause:
Pause can be used to stop the music temporarily. Click the pause button again to resume or click the play button.

### Stop:
The stop button stops the current song playing. You will have to replay the song from the beginning.

## Renaming files/songs
* Open the file renaming window by clicking the "Rename file" button on the mid-left.

![image](https://user-images.githubusercontent.com/69216256/147937798-1ce0ee43-f887-409c-8ddf-0b1aa223f518.png)

* Select the file you want to rename and click "Open".

![image](https://user-images.githubusercontent.com/69216256/147938003-31049fe1-202b-4d37-8696-35b231899f8f.png)

* Type the new name of the file into the input box and click "Done".

![image](https://user-images.githubusercontent.com/69216256/147938227-6c6aea8f-b247-4ca9-ac3a-a56ced27991e.png)

The file should now be renamed.

## Download randomly freezes
A bug that I identified mainly on low-tier computers is the download randomly pauses mid-download. A fix I found is to go into the terminal that is running the Python file and clicking **Ctrl + Z**. For some reason, this resumes the download process.

> ⚠️ **Do not click Ctrl + Z more than once** or it may kill the process.