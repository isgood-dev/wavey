import datetime
import os
import threading
import logging
from mutagen.mp3 import MP3

from tkinter import *
from tkinter import messagebox
from tkinter.font import Font

import player.widgets as widgets
import player.utils.data as data
import player.utils.audio as audio
import player.utils.timer as timer

BACK_COLOUR = data.view("back_colour", "c")
FORE_COLOUR = data.view("fore_colour", "c")

_log = logging.getLogger("app.main")

class MainWindow(Tk):
    def __init__(self):
        super().__init__()
        
        # Window attrs
        self.configure(bg=data.view("back_colour", "c"))
        self.wm_title("Music Player")
        self.geometry("850x600")
        self.resizable(False, False)

        self.sbf = None
        self.current_song = None

        self.bind("<space>", self.pause_or_resume)
        self.bind("<Escape>", self.close_window)

        self._setup_extensions()
        self._setup_constants()
        self._setup_widgets()

        try:
            self.iconbitmap(self.images.MAIN)
        except TclError:
            pass

        self.refresh_songlist()

    def _setup_extensions(self):
        import player.menus.download as download
        import player.menus.settings as settings
        import player.menus.files as files
        import player.menus.updater as updater
        # import player.menus.playlists as playlist

        self.audio = audio.Audio()
        self.timer = timer.Timer()
        self.settings = settings.Settings()
        self.files = files.Files()
        self.download = download.Download()
        self.updater = updater.Updater()
        _log.info("Extensions have been started.")

    def _setup_constants(self):
        import player.utils.constants as constants

        self.fonts = constants.Font()
        self.images = constants.Image()

    def _setup_widgets(self):
        Label(self, bg=FORE_COLOUR, height=35, width=600).place(x=0, y=500)
        Frame(self, bg=data.view("accent_colour", "c"), height=2, bd=0).pack(fill=X, side=BOTTOM, pady=100)
        Frame(self, bg=data.view("accent_colour", "c"), width=2, height=500, bd=0 ).place(x=198, y=0)

        self.duration_label = Label(self, text="00:00 / 00:00", bg=FORE_COLOUR, fg="white", font=self.fonts.MAIN)
        self.duration_label.place(x=250, y=545)
        
        self.now_playing = Label(self, text="Nothing is playing. Play a song by finding a song in the song list and clicking the ▶ next to it!", bg=FORE_COLOUR, fg="white", font=self.fonts.MAIN)
        self.now_playing.place(relx=0.5, rely=0.86, anchor=CENTER)
        
        self.ispaused = Label(self, text="", bg=FORE_COLOUR, fg="white", font=self.fonts.MAIN)
        self.ispaused.place(relx=0.5, rely=0.97, anchor=CENTER)

        self.pauseplay_button = Button(self, image=self.images.PAUSEPLAY, background=FORE_COLOUR, borderwidth=0, command=self.pause_or_resume, activebackground=FORE_COLOUR)
        self.pauseplay_button.place(relx=0.499, rely=0.93, anchor=CENTER)

        self.addmusic_button = widgets.HoverButton(self, text="Add music", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=self.download.download_window, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.addmusic_button["font"] = self.fonts.MEDIUM
        self.addmusic_button.place(x=25, y=40)

        self.rename_file = widgets.HoverButton(self, text="Rename a file", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=self.files.rename_window, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.rename_file["font"] = self.fonts.MEDIUM
        self.rename_file.place(x=25, y=70)
        
        self.settings_btn = widgets.HoverButton(self, text="Settings", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=self.settings.settings_window, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.settings_btn["font"] = self.fonts.MEDIUM
        self.settings_btn.place(x=25, y=100)

        self.delete_song = widgets.HoverButton(self, text="Delete a file", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=self.files.delete_file, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.delete_song["font"] = self.fonts.MEDIUM
        self.delete_song.place(x=25, y=130)

        self.myplaylists = widgets.HoverButton(self, text="My Playlists", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=None, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.myplaylists["font"] = self.fonts.MEDIUM
        self.myplaylists.place(x=25, y=160)

        self.update = widgets.HoverButton(self, text="Check for updates", bg=BACK_COLOUR, fg="white", compound="left", borderwidth=0, command=self.updater.update, activebackground=BACK_COLOUR, activeforeground=data.view("accent_colour", "c"))
        self.update["font"] = self.fonts.MEDIUM
        self.update.place(x=15, y=460)

        self.volume = Scale(self, orient=HORIZONTAL, variable=DoubleVar(), bg=FORE_COLOUR, fg="white", troughcolor=BACK_COLOUR, highlightthickness=0)
        self.volume.set(data.view("volume", "c"))
        self.volume.bind("<ButtonRelease-1>", self.set_volume)
        self.volume.place(x=530, y=535)

        self.stop_button = Button(self, image=self.images.STOP, background=FORE_COLOUR, borderwidth=0, command=self.stop, activebackground=FORE_COLOUR)
        self.stop_button.place(x=375, y=545)

    def set_np(self, text: str):
        """Sets the "now playing" label"""
        return self.now_playing.configure(text=text)

    def refresh_songlist(self):
        _log.info("Refresh songlist")
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
                font=self.fonts.MAIN
            ).grid(row=0, column=0)
            
            refnosongs = Button(
                self.scroll_frame,
                text="[Refresh]",
                bg=data.view("songlist_colour", "c"),
                fg="white",
                font=self.fonts.MAIN,
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
            font=self.fonts.MAIN,
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
                font=self.fonts.MAIN,
                fg="white",
            ).grid(row=i, column=1, sticky="w")

            Label(
                self.scroll_frame,
                text="        " + duration,
                bg=data.view("songlist_colour", "c"),
                font=self.fonts.MAIN,
                fg="white"
            ).grid(row=i, column=2, sticky="e")
    
    def play(self, source, append_queue=False):
        _log.info(f"Play - source:{source}")

        if self.timer.is_active:
            self.audio._stop() # if a song is already playing, stop that song to allow new one to play
            self.timer.stop = True

        duration = MP3("./data/audio/" + source + ".mp3")
        duration = duration.info.length
        self.duration = str(datetime.timedelta(seconds=round(duration)))[2:]

        self.current_song = source

        self.audio._play(os.getcwd() + f"/data/audio/{source}.mp3", append_queue=append_queue)
        self.update_now_playing()

        self.timer = timer.Timer(self.duration)
        thread = threading.Thread(target=lambda: self.timer.start(self.duration_label))
        thread.daemon = True
        thread.start()

        
    def pause_or_resume(self, event=None):
        _log.info(f"Pause - event:{event}")
        if self.audio.paused:
            self.audio.pause_or_resume()
            self.timer.paused = False
        else:
            self.audio.pause_or_resume()
            self.timer.paused = True

    def stop(self):
        _log.info("Stop")
        self.audio._stop()
        self.timer.stop = True

        self.duration_label.configure(text="00:00 / 00:00")
        self.now_playing.configure(text="Nothing is playing.")
            
    
    def update_now_playing(self):
        song = self.current_song
        self.now_playing.configure(text=song)
    
    def set_volume(self, event=None):
        amount = round(self.volume.get())

        self.audio._set_vol(amount)
        
    def close_window(self, event=None):
        _log.info(f"Close window prompted - event:{event}")
        ask = messagebox.askyesno(
            title="Close Music Player",
            message="Are you sure you want to close the Music Player?"
        )

        if ask:
            return self.destroy()
    
    def _run(self):
        """Calls the mainloop, instantiating the window"""
        _log.info("Running MainWindow...")
        print("Go to https://acatiadroid.github.io/music-player-site/ for help")
        self.mainloop()