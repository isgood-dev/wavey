import requests
import logging

import tkinter as tk

import player.utils.data as data
import player.utils.constants as constants

UPDATE_URL = "https://raw.githubusercontent.com/acatiadroid/music-player/main/data/VERSION"

class Updater:
    def __init__(self):
        self.root = None

        self.fonts = constants.Font()

    def check_versions(self):
        req = requests.get(UPDATE_URL)

        

    def update(self):
        self.root = tk.Toplevel()
        self.root.wm_title("Update Music Player")
        self.root.configure(bg=data.view("back_colour", "c"))
        self.root.geometry("300x200")

        checking = tk.Label(
            self.root,
            text="Checking for updates...",
            bg=data.view("back_colour", "c"),
            fg="white",
            font=self.fonts.MAIN
        )
        checking.pack(pady=30)
        

    