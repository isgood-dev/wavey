import os
import json
import logging

import tkinter as tk
from tkinter import TclError, messagebox
from tkinter.font import Font
from tkinter.colorchooser import askcolor

import player.data as data

# These remain cosntant as we do not want to make the colour
# of the settings window customizable
FORE_COLOUR = "#2b2b2b"
BACK_COLOUR = "#111111"

content = """All files have a corresponding unique ID associated with them. This will check all files and ensure they have an ID.
        
Why? If a file doesn't have an ID, it won't be recognised or be able to be added to playlists, etc."""

class Settings:
    def __init__(self):
        self._log = logging.getLogger("app.settings")
        self._log.info("Settings has been initialized")

    def file_sync(self):
        messagebox.showinfo(
            title="What's this?",
            message=content
        )
        with open("./data/songs.json") as file:
            stored_songs = json.load(file)
            
        for f in os.listdir("./data/audio/"):
            if f not in stored_songs.values():
                data.add_song(f)
        
        messagebox.showinfo("All done!", "File sync completed.")



    def pick_colour(self, *, bg=False, fg=False, accent=False, songlist=False):
        clr = askcolor(color=None)
        _, hex_colour = clr

        if not hex_colour:
            return

        if bg:
            data.write("back_colour", hex_colour, "c")
        if fg:
            data.write("fore_colour", hex_colour, "c")
        if accent:
            data.write("accent_colour", hex_colour, "c")
        if songlist:
            data.write("songlist_colour", hex_colour, "c")

        messagebox.showinfo("Settings Updated", "Settings have been saved!\n\nYou will need to restart the music player for the settings to be applied.")


    def settings_window(self):
        root = tk.Toplevel()
        root.wm_title("Settings")
        root.wm_attributes("-topmost", 1)
        root.configure(bg=data.view("back_colour", "c"))
        root.geometry("550x500")
        root.resizable(False, False)
        
        try:
            root.wm_iconbitmap("player/Assets/settings.ico")
        except TclError:
            pass

        # Labels

        font = Font(size=11, family="Cascadia Mono")

        tk.Label(
            root,
            font=Font(size=13, family="Cascadia Mono", weight="bold"),
            text="Settings",
            fg="white",
            bg=BACK_COLOUR
        ).grid(sticky="n")

        # Background
        
        tk.Label(
            root,
            text="Background Colour:",
            font=font,
            bg=BACK_COLOUR,
            fg="white"
        ).grid(row=3, column=0, sticky="e")

        current_bg = tk.Entry(
            root,
            bg=BACK_COLOUR,
            fg="white",
            width=10,
            font=font,
        )
        current_bg.grid(row=3, column=1)
        current_bg.insert(0, data.view("back_colour", "c"))

        change_bg = tk.Button(
            root,
            text="Change",
            command=lambda: self.pick_colour(bg=True),
            borderwidth=0,
            fg="white",
            bg=FORE_COLOUR,
            font=Font(size=10, family="Cascadia Mono")
        )
        change_bg.grid(row=3, column=2)

        # Foreground
        
        tk.Label(
            root,
            text="Foreground Colour:",
            font=font,
            bg=BACK_COLOUR,
            fg="white"
        ).grid(row=4, column=0, sticky="e")

        current_fg = tk.Entry(
            root,
            bg=BACK_COLOUR,
            fg="white",
            width=10,
            font=font,
        )
        current_fg.grid(row=4, column=1)
        current_fg.insert(0, data.view("fore_colour", "c"))

        change_fg = tk.Button(
            root,
            text="Change",
            command=lambda: self.pick_colour(fg=True),
            borderwidth=0,
            fg="white",
            bg=FORE_COLOUR,
            font=Font(size=10, family="Cascadia Mono")
        )
        change_fg.grid(row=4, column=2)

        # Accent

        tk.Label(
            root,
            text="Accent Colour:",
            font=font,
            bg=BACK_COLOUR,
            fg="white"
        ).grid(row=5, column=0, sticky="e")

        current_fg = tk.Entry(
            root,
            bg=BACK_COLOUR,
            fg="white",
            width=10,
            font=font,
        )
        current_fg.grid(row=5, column=1)
        current_fg.insert(0, data.view("accent_colour", "c"))

        change_fg = tk.Button(
            root,
            text="Change",
            command=lambda: self.pick_colour(accent=True),
            borderwidth=0,
            fg="white",
            bg=FORE_COLOUR,
            font=Font(size=10, family="Cascadia Mono")
        )
        change_fg.grid(row=5, column=2)

        # Songlist colour

        tk.Label(
            root,
            text="Song list Colour:",
            font=font,
            bg=BACK_COLOUR,
            fg="white"
        ).grid(row=6, column=0, sticky="e")

        current_songlist = tk.Entry(
            root,
            bg=BACK_COLOUR,
            fg="white",
            width=10,
            font=font,
        )
        current_songlist.grid(row=6, column=1)
        current_songlist.insert(0, data.view("songlist_colour", "c"))

        change_songlist = tk.Button(
            root,
            text="Change",
            command=lambda: self.pick_colour(accent=True),
            borderwidth=0,
            fg="white",
            bg=FORE_COLOUR,
            font=Font(size=10, family="Cascadia Mono")
        )
        change_songlist.grid(row=6, column=2)

        tk.Label(
            root,
            text="Sync files:",
            font=font,
            bg=BACK_COLOUR,
            fg="white"
        ).grid(row=7, column=0)

        sync_files = tk.Button(
            root,
            text="Synchronize",
            fg="white",
            bg=FORE_COLOUR,
            font=Font(size=10, family="Cascadia Mono"),
            command=self.file_sync
        )
        sync_files.grid(row=7, column=1)