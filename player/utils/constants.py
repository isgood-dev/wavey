from tkinter.font import Font as TkFont
from tkinter import PhotoImage

class Image:
    DOWNLOAD = "./data/assets/download.ico"
    MAIN = "./data/assets/main.ico"
    PENCIL = "./data/assets/pencil.ico"
    SETTINGS = "./data/assets/settings.ico"
    UPDATE = "./data/assets/update.ico"

    LOOP = PhotoImage(file="./data/assets/loop.png")
    PAUSEPLAY = PhotoImage(file="./data/assets/pauseplay.png")
    STOP = PhotoImage(file="./data/assets/stop.png")

class Font:
    MAIN = TkFont(size=10, family="Cascadia Mono")
    SMALL = TkFont(size=8, family="Cascadia Mono")
    MEDIUM = TkFont(size=12, family="Cascadia Mono")
    LARGE = TkFont(size=14, family="Cascadia Mono")