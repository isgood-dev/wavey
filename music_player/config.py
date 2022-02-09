"""
A JSON wrapper for window configurations.
"""

import os, json

def check_file():
    if not os.path.exists("config.json"):
        with open("config.json", "w") as f:
            json.dump({}, f, indent=4)

def write(key: str, value):
    check_file()
    with open("config.json") as file:
        decoded = json.load(file)
    
    decoded[key] = value

    with open("config.json", "w") as file:
        json.dump(decoded, file, indent=4)

def view(key: str):
    check_file()
    with open("config.json") as file:
        decoded = json.load(file)

    if decoded:
        try:
            return decoded[key]
        except KeyError:
            return False
    else:
        return False

if not view("volume"):
    write("volume", 100)
if not view("back_colour"):
    write("back_colour", "#111111")
if not view("fore_colour"):
    write("fore_colour", "#2b2b2b")
if not view("songlist_colour"):
    write("songlist_colour", "#383838")
if not view("accent_colour"):
    write("accent_colour", "#389fc1")