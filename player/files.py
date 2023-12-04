import os
import tkinter as tk
from tkinter import TclError, messagebox
import tkinter.filedialog as filedialog
from tkinter.font import Font

import player.data as data

def open_filedialog(split=False):
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

def rename_window():
    file = open_filedialog(split=True)
    
    if not file:
        return 
        
    window = tk.Toplevel()
    window.configure(bg=data.view("back_colour", "c"))
    window.geometry("450x300")
    window.wm_title("Rename a file")
    window.resizable(True, False)
    
    try:
        window.iconbitmap("player/Assets/pencil.ico")
    except TclError:
        pass
    
    tk.Label(
        window,
        text="Rename a file",
        font=Font(size=13, family="Cascadia Mono"),
        fg="white",
        bg=data.view("back_colour", "c")
    ).pack()

    tk.Label(
        window,
        text="Selected song",
        font=Font(size=10, family="Cascadia Mono"),
        fg="white",
        bg=data.view("back_colour", "c")
    )

    selected_song = tk.Label(
        window,
        text=file,
        fg="#2ca351",
        bg=data.view("back_colour", "c"),
        font=Font(size=12, family="Cascadia Mono")
    ).pack()

    tk.Label(
        window, 
        text="Enter new name: (don't include .mp3)",
        fg="white", 
        bg=data.view("back_colour", "c"), 
        font=Font(size=10, family="Cascadia Mono")
    ).pack(pady=10)

    new_name = tk.Entry(
        window,
        fg="white",
        bg=data.view("back_colour", "c"),
        font=Font(size=10, family="Cascadia Mono")
    )
    new_name.pack()

    def rename():
        os.rename(f"./data/audio/{file}", "./data/audio/" + new_name.get() + ".mp3")
        window.destroy()
        messagebox.showinfo(
            title="Success!",
            message="File renamed successfully."
        )

    btn = tk.Button(
        window,
        fg="white",
        bg=data.view("fore_colour", "c"),
        text="Done",
        font=Font(size=10, family="Cascadia Mono"),
        command=rename
    )
    btn.pack(pady=20)

def delete_file():
    file = open_filedialog(split=True)

    if not file:
        return

    os.remove("./data/audio" + file)

    messagebox.showinfo(
        title="File deleted",
        message=f"Successfully deleted \"{file}\""
    )