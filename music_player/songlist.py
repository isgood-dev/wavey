import os
from tkinter import *
from .main_window import MainWindow
from .assets import play_icon_small
from .config import view
from .music import Song
from .scrollbarframe import ScrolledWindow
mw = MainWindow
songs = {}

sbf = (mw)
sbf.place(x=60, y=60)

frame = sbf.scrolled_frame

i = 0
for file in os.listdir("./Audio bin/"):
    i += 1  
    btn = Button(
        frame,
        image=play_icon_small,
        bg=view("back_colour"),
        command=lambda song=file: Song().play("./Audio bin/" + str(file))
    )
    btn.grid(row=i, column=0)

    Label(
        frame,
        text=file[:-4],
        background=sbf.scrolled_frame.cget("bg")
    ).grid(row=i, column=1)


