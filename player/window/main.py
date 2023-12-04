import datetime
import os
import threading
from time import sleep
from mutagen.mp3 import MP3

from tkinter import *
from tkinter import messagebox
from tkinter.font import Font

import player.window.widgets as widgets
import player.data as data
import player.audio as audio
import player.files as files
import player.timer as timer
import player.download as download
import player.settings as settings
import player.updater as updater
import player.playlist as playlist

BACK_COLOUR = data.view("back_colour", "c")
FORE_COLOUR = data.view("fore_colour", "c")

class MainWindow(Tk):
    def __init__(self):
        super().__init__()
        
        # Window attrs
        self.configure(bg=data.view("back_colour", "c"))
        self.wm_title("Music Player")
        self.geometry("850x600")
        self.resizable(False, False)

        self.audio = audio.Audio()
        self.timer = None

        self.assets = {
            "pauseplay": PhotoImage(file="player/Assets/pauseplay.png"),
            "stop": PhotoImage(file="player/Assets/stopicon.png"),
            "cascadia": Font(size=10, family="Cascadia Mono"),
            "small_cascadia": Font(size=8, family="Cascadia Mono"),
            # "loop": PhotoImage(file="player/Assets/loopicon.png"),
        }

        self.sbf = None
        self.current_song = None
        self.refresh_songlist()

        self.bind("<space>", self.audio.pause_or_resume)
        self.bind("<Escape>", self.close_window)

        try:
            self.iconbitmap("player/Assets/musical_note.ico")
        except TclError:
            pass

        bottom_bar = Label(self, bg=FORE_COLOUR, height=35, width=600).place(x=0, y=500)
        sep1 = Frame(self, bg=data.view("accent_colour", "c"), height=2, bd=0).pack(fill=X, side=BOTTOM, pady=100)
        sep2 = Frame(self, bg=data.view("accent_colour", "c"), width=2, height=500, bd=0 ).place(x=198, y=0)

        self.duration_label = Label(self, text="00:00 / 00:00", bg=FORE_COLOUR, fg="white", font=self.assets["cascadia"])
        self.duration_label.place(x=250, y=545)
        
        self.now_playing = Label(self, text="Nothing is playing. Play a song by finding a song in the song list and clicking the ▶ next to it!", bg=FORE_COLOUR, fg="white", font=self.assets["cascadia"])
        self.now_playing.place(relx=0.5, rely=0.86, anchor=CENTER)
        
        self.ispaused = Label(self, text="", bg=FORE_COLOUR, fg="white", font=self.assets["cascadia"])
        self.ispaused.place(relx=0.5, rely=0.97, anchor=CENTER)

        self.pauseplay_button = Button(self, image=self.assets["pauseplay"], background=FORE_COLOUR, borderwidth=0, command=self.audio.pause_or_resume, activebackground=FORE_COLOUR)
        self.pauseplay_button.place(relx=0.499, rely=0.93, anchor=CENTER)

        self.addmusic_button = widgets.HoverButton(self, text="Add music", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=download.download_window, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.addmusic_button["font"] = Font(size=12, family="Cascadia Mono", weight="bold")
        self.addmusic_button.place(x=25, y=40)

        self.rename_file = widgets.HoverButton(self, text="Rename a file", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=files.rename_window, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.rename_file["font"] = Font(size=12, family="Cascadia Mono", weight="bold")
        self.rename_file.place(x=25, y=70)
        
        self.settings = widgets.HoverButton(self, text="Settings", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=settings.settings_window, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.settings["font"] = Font(size=12, family="Cascadia Mono", weight="bold")
        self.settings.place(x=25, y=100)

        self.delete_song = widgets.HoverButton(self, text="Delete a file", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=files.delete_file, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.delete_song["font"] = Font(size=12, family="Cascadia Mono", weight="bold")
        self.delete_song.place(x=25, y=130)

        self.myplaylists = widgets.HoverButton(self, text="My Playlists", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=playlist.show_playlists, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.myplaylists["font"] = Font(size=12, family="Cascadia Mono", weight="bold")
        self.myplaylists.place(x=25, y=160)

        self.update = widgets.HoverButton(self, text="Check for updates", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=updater.check_updates, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.update["font"] = Font(size=12, family="Cascadia Mono", weight="bold")
        self.update.place(x=15, y=460)

        self.volume = Scale(self, orient=HORIZONTAL, variable=DoubleVar(), bg=FORE_COLOUR, fg="white", troughcolor=BACK_COLOUR, highlightthickness=0)
        self.volume.set(data.view("volume", "c"))
        self.volume.bind("<ButtonRelease-1>", self.set_volume)
        self.volume.place(x=530, y=535)

        self.stop_button = Button(self, image=self.assets["stop"], background=FORE_COLOUR, borderwidth=0, command=self.audio._stop, activebackground=FORE_COLOUR)
        self.stop_button.place(x=375, y=545)
    
    def set_np(self, text: str):
        """Sets the "now playing" label"""
        return self.now_playing.configure(text=text)

    def refresh_songlist(self):
        if self.sbf:
            self.sbf.destroy()

        self.sbf = widgets.ScrollbarFrame(self)
        self.grid_rowconfigure(0, weight=1)
        self.grid_columnconfigure(0, weight=1)
        self.sbf.place(x=200, y=0)

        self.scroll_frame = self.sbf.scrolled_frame
        i = 1

        if len([f for f in os.listdir("./data/audio/") if f.endswith(".mp3")]) == 0:
            Label(
                self.scroll_frame,
                text="Songs that you have will appear here, but you don't have any!",
                bg=data.view("songlist_colour", "c"),
                fg="white",
                font=self.assets["cascadia"]
            ).grid(row=0, column=0)
            
            refnosongs = Button(
                self.scroll_frame,
                text="[Refresh]",
                bg=data.view("songlist_colour", "c"),
                fg="white",
                font=self.assets["cascadia"],
                command=self.refresh_songlist,
                borderwidth=0
            )
            refnosongs.grid(row=0, column=1)
            return
        
        refresh = widgets.HoverButton(
            self.scroll_frame,
            text="[Refresh]",
            bg=data.view("songlist_colour", "c"),
            fg="white",
            font=self.assets["cascadia"],
            borderwidth=0,
            height=2,
            width=10,
            command=self.refresh_songlist,
            activebackground=data.view("songlist_colour", "c"),
            activeforeground=data.view("accent_colour", "c")            
        )
        refresh.grid(row=0, column=1, sticky="w")
            

        for file in os.listdir("./data/audio/"):
            if not file.endswith(".mp3"):
                continue

            i += 1
            
            duration = MP3("./data/audio/" + file)
            duration = duration.info.length
            duration = str(datetime.timedelta(seconds=round(duration)))[2:]
        
            if len(file[:-4]) > 40: # Reduce length of song name to prevent duration from being pushed off the screen.
                to_chop = len(file[:-4]) - 40
                file = file[:-to_chop] + "..."
            else:
                file = file[:-4] # remove ".mp3"
            btn = widgets.HoverButton(
                self.scroll_frame,
                text="▶",
                borderwidth=0,
                bg=data.view("songlist_colour", "c"),
                font=Font(size=18),
                fg=data.view("accent_colour", "c"),
                command=lambda file=file: self.play(file, append_queue=False),
                activebackground=data.view("songlist_colour", "c"),
                activeforeground="white"
            )
            btn.grid(row=i, column=0)
            Label(
                self.scroll_frame,
                text=file,
                bg=data.view("songlist_colour", "c"),
                font=self.assets["cascadia"],
                fg="white",
            ).grid(row=i, column=1, sticky="w")

            Label(
                self.scroll_frame,
                text="        " + duration,
                bg=data.view("songlist_colour", "c"),
                font=self.assets["cascadia"],
                fg="white"
            ).grid(row=i, column=2, sticky="e")
    
    def play(self, source, append_queue=False):
        duration = MP3("./data/audio/" + source + ".mp3")
        duration = duration.info.length
        self.duration = str(datetime.timedelta(seconds=round(duration)))[2:]

        self.current_song = source

        self.audio._play(os.getcwd() + f"/data/audio/{source}.mp3", append_queue=append_queue)
        self.update_now_playing()

        print(1)
        self.timer = timer.Timer(self.duration)
        print(2)
        thread = threading.Thread(target=self.timer.start)
        thread.daemon = True
        thread.start()

        while self.timer.is_active:
            if self.audio.paused:
                continue
            
            if self.timer.end_reached:
                self.audio._stop()
                self.now_playing.configure(text="Nothing is playing.")
                self.duration_label.configure(text="00:00 / 00:00")
                return
            
            self.duration_label.configure(
                text=f"{self.timer.current_time} / {self.duration}"
            )
            

    
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
        print("Go to https://musicplayer.isgood.dev/ for help")
        self.mainloop()