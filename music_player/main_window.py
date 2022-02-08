import os
from tkinter import *
from tkinter.font import Font

from .extensions import ScrollbarFrame
from .config import view
from .audio import Audio
from .download import *
from .rename import rename_window

class MainWindow(Tk):
    def __init__(self):
        super().__init__()
        
        # Window attrs
        self.configure(bg=view("back_colour"))
        self.iconbitmap("music_player/Assets/musical_note.ico")
        self.wm_title("Music Player")
        self.geometry("850x600")
        self.resizable(False, False)

        self.sc = Audio()

        self.sbf = None
        self.assets = {
            "play": PhotoImage(file="music_player/Assets/playicon.png"),
            "pause": PhotoImage(file="music_player/Assets/pauseicon.png"),
            "pauseplay": PhotoImage(file="music_player/Assets/pauseplay.png"),
            "pencil": PhotoImage(file="music_player/Assets/pencil.png"),
            "cascadia": Font(size=10, family="Cascadia Mono"),
            "small_cascadia": Font(size=8, family="Cascadia Mono")
        }
        self.play_small = PhotoImage(file='music_player/Assets/playicon_small.png'),
        self.playicon = PhotoImage(file="music_player/Assets/playicon.png"),
        self.pause = PhotoImage(file="music_player/Assets/pauseicon.png"),
        self.pauseplay = PhotoImage(file="music_player/Assets/pauseplay.png")
        self.cascadia = Font(size=10, family="Cascadia Mono"),
        self.smaller_cascadia = Font(size=8, family="Cascadia Mono")


        self.bind("<space>", lambda event: print("hi"))

        self.paused = False
        self.song = None

        BACK_COLOUR = view("back_colour")
        FORE_COLOUR = view("fore_colour")

        
        # Labels
        
        bottom_bar = Label(
            self,
            bg=FORE_COLOUR,
            height=35,
            width=600
        ).place(x=0, y=500)
        
        self.now_playing = Label(
            self,
            text="test",
            bg=FORE_COLOUR,
            fg="white",
            font=self.assets["cascadia"]
        )
        self.now_playing.place(relx=0.5, rely=0.86, anchor=CENTER)
        
        self.download_label = Label(
            self,
            text="",
            bg=BACK_COLOUR,
            fg="white",
            font=self.assets["cascadia"]
        )
        self.download_label.place(x=10, y=230)

        self.refresh_button = Button(
            self,
            text="refresh",
            command=self.refresh_songlist
        )
        self.refresh_button.place(x=30, y=30)


        self.pauseplay_button = Button(
            self,
            image=self.assets["pauseplay"],
            background=FORE_COLOUR,
            borderwidth=0,
            command=self.change_pauseplay
        )
        self.pauseplay_button.place(relx=0.499, rely=0.93, anchor=CENTER)

        self.renamefile = Button(
            self,
            text=" Rename file",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=14, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=rename_window,
            image=self.assets["pencil"]
        )
        self.renamefile.place(x=14, y=60)
    
    def set_np(self, text: str):
        """Sets the "now playing" label"""
        return self.now_playing.configure(text=text)

    
    def refresh_songlist(self):
        if self.sbf:
            self.sbf.destroy()

        self.sbf = ScrollbarFrame(self)
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)
        self.sbf.place(x=250, y=0)

        self.scroll_frame = self.sbf.scrolled_frame
        i = 0
        for file in os.listdir("./Audio bin/"):
            if not file.endswith(".mp3"):
                continue

            i += 1
            btn = Button(
                self.scroll_frame,
                text="▶",
                borderwidth=0,
                bg=view("songlist_colour"),
                font=Font(size=18),
                fg="white",
                command=lambda file=file: self.play(file)
            )
            btn.grid(row=i, column=0)
            Label(
                self.scroll_frame,
                text=file[:-4],
                bg=view("songlist_colour"),
                font=self.cascadia,
                fg="white",
            ).grid(row=i, column=1, sticky=W)

    def play(self, audiofile):
        print(str(os.getcwd()) + f"\Audio bin\{audiofile}")
        self.sc.play(file=str(os.getcwd()) + f".\Audio bin\{audiofile}")
    
    # def pauseplay_button(self):
    #     if not self.sc.song:
    #         return
        
    #     self.change_pauseplay()

        
    def change_pauseplay(self):
        if not self.sc.song:
            return
        if not self.sc.paused:
            self.pauseplay_button.configure(image=self.playicon)
        else:
            self.pauseplay_button.configure(image=self.pause)

    def _run(self):
        """Calls the mainloop, instantiating the window"""
        self.mainloop()
