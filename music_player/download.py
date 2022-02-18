import threading
from tkinter import messagebox
import pafy
import os
import shutil
from youtubesearchpython import VideosSearch
from moviepy.video.io.VideoFileClip import VideoFileClip # Importing from submodule as moviepy.editor is intended for manual use.
from tkinter import *
from tkinter.font import Font
from tkinter import filedialog
from .config import view


def file_convert(title):
    mp4 = f'./Audio bin/{title}.mp4'
    mp3 = f'./Audio bin/{title}.mp3'
    clip = VideoFileClip(mp4)
    audioclip = clip.audio
    audioclip.write_audiofile(mp3)
    audioclip.close()
    clip.close()
    os.remove(f'./Audio bin/{title}.mp4')

def download(*, title=None, link=None):
    win.destroy()
    if not link and title:
        search = VideosSearch(title, limit=1)
        search = search.result()

        result = search["result"][0]
        url = result["link"]
        name = result["title"]
        
        print(f"Downloading: {name} ({url})")

        video = pafy.new(url)
        video = video.getbest()
        
        video.download(
            filepath="./Audio bin/"
        )
        
        file_convert(name)
        messagebox.showinfo(
            title="Song Downloaded",
            message=f"Downloaded:\n{name}\n\nPlease refresh the song list!"
        )
        return
    else:
        video = pafy.new(link)
        title = video.title
        link = video.watchv_url
        video = video.getbest()
        print(f"Downloading: {title} ({link})")
        video.download(
            filepath="./Audio bin/"
        )
        file_convert(title)

def file_opener():
    file = filedialog.askopenfile(
        initialdir=".",
        title="Select an MP3 file",
        filetypes=(
            ("MP3 files", "*.mp3"),
            ("all files", "*.*")
        )
    )
    if not input:
        return

    filename = file.name.split("/")
    filename = filename[len(filename)-1]
    shutil.copyfile(file.name, f"./Audio bin/{filename}")
    # TODO: update song list

def download_window():
    back_colour = view("back_colour")
    fore_colour = view("fore_colour")

    window = Toplevel()
    window.configure(bg=view("back_colour"))
    window.geometry("300x300")
    window.wm_title("Add music")
    window.iconbitmap("music_player/Assets/downloadicon.ico")
    window.resizable(False, False)

    global win
    win = window
    
    Label(
        window,
        text="Add song by name:",
        bg=back_colour,
        fg="white"
    ).pack()

    add_name_entry = Entry(
        window,
        fg="white",
        bg=fore_colour,
        font=Font(size=10, family="Cascadia Mono")
    )
    add_name_entry.pack()
    
    download_name = Button(
        window,
        fg="white",
        bg=fore_colour,
        font=Font(size=10, family="Cascadia Mono"),
        text="Download",
        borderwidth=0,
        command=lambda: threading.Thread(
            target=lambda: download(
                title=add_name_entry.get()
            )
        ).start()
    )
    download_name.pack()

    Label(
        window,
        text="Add song by YT URL:",
        fg="white",
        bg=back_colour,
        font=Font(size=10, family="Cascadia Mono")
    ).pack()

    add_by_url_entry = Entry(
        window,
        fg="white",
        bg=fore_colour,
        font=Font(size=10, family="Cascadia Mono")
    )
    add_by_url_entry.pack()

    download_url = Button(
        window,
        fg="white",
        bg=fore_colour,
        font=Font(size=10, family="Cascadia Mono"),
        text="Download",
        borderwidth=0,
        command=lambda: threading.Thread(
            target=lambda: download(
                link=add_by_url_entry.get()
            )
        ).start()
    )
    download_url.pack()

    import_music = Button(
        window,
        fg="white",
        bg=fore_colour,
        font=Font(size=10, family="Cascadia Mono"),
        text="Import Music from PC",
        command=file_opener
    )
    import_music.pack(pady=20)
