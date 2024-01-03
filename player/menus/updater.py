import requests
import logging

import tkinter as tk

import player.utils.data as data

UPDATE_URL = "https://raw.githubusercontent.com/acatiadroid/music-player/main/data/VERSION"

class Updater:
    def __init__(self):
        self.root = None

    def update(self):
        self.root = tk.Toplevel()
        self.root.wm_title("Update Music Player")
        self.root.configure(bg=data.view("back_colour", "c"))

        checking = tk.Label(
            self.root,
            text="Checking for updates"
        )

    