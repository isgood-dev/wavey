import tkinter as tk
from tkinter.font import Font

import player.data as data

def check_updates():       
    window = tk.Toplevel()
    window.configure(bg=data.view("back_colour", "c"))
    window.geometry("450x300")
    window.wm_title("Check for updates")
    window.resizable(True, False)
    
    try:
        window.iconbitmap("player/Assets/update.ico")
    except tk.TclError:
        pass
    
    tk.Label(
        window,
        text="Feature not yet released. Check back soon!",
        font=Font(size=13, family="Cascadia Mono"),
        fg="white",
        bg=data.view("back_colour", "c")
    ).pack()

    