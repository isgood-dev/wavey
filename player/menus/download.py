import threading
import os
import shutil
import logging
from pytube import YouTube
from pytube.exceptions import RegexMatchError
from youtubesearchpython import VideosSearch
from moviepy.video.io.VideoFileClip import VideoFileClip

from tkinter import *
from tkinter import messagebox
from tkinter import filedialog

import player.utils.data as data
import player.utils.constants as constants

_log = logging.getLogger("app.download")

class Download:
    def __init__(self):
        _log.info("Download has been initialized")
        
        self.root = None
        self.dl_root = None

        self.prog_label = None

        self.fonts = constants.Font()
        self.images = constants.Image()

    def file_convert(self, title):
        mp4 = f'./data/audio/{title}.mp4'
        mp3 = f'./data/audio/{title}.mp3'
        clip = VideoFileClip(mp4)
        audioclip = clip.audio
        audioclip.write_audiofile(mp3)
        audioclip.close()
        clip.close()
        os.remove(f'./data/audio/{title}.mp4')

    def download(self, *, title=None, link=None):
        self.dl_root.destroy()
        
        self.root = Toplevel()
        self.root.wm_title("Progress")
        self.root.configure(bg=data.view("back_colour", "c"))

        self.prog_label = Label(
            self.root,
            text="Starting download...",
            font=self.fonts.LARGE,
            fg="white",
            bg=data.view("back_colour", "c")
        )
        self.prog_label.pack()

        def clean_name(title):
            """Removes any illegal file characters from the name given from YT"""
            bannedchars = ["<", ">", ":", "\"", "/", "\\", "|", "?", "*"] # banned characters in Windows.
            for char in bannedchars:                                      # may not apply to other operating systems.
                if char in title:
                    title = title.replace(char, "_")
            
            return title
        
        if not link and title:
            search = VideosSearch(title, limit=1)
            search = search.result()

            result = search["result"][0]
            url = result["link"]
            title = result["title"]
            
            video = YouTube(url)   
        else:
            try:
                video = YouTube(link)
            except RegexMatchError:
                return messagebox.showerror(
                    title="Invalid URL",
                    message="The URL provided is invalid. Please provide the 11 character video id or the URL to the video."
                )
        
        video_info = video.streams.filter(progressive=True, file_extension="mp4").order_by("resolution").first()
        title = video_info.title
        link = video_info.url
        
        print(f"Downloading: {title} ({link})")
        self.prog_label.configure(text=f"Downloading: {title}")

        clean_title = clean_name(title)
        video_info.download("./data/audio/", clean_title + ".mp4")

        self.prog_label.configure(text="Converting to audio")
        self.file_convert(clean_title)
        
        self.root.destroy()

        messagebox.showinfo(
            title="Song Downloaded",
            message=f"Downloaded:\n{title}"
        )

    def file_opener(self):
        file = filedialog.askopenfile(
            initialdir=".",
            title="Select an MP3 file",
            filetypes=(
                ("MP3 files", "*.mp3"),
                ("all files", "*.*")
            )
        )
        if not file:
            return

        filename = file.name.split("/")
        filename = filename[len(filename)-1]
        shutil.copyfile(file.name, f"./data/audio/{filename}")
        # TODO: update song list

    def download_window(self):
        back_colour = data.view("back_colour", "c")
        fore_colour = data.view("fore_colour", "c")

        self.dl_root = Toplevel()

        self.dl_root.configure(bg=data.view("back_colour", "c"))
        self.dl_root.geometry("300x300")
        self.dl_root.wm_title("Add music")
        self.dl_root.resizable(False, False)
        
        try:
            self.dl_root.iconbitmap(self.images.DOWNLOAD)
        except TclError:
            pass

        Label(
            self.dl_root,
            text="Add song by name:",
            bg=back_colour,
            fg="white",
            font=self.fonts.MAIN
        ).pack()

        add_name_entry = Entry(
            self.dl_root,
            fg="white",
            bg=fore_colour,
            font=self.fonts.MAIN
        )
        add_name_entry.pack()
        
        download_name = Button(
            self.dl_root,
            fg="white",
            bg=fore_colour,
            font=self.fonts.MAIN,
            text="Download",
            borderwidth=0,
            command=lambda: threading.Thread(
                target=lambda: self.download(
                    title=add_name_entry.get()
                )
            ).start()
        )
        download_name.pack()

        Label(
            self.dl_root,
            text="Add song by YT URL:",
            fg="white",
            bg=back_colour,
            font=self.fonts.MAIN
        ).pack()

        add_by_url_entry = Entry(
            self.dl_root,
            fg="white",
            bg=fore_colour,
            font=self.fonts.MAIN
        )
        add_by_url_entry.pack()

        download_url = Button(
            self.dl_root,
            fg="white",
            bg=fore_colour,
            font=self.fonts.MAIN,
            text="Download",
            borderwidth=0,
            command=lambda: threading.Thread(
                target=lambda: self.download(
                    link=add_by_url_entry.get()
                )
            ).start()
        )
        download_url.pack()

        import_music = Button(
            self.dl_root,
            fg="white",
            bg=fore_colour,
            font=self.fonts.MAIN,
            text="Import Music from PC",
            command=self.file_opener
        )
        import_music.pack(pady=20)