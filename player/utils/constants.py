from tkinter.font import Font as TkFont

class Image:
    DOWNLOAD = "./data/assets/download.ico"
    MAIN = "./data/assets/main.ico"
    PENCIL = "./data/assets/pencil.ico"
    SETTINGS = "./data/assets/settings.ico"
    UPDATE = "./data/assets/update.ico"

    LOOP = "./data/assets/loop.png"
    PAUSEPLAY = "./data/assets/pauseplay.png"
    STOP = "./data/assets/stop.png"

class Font:
    MAIN = TkFont(size=10, family="Cascadia Mono")
    SMALL = TkFont(size=8, family="Cascadia Mono")
    MEDIUM = TkFont(size=12, family="Cascadia Mono")
    LARGE = TkFont(size=14, family="Cascadia Mono")