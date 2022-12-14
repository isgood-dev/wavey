import os, json, uuid

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
    
    all_files[uuid.uuid4()] = name

    with open(parse_file(song_file), "w") as file:
        json.dump(all_files, file, indent=4)
    

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