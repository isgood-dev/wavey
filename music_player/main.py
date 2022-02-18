import datetime
import os
import threading
from mutagen.mp3 import MP3

from tkinter import *
from tkinter import messagebox
from tkinter.font import Font
from tkinter.ttk import Scrollbar

from .extensions import ScrollbarFrame
from .config import view
from .audio import Audio
from .download import download_window
from .rename import rename_window
from .settings import settings_window

BACK_COLOUR = view("back_colour")
FORE_COLOUR = view("fore_colour")

class MainWindow(Tk):
    def __init__(self):
        super().__init__()
        
        # Window attrs
        self.configure(bg=view("back_colour"))
        self.iconbitmap("music_player/Assets/musical_note.ico")
        self.wm_title("Music Player")
        self.geometry("850x600")
        self.resizable(True, True)

        self.sc = Audio()

        self.sbf = None
        self.assets = {
            "pauseplay": PhotoImage(file="music_player/Assets/pauseplay.png"),
            "cascadia": Font(size=10, family="Cascadia Mono"),
            "small_cascadia": Font(size=8, family="Cascadia Mono")
        }
        self.current_song = None

        self.bind("<space>", self.pause_or_resume)
        self.bind("<Escape>", lambda event: self.close_window)

        self.paused = False
        self.song = None
        
        # Labels
        
        bottom_bar = Label(
            self,
            bg=FORE_COLOUR,
            height=35,
            width=600
        ).place(x=0, y=500)

        sep1 = Frame(
            self,
            bg=view("accent_colour"),
            height=2,
            bd=0
        ).pack(fill=X, side=BOTTOM, pady=100)

        sep2 = Frame(
            self,
            bg=view("accent_colour"),
            width=2,
            height=500,
            bd=0
        ).place(x=198, y=0)
        
        self.now_playing = Label(
            self,
            text="Nothing is playing. Play a song by finding a song in the song list and clicking the ▶ next to it!",
            bg=FORE_COLOUR,
            fg="white",
            font=self.assets["cascadia"]
        )
        self.now_playing.place(relx=0.5, rely=0.86, anchor=CENTER)
        
        self.ispaused = Label(
            self,
            text="",
            bg=FORE_COLOUR,
            fg="white",
            font=self.assets["cascadia"]
        )
        self.ispaused.place(relx=0.5, rely=0.97, anchor=CENTER)

        self.pauseplay_button = Button(
            self,
            image=self.assets["pauseplay"],
            background=FORE_COLOUR,
            borderwidth=0,
            command=self.pause_or_resume
        )
        self.pauseplay_button.place(relx=0.499, rely=0.93, anchor=CENTER)

        self.addmusic_button = Button(
            self,
            text="Add music",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=download_window
        )
        self.addmusic_button.place(x=25, y=40)


        self.rename_file = Button(
            self,
            text="Rename a file",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=rename_window
        )
        self.rename_file.place(x=25, y=70)
        self.refresh_songlist()

        self.settings = Button(
            self,
            text="Settings",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=settings_window
        )
        self.settings.place(x=25, y=100)

    def set_np(self, text: str):
        """Sets the "now playing" label"""
        return self.now_playing.configure(text=text)

    
    def refresh_songlist(self):
        if self.sbf:
            self.sbf.destroy()

        self.sbf = ScrollbarFrame(self)
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)
        self.sbf.place(x=200, y=0)

        self.scroll_frame = self.sbf.scrolled_frame
        i = 1

        if len([f for f in os.listdir("./Audio bin/") if f.endswith(".mp3")]) == 0:
            Label(
                self.scroll_frame,
                text="Songs that you have will appear here, but you don't have any!",
                bg=view("songlist_colour"),
                fg="white",
                font=self.assets["cascadia"]
            ).grid(row=0, column=0)
            refnosongs = Button(
                self.scroll_frame,
                text="[Refresh]",
                bg=view("songlist_colour"),
                fg="white",
                font=self.assets["cascadia"],
                command=self.refresh_songlist,
                borderwidth=0
            )
            refnosongs.grid(row=0, column=1)
            return
        
        refresh = Button(
            self.scroll_frame,
            text="[Refresh]",
            bg=view("songlist_colour"),
            fg="white",
            font=self.assets["cascadia"],
            borderwidth=0,
            height=2,
            width=10,
            command=self.refresh_songlist
        )
        refresh.grid(row=0, column=1, sticky=W)
            

        for file in os.listdir("./Audio bin/"):
            if not file.endswith(".mp3"):
                continue
            
            duration = MP3("./Audio bin/" + file)
            duration = duration.info.length
            duration = str(datetime.timedelta(seconds=round(duration)))[2:]

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
                text=f"{file[:-4]}",
                bg=view("songlist_colour"),
                font=self.assets["cascadia"],
                fg="white",
            ).grid(row=i, column=1, sticky=W)

            Label(
                self.scroll_frame,
                text="        " + duration,
                bg=view("songlist_colour"),
                font=self.assets["cascadia"],
                fg="white"
            ).grid(row=i, column=2, sticky=E)

    def play(self, audiofile):
        self.current_song = audiofile[:-4]
        self.sc.play(file=str(os.getcwd()) + f".\Audio bin\{audiofile}")
        self.update_now_playing()
    def pause_or_resume(self, event=None):
        if not self.sc.song:
            if event:
                return

            return messagebox.showerror(
                title="No song selected",
                message="""You haven't selected a song from the song list.\n\nPlease find a song from the list and click the play button to the left of the song."""
            )
        
        if self.sc.paused:
            self.sc.song.resume()
            self.sc.paused = False
            self.ispaused.configure(text="")
        else:
            self.sc.song.pause()
            self.sc.paused = True
            self.ispaused.configure(text="(Paused)")

    def update_now_playing(self):
        song = self.current_song
        self.now_playing.configure(text=song)
    
    def close_window(self, event):
        ask = messagebox.askyesno(
            title="Close Music Player",
            message="Are you sure you want to close the Music Player?"
        )

        if ask:
            return self.destroy()

    def _run(self):
        """Calls the mainloop, instantiating the window"""
        self.mainloop()
