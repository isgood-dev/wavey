import os
import json
import string
import random

config_file = "./data/config.json"
song_file = "./data/songs.json"
playlist_file = "./data/playlists.json"

def parse_file(name):
    if name in ["playlist", "playlists", "p"]:
        return playlist_file
    elif name in ["song", "songs", "s"]:
        return song_file
    else:
        return config_file

def check_file():
    if not os.path.exists(config_file):
        with open(config_file, "w") as f:
            json.dump({}, f, indent=4)

def write(key: str, value, which: str):
    check_file()
    with open(parse_file(which)) as file:
        decoded = json.load(file)
    
    decoded[key] = value

    with open(parse_file(which), "w") as file:
        json.dump(decoded, file, indent=4)

def view(key: str, which: str):
    check_file()
    with open(parse_file(which)) as file:
        decoded = json.load(file)

    if decoded:
        try:
            return decoded[key]
        except KeyError:
            return False
    else:
        return False

def add_song(name: str):
    with open(song_file) as file:
        all_files = json.load(file)

    if name in all_files.values():
        return
    
    songid = ""
    for _ in range(5):
        songid = songid + random.choice(string.hexdigits())
    
    all_files[songid] = name

    with open(parse_file(song_file), "w") as file:
        json.dump(all_files, file, indent=4)

def verify_link_integrity():
    """Verifies the integrity of all files in playlists.json and songs.json, ensuring all MP3s are linked to a primary key."""
    for file in os.listdir("./data/audio"):
        if file.endswith(".mp3"):
            filename = file.split(".")

            infile = view(filename, "s")

            if not infile:
                add_song()

def get_playlists():
    with open(playlist_file) as file:
        decoded = json.load(file)
    
    return decoded

# Set default settings if not set already
if not view("volume", "c"):
    write("volume", 25, "c")
if not view("back_colour", "c"):
    write("back_colour", "#111111", "c")
if not view("fore_colour", "c"):
    write("fore_colour", "#2b2b2b", "c")
if not view("songlist_colour", "c"):
    write("songlist_colour", "#383838", "c")
if not view("accent_colour", "c"):
    write("accent_colour", "#f24646", "c")