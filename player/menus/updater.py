import requests
import subprocess
import threading
import time
import sys
import os

import tkinter as tk
from tkinter import messagebox

import player.utils.data as data
import player.utils.constants as constants

UPDATE_URL = "https://raw.githubusercontent.com/acatiadroid/music-player/main/data/VERSION"
RUN_SCRIPT = "./player/utils/update_runner.py"

class Updater:
    def __init__(self):
        self.root = None

        self.fonts = constants.Font()

    def test_network_connectivity(self):
        try:
            req = requests.head("https://example.com", timeout=5)
            req.raise_for_status()

            return True
        except:
            return False

    def check_versions_align(self):        
        req = requests.get(UPDATE_URL)
        
        self.latest = req.text
        self.current = data.get_local_version()

        if self.latest == self.current:
            return True
        
        return False

    def ask_update(self):
        ask = messagebox.askyesno(
            "Start Update?",
            "The music player will close. Are you sure?"
        )

        if not ask:
            return
        
        self.start_update_script()

    def start_update_script(self):
        # start update script
        command = [sys.executable, RUN_SCRIPT]
        subprocess.Popen(command, shell=True)

        # kill current process
        current_pid = os.getpid()
        os.kill(current_pid, 9)

    def update(self):
        self.root = tk.Toplevel()
        self.root.wm_title("Update Music Player")
        self.root.configure(bg=data.view("back_colour", "c"))
        self.root.geometry("300x200")

        def start_check():
            checking = tk.Label(
                self.root,
                text="Checking for updates...",
                bg=data.view("back_colour", "c"),
                fg="white",
                font=self.fonts.MAIN
            )
            checking.pack(pady=20)
            
            test = self.test_network_connectivity()
            if not test:
                checking.configure(text="No internet connection.")
                return
            
            check_versions = self.check_versions_align()

            time.sleep(1)

            if check_versions:
                checking.configure(text="Music Player is up-to-date!")
            else:
                checking.configure(text=f"A new version is available.\nCurrent version: {self.current}\nAvailable version: {self.latest}")
                start_update = tk.Button(
                    self.root,
                    text="Start Update",
                    command=self.ask_update
                )
                start_update.pack(pady=20)

        threading.Thread(target=start_check).start()

        
        

    