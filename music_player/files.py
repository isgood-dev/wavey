import os
import tkinter as tk
from tkinter import messagebox
import tkinter.filedialog as filedialog
from tkinter.font import Font
from .config import view

def open_filedialog(split=False):
    file = filedialog.askopenfile(
        initialdir="./Audio bin/",
        title="Select an MP3 file",
        filetypes=(
            ("MP3 files", "*.mp3"),
            ("all files", "*.*"),
        )
    )
    if not file:
        return

    if split:
        file = file.name.split("/")
        file = file[len(file)-1]

    return file

def rename_window():
    file = open_filedialog(split=True)
        
    window = tk.Toplevel()
    window.configure(bg=view("back_colour"))
    window.geometry("450x300")
    window.wm_title("Rename a file")
    window.resizable(True, False)
    window.iconbitmap("music_player/Assets/pencil.ico")

    tk.Label(
        window,
        text="Rename a file",
        font=Font(size=13, family="Cascadia Mono"),
        fg="white",
        bg=view("back_colour")
    ).pack()

    tk.Label(
        window,
        text="Selected song",
        font=Font(size=10, family="Cascadia Mono"),
        fg="white",
        bg=view("back_colour")
    )

    selected_song = tk.Label(
        window,
        text=file,
        fg="#2ca351",
        bg=view("back_colour"),
        font=Font(size=12, family="Cascadia Mono")
    ).pack()

    tk.Label(
        window, 
        text="Enter new name: (don't include .mp3)",
        fg="white", 
        bg=view("back_colour"), 
        font=Font(size=10, family="Cascadia Mono")
    ).pack(pady=10)

    new_name = tk.Entry(
        window,
        fg="white",
        bg=view("back_colour"),
        font=Font(size=10, family="Cascadia Mono")
    )
    new_name.pack()

    def rename():
        os.rename(f"./Audio bin/{file}", "./Audio bin/" + new_name.get() + ".mp3")
        window.destroy()
        messagebox.showinfo(
            title="Success!",
            message="File renamed successfully."
        )

    btn = tk.Button(
        window,
        fg="white",
        bg=view("fore_colour"),
        text="Done",
        font=Font(size=10, family="Cascadia Mono"),
        command=rename
    )
    btn.pack(pady=20)

def delete_file():
    file = open_filedialog(split=True)

    os.remove("./Audio bin/" + file)

    messagebox.showinfo(
        title="File deleted",
        message=f"Successfully deleted \"{file}\""
    )


