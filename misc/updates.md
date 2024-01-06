# 06/01/2024
* Finally implemented an updater.
    - This works by getting the current version on the `main` branch and comparing it to the local version. If there's a mismatch, it will run a `git pull`.

# 04/01/2024
* Major code restructure and clean up.

# 05/12/2023
This is the first update in a while. In the time between the last update, many essential functionalities of the music player stopped working. This addresses those issues.
* All downloaded audio will now download into `data/audio/` instead of `Audio bin/`.
* Sub menus will be pushed as the topmost window now.
* Implemented song progress timer as separate class instead of having it entirely done in the main class.
* `requirements.txt` hadn't been updated for a while and included loads of old, outdated dependancies. This has been fixed.
* Replaced Pafy with PyTube. In reality I should've done this sooner since I've had issue after issue with Pafy. It seems that the Pafy library had become neglected and no longer worked.
    - For example, Pafy still doesn't account for YouTube removing the dislike count and still tries to grab it, causing an error.
    - As a result, downloading audio has become *a lot* quicker.
        - If it was downloading a high quality song, it would take minutes, whereas this only takes seconds.
* Cleaned up a lot of code.

# 22/10/2022 (pre-release)
* Using Pyglet for audio playing.
* Added Music Player installer.
* Massive cleanup of codespace.
* All imports are now absolute to prevent circular imports.

# 12/03/2022 (pre-release)
* Added file deletion
* Added stop button
* Added song duration timer (likely to be buggy)
* Moved database files to a special `data/` directory.

# 20/02/2022 (pre-release)
* Changed default accent colour (#389fc1 → #f24646) to follow colour scheme of the website.
* Completed settings page for now.
    - Colour scheme of the music player can be customized entirely.
    - Added settings icon for the window.
* Buttons now change colour when hovered over.
* Replaces illegal file characters with underscores.

# 19/02/2022
* New song list
    - Displays duration of song
    - Refresh button
    - If no songs, a message will show you have no songs
* Escape now closes the window
* Added Settings (incomplete)
    - You can customize music player's colour scheme.
* Embedded into an executable to allow the user to use the music player through an `exe`.
* Restructured codebase - code has been separated into different files to prevent clutter.
* Added accent colours.
* New website for listing documentation (incomplete)
* Prints message in terminal with the link to the website

# 18/01/2022
* Space bar now pauses/plays the music

# 16/01/2022
* Added volume slider
* Volumes are stored between sessions using a JSON file.

# 03/01/2022
* File renaming (seperate window for doing this)
* Play button can now be used to resume a paused song instead if clicking the pause button again.
* Providing an invalid YouTube URL will now notify the user that an invalid URL has been provided.

# 18/12/2021
* Complete UI redesign with a darker colour scheme.
* Seperate window for adding music.
* Ability to import music files from elsewhere on your computer into the Audio Bin directory.
* File extensions and non mp3 files are no longer shown on the song selection box.
* New bottom bar that holds the buttons and the name of the song that's playing.
* Redesigned play, pause & stop button and moved them to the bottom.
* Window is no longer resizable.
* New font — "Cascadia Mono"

(All previous updates have not been recorded)