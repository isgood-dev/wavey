from tkinter import *
from .config import view
from .assets import cascadia
from .download import download_window

class MainWindow(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master)
        self.master = master
        self.pack(fill=BOTH, expand=1)

        self.master.bind("<space>",)

        self.paused = False
        self.song = None

        BACK_COLOUR = view("back_colour")
        FORE_COLOUR = view("fore_colour")

        
        # Labels
        
        bottom_bar = Label( # Not a class reference as it's static.
            self,
            bg=FORE_COLOUR,
            height=35,
            width=600
        ).place(x=0, y=310)
        
        self.now_playing = Label(
            self,
            text="Nothing. Let's change that!",
            bg=FORE_COLOUR,
            fg="white",
            font=cascadia
        )
        self.now_playing.place(x=5, y=320)
        
        self.download_label = Label(
            self,
            text="",
            bg=BACK_COLOUR,
            fg="white",
            font=cascadia
        )
        self.download_label.place(x=10, y=230)

        self.btn = Button(
            self,
            text="test",
            command=download_window
        )
        self.btn.place(x=10, y=10)
        # Buttons

        
        