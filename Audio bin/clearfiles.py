# A useful for removing all MP3 files from this directory.
# When executing this file, a warning will be prompted to confirm whether you want to actually delete all of the files.
# It's not recommened to run this script whilst the music player is running.

import os

confirm = input("Type \"y\" to confirm that you want to delete all files:\n-> ")

if confirm.lower() == "y":
    for file in os.listdir():
        if file.endswith(".mp3"):
            os.remove(file)
    
    print("Files removed.")
