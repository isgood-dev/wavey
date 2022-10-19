import datetime
import os
import threading
from time import sleep
from mutagen.mp3 import MP3

from tkinter import *
from tkinter import messagebox
from tkinter.font import Font

from player.window.extensions import ScrollbarFrame, HoverButton
from player.config import view, write
from player.audio import Audio
from player.files import delete_file, rename_window
from player.download import download_window
from player.settings import settings_window

BACK_COLOUR = view("back_colour")
FORE_COLOUR = view("fore_colour")

class MainWindow(Tk):
    def __init__(self):
        super().__init__()
        
        # Window attrs
        self.configure(bg=view("back_colour"))
        self.wm_title("Music Player")
        self.geometry("850x600")
        self.resizable(False, False)

        self.audio = Audio()

        self.assets = {
            "pauseplay": PhotoImage(file="player/assets/pauseplay.png"),
            "stop": PhotoImage(file="player/assets/stopicon.png"),
            # "loop": PhotoImage(file="player/assets/loopicon.png"),
            "cascadia": Font(size=10, family="Cascadia Mono"),
            "small_cascadia": Font(size=8, family="Cascadia Mono")
        }

        self.sbf = None
        self.current_song = None
        self.refresh_songlist()

        self.bind("<space>", self.audio.pause_or_resume)
        self.bind("<Escape>", self.close_window)

        try:
            self.iconbitmap("player/assets/musical_note.ico")
        except TclError:
            pass

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

        self.duration_label = Label(
            self,
            text="00:00 / 00:00",
            bg=FORE_COLOUR,
            fg="white",
            font=self.assets["cascadia"]
        )
        self.duration_label.place(x=250, y=545)
        
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
            command=self.audio.pause_or_resume,
            activebackground=FORE_COLOUR,
        )
        self.pauseplay_button.place(relx=0.499, rely=0.93, anchor=CENTER)

        self.addmusic_button = HoverButton(
            self,
            text="Add music",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=download_window,
            activebackground=BACK_COLOUR,
            activeforeground=view("accent_colour")
        )
        self.addmusic_button.place(x=25, y=40)

        self.rename_file = HoverButton(
            self,
            text="Rename a file",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=rename_window,
            activebackground=BACK_COLOUR,
            activeforeground=view("accent_colour")
        )
        self.rename_file.place(x=25, y=70)
        
        self.settings = HoverButton(
            self,
            text="Settings",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=settings_window,
            activebackground=BACK_COLOUR,
            activeforeground=view("accent_colour")
        )
        self.settings.place(x=25, y=100)

        self.delete_song = HoverButton(
            self,
            text="Delete a file",
            bg=BACK_COLOUR,
            fg="white",
            compound="left",
            font=Font(size=12, family="Cascadia Mono", weight="bold"),
            borderwidth=0,
            command=delete_file,
            activebackground=BACK_COLOUR,
            activeforeground=view("accent_colour")
        )
        self.delete_song.place(x=25, y=130)

        self.volume = Scale(
            self,
            orient=HORIZONTAL,
            variable=DoubleVar(),
            bg=FORE_COLOUR,
            fg="white",
            troughcolor=BACK_COLOUR,
            highlightthickness=0
        )
        self.volume.set(view("volume"))
        self.volume.bind("<ButtonRelease-1>", self.set_volume)
        self.volume.place(x=530, y=535)

        self.stop_button = Button(
            self,
            image=self.assets["stop"],
            background=FORE_COLOUR,
            borderwidth=0,
            command=self.audio._stop,
            activebackground=FORE_COLOUR,
        )
        self.stop_button.place(x=375, y=545)
    
    def set_np(self, text: str):
        """Sets the "now playing" label"""
        return self.now_playing.configure(text=text)

    def start_duration(self, from_paused=False, clear=False):
        if clear:
            self.audio._stop()
            self.now_playing.configure(text="Nothing is playing.")
            self.duration_label.configure(text="00:00 / 00:00")
            return
        
        if from_paused:
            mins = self.duration_data["m"]
            secs = self.duration_data["s"]
        else:
            mins = 0
            secs = 0

        playing = self.current_song
        

        while f"{str(secs).zfill(2)}:{str(mins).zfill(2)}" != self.duration:
            if self.current_song != playing:
                break

            if self.audio.paused:
                # Stop the counter from continuing as player is paused.
                self.duration_data = {
                    "s": secs,
                    "m": mins
                }
                return

            if secs == 59:
                mins += 1
                secs = 0
            else:
                secs += 1
                
            sleep(1)

            pad_zeros_mins = str(mins).zfill(2)
            pad_zeros_secs = str(secs).zfill(2)
            joined_duration = f"{pad_zeros_mins}:{pad_zeros_secs}"

            if joined_duration == self.duration:
                self.audio._stop()
                self.now_playing.configure(text="Nothing is playing.")
                self.duration_label.configure(text="00:00 / 00:00")
                return
            
            self.duration_label.configure(
                text=f"{str(pad_zeros_mins)}:{pad_zeros_secs} / {self.duration}")

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
        
        refresh = HoverButton(
            self.scroll_frame,
            text="[Refresh]",
            bg=view("songlist_colour"),
            fg="white",
            font=self.assets["cascadia"],
            borderwidth=0,
            height=2,
            width=10,
            command=self.refresh_songlist,
            activebackground=view("songlist_colour"),
            activeforeground=view("accent_colour")            
        )
        refresh.grid(row=0, column=1, sticky="w")
            

        for file in os.listdir("./Audio bin/"):
            if not file.endswith(".mp3"):
                continue

            i += 1
            
            duration = MP3("./Audio bin/" + file)
            duration = duration.info.length
            duration = str(datetime.timedelta(seconds=round(duration)))[2:]
        
            if len(file[:-4]) > 40: # Reduce length of song to prevent duration from being pushed off the screen.
                to_chop = len(file[:-4]) - 40
                file = file[:-to_chop] + "..."
            else:
                file = file[:-4] # remove ".mp3"
            btn = HoverButton(
                self.scroll_frame,
                text="▶",
                borderwidth=0,
                bg=view("songlist_colour"),
                font=Font(size=18),
                fg=view("accent_colour"),
                command=lambda file=file: self.play(file, append_queue=False),
                activebackground=view("songlist_colour"),
                activeforeground="white"
            )
            btn.grid(row=i, column=0)
            Label(
                self.scroll_frame,
                text=file,
                bg=view("songlist_colour"),
                font=self.assets["cascadia"],
                fg="white",
            ).grid(row=i, column=1, sticky="w")

            Label(
                self.scroll_frame,
                text="        " + duration,
                bg=view("songlist_colour"),
                font=self.assets["cascadia"],
                fg="white"
            ).grid(row=i, column=2, sticky="e")

    def play(self, source, append_queue=False):
        duration = MP3("./Audio bin/" + source + ".mp3")
        duration = duration.info.length
        self.duration = str(datetime.timedelta(seconds=round(duration)))[2:]

        self.current_song = source

        self.audio._play(os.getcwd() + f"/Audio bin/{source}.mp3", append_queue=append_queue)
        self.update_now_playing()

        threading.Thread(target=self.start_duration).start()
    
    def update_now_playing(self):
        song = self.current_song
        self.now_playing.configure(text=song)
    
    def set_volume(self, _):
        amount = round(self.volume.get())

        self.audio._set_vol(amount)
        
    def close_window(self, _):
        ask = messagebox.askyesno(
            title="Close Music Player",
            message="Are you sure you want to close the Music Player?"
        )

        if ask:
            return self.destroy()
    
    def _run(self):
        """Calls the mainloop, instantiating the window"""
        print("Go to https://acatiadroid.github.io/music-player/ for help")
        self.mainloop()