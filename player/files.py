import os
import logging

import tkinter as tk
from tkinter import TclError, messagebox
import tkinter.filedialog as filedialog
from tkinter.font import Font

import player.data as data

_log = logging.getLogger("app.files").info("Files has been initialized")

class Files:
    def __init__(self):
        pass

    def open_filedialog(self, split=False):
        file = filedialog.askopenfile(
            initialdir="./data/audio/",
            title="Select an MP3 file",
            filetypes=(
                ("MP3 files", "*.mp3"),
                ("all files", "*.*"),
            )
        )
        if not file:
            return None

        if split:
            file = file.name.split("/")
            file = file[len(file)-1]

        return file

    def rename_window(self):
        file = self.open_filedialog(split=True)
        
        if not file:
            return 
            
        root = tk.Toplevel()
        root.configure(bg=data.view("back_colour", "c"))
        root.geometry("450x300")
        root.wm_attributes("-topmost", 1)
        root.wm_title("Rename a file")
        root.resizable(True, False)
        
        try:
            root.iconbitmap("player/Assets/pencil.ico")
        except TclError:
            pass
        
        tk.Label(
            root,
            text="Rename a file",
            font=Font(size=13, family="Cascadia Mono"),
            fg="white",
            bg=data.view("back_colour", "c")
        ).pack()

        tk.Label(
            root,
            text="Selected song",
            font=Font(size=10, family="Cascadia Mono"),
            fg="white",
            bg=data.view("back_colour", "c")
        )

        selected_song = tk.Label(
            root,
            text=file,
            fg="#2ca351",
            bg=data.view("back_colour", "c"),
            font=Font(size=12, family="Cascadia Mono")
        ).pack()

        tk.Label(
            root, 
            text="Enter new name: (don't include .mp3)",
            fg="white", 
            bg=data.view("back_colour", "c"), 
            font=Font(size=10, family="Cascadia Mono")
        ).pack(pady=10)

        new_name = tk.Entry(
            root,
            fg="white",
            bg=data.view("back_colour", "c"),
            font=Font(size=10, family="Cascadia Mono")
        )
        new_name.pack()

        def rename():
            os.rename(f"./data/audio/{file}", "./data/audio/" + new_name.get() + ".mp3")
            root.destroy()
            messagebox.showinfo(
                title="Success!",
                message="File renamed successfully."
            )

        btn = tk.Button(
            root,
            fg="white",
            bg=data.view("fore_colour", "c"),
            text="Done",
            font=Font(size=10, family="Cascadia Mono"),
            command=rename
        )
        btn.pack(pady=20)

    def delete_file(self):
        file = self.open_filedialog(split=True)

        if not file:
            return

        os.remove("./data/audio/" + file)

        messagebox.showinfo(
            title="File deleted",
            message=f"Successfully deleted \"{file}\""
        )