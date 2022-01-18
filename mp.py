# pyright: reportMissingImports=false
# pyright: reportUndefinedVariable=false

import audioplayer
import pafy
import shutil
import os
import threading
from moviepy.editor import VideoFileClip
from youtubesearchpython import VideosSearch

import tkinter.messagebox
from tkinter import *
from tkinter.font import Font
from tkinter import filedialog

import config

# Set to true if you DON'T want the the bytes, download rate and download ETA to be shown in terminal. (Recommended to keep to False for debugging and shows nice stats.)
DOWNLOAD_NOISE = False

BACK_COLOUR = "#111111"
FORE_COLOUR = "#2b2b2b"

class Window(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master)
        self.master = master
        self.pack(fill=BOTH, expand=1)
        self.song = None

        self.master.bind("<space>", self.pauseEvent)

        self.ARIAL = Font(size=10, family="Cascadia Mono")
        self.SMALLER_ARIAL = Font(size=8, family="Cascadia Mono")

        self.paused = False

        self.bottom_Bar = Label(self, bg=FORE_COLOUR, height=35, width=600)
        self.bottom_Bar.place(x=0, y=310)

        self.play_icon = PhotoImage(file='Assets/playicon.png')
        self.pause_icon = PhotoImage(file='Assets/pauseicon.png')
        self.stop_icon = PhotoImage(file='Assets/stopicon.png')

        self.playSong = Button(self, image=self.play_icon,
                               bg=FORE_COLOUR, command=self.playPlayer, borderwidth=0)
        self.playSong.place(x=285, y=350)

        self.stopSong = Button(self, image=self.stop_icon,
                               bg=FORE_COLOUR, borderwidth=0, command=self.stopPlayer)
        self.stopSong.place(x=325, y=350)

        self.pauseSong = Button(
            self, image=self.pause_icon, bg=FORE_COLOUR, command=self.pausePlayer, borderwidth=0)
        self.pauseSong.place(x=245, y=350)

        self.songsLabel = Label(self, bg=BACK_COLOUR,
                                fg="white", font=self.ARIAL)
        self.songBox = Listbox(self, bg=BACK_COLOUR, fg='white', width=75, height=15,
                               activestyle=None, font=self.SMALLER_ARIAL, borderwidth=0, highlightthickness=1)
        self.songBox.place(x=130, y=55)
        self.updateList()

        self.nowPlayinglabel = Label(
            self, text='Nothing is playing. Let\'s change that!', bg=FORE_COLOUR, fg='white', font=self.ARIAL)
        self.nowPlayinglabel.place(x=5, y=320)

        self.downloadingLabel = Label(
            self, text='', bg=BACK_COLOUR, fg='white', font=self.ARIAL)
        self.downloadingLabel.place(x=10, y=230)

        self.enterButton = Button(self, text='Add Music', command=self.download_window,
                                  borderwidth=0, bg=FORE_COLOUR, fg="white", width=14, height=2)
        self.enterButton.place(x=12, y=125)

        self.renamefile = Button(self, text='Rename file', command=self.rename_window,
                                 borderwidth=0, bg=FORE_COLOUR, fg="white", width=14, height=2)
        self.renamefile.place(x=12, y=175)

        self.volume = Scale(self, orient=HORIZONTAL, variable=DoubleVar(), bg=FORE_COLOUR, fg="white", troughcolor=BACK_COLOUR, highlightthickness=0)
        self.volume.place(x=400, y=340)
        self.volume.set(config.view("volume"))
        self.volume.bind("<ButtonRelease-1>", self.volumeSet)
    
    def volumeSet(self, event):
        vol = self.volume.get()
        config.write("volume", vol)
        if self.song:
            self.song.volume = vol
    def playPlayer(self):
        if self.song and self.paused: 
            self.song.resume()
            self.paused = False
            return

        self.paused = False
        if self.song:
            self.song.stop()
            self.song = None

        song = self.songBox.get(ACTIVE)
        song = f"{os.getcwd()}/Audio bin/{song}.mp3"
        self.song = audioplayer.AudioPlayer(song)
        self.song.play(loop=False)

        self.updateNowPlaying()
    
    def pauseEvent(self, event):
        if not self.song:
            return

        if self.paused:
            self.song.resume()
            self.paused = False
        else:
            self.song.pause()
            self.paused = True

    def updateNowPlaying(self):
        song = self.songBox.get(ACTIVE)
        self.nowPlayinglabel.configure(text=song)

    def pausePlayer(self):
        if not self.song:
            return

        if self.paused:
            self.song.resume()
            self.paused = False
        else:
            self.song.pause()
            self.paused = True

    def stopPlayer(self):
        if not self.song:
            return
        self.song.stop()
        self.song.close()  # Releases resources and provents memory leaks
        self.nowPlayinglabel.configure(
            text='Nothing is playing. Let\'s change that!')

    def updateList(self):
        songlist = os.listdir('./Audio bin/')
        self.songBox.delete(0, END)
        for file in songlist:
            if file.endswith(".mp3"):
                self.songBox.insert(END, file[:-4])

    def mp4_to_mp3(self, title):
        mp4 = f'./Audio bin/{title}.mp4'
        mp3 = f'./Audio bin/{title}.mp3'
        clip = VideoFileClip(mp4)
        audioclip = clip.audio
        audioclip.write_audiofile(mp3)
        audioclip.close()
        clip.close()
        os.remove(f'./Audio bin/{title}.mp4')

    def file_opener(self):
        input = filedialog.askopenfile(
            initialdir="/", title="Select an mp3 file", filetypes=(("MP3 files", "*.mp3"), ("all files", "*.*")))
        if input is None:
            return
        filename = input.name.split("/")
        filename = filename[len(filename) - 1]
        shutil.copyfile(input.name, f"./Audio bin/{filename}")
        self.updateList()

    def fetch_by_link(self, link):
        self.downloadingLabel.configure(text='Downloading...')
        self.window.destroy()
        try:
            url = pafy.new(link)
        except ValueError:
            tkinter.messagebox.showerror("Failed", "Invalid YouTube URL provided. 11 character video ID or URL required.")
            self.downloadingLabel.configure(text='')
            return
        title = url.title
        link = url.watchv_url
        print(f"Downloading: {title} ({link})")
        hq = url.getbest()
        hq.download(filepath="./Audio bin/",
                    quiet=DOWNLOAD_NOISE, remux_audio=True)
        self.mp4_to_mp3(title)
        self.updateList()
        self.downloadingLabel.configure(text='')
        tkinter.messagebox.showinfo(
            title="Success", message=f"\"{title}\" has been downloaded!")

    def fetch_by_name(self, name):
        self.downloadingLabel.configure(text="Downloading...")
        self.window.destroy()
        videosearch = VideosSearch(name, limit=1)
        vs = videosearch.result()
        link = vs["result"][0]["link"]
        title = vs["result"][0]["title"]
        print(f"Downloading: {title} ({link})")
        op = pafy.new(link)
        hq = op.getbest()
        hq.download(filepath=f"./Audio bin/",
                    quiet=DOWNLOAD_NOISE, remux_audio=True)
        self.mp4_to_mp3(title)
        self.updateList()
        self.downloadingLabel.configure(text='')
        tkinter.messagebox.showinfo(
            title="Success", message=f"\"{title}\" has been downloaded!")

    def download_window(self):
        self.window = Toplevel()
        self.window.configure(bg=BACK_COLOUR)
        self.window.geometry("300x300")
        self.window.wm_title("Add music")
        self.window.iconbitmap("Assets/downloadicon.ico")
        self.window.resizable(False, False)
        label = Label(self.window, text="Download/Import Music",
                      font=Font(size=13, family="Cascadia Mono"), fg="white", bg=BACK_COLOUR)
        label.pack()

        addbynameLabel = Label(self.window, text="Add song by name:",
                               fg="white", bg=BACK_COLOUR, font=self.ARIAL)
        addbynameLabel.pack()

        self.addbynameEntry = Entry(
            self.window, fg="white", bg=FORE_COLOUR, font=self.ARIAL)
        self.addbynameEntry.pack()

        downloadnameBtn = Button(self.window, fg="white", bg=FORE_COLOUR, font=self.ARIAL, text="Download", borderwidth=0, command=lambda: threading.Thread(target=lambda: self.fetch_by_name(
            self.addbynameEntry.get())).start())
        downloadnameBtn.pack()

        addbyurlLabel = Label(self.window, text="Add song by YT URL:",
                              fg="white", bg=BACK_COLOUR, font=self.ARIAL)
        addbyurlLabel.pack()

        self.addbyurlEntry = Entry(
            self.window, fg="white", bg=FORE_COLOUR, font=self.ARIAL)
        self.addbyurlEntry.pack()

        downloadurlBtn = Button(self.window, fg="white", bg=FORE_COLOUR, font=self.ARIAL, text="Download", borderwidth=0, command=lambda: threading.Thread(target=lambda: self.fetch_by_link(
            self.addbyurlEntry.get())).start())
        downloadurlBtn.pack()

        importMusicBtn = Button(self.window, fg="white", bg=FORE_COLOUR,
                                font=self.ARIAL, text="Import Music from PC", command=self.file_opener)
        importMusicBtn.pack(pady=20)

        self.downloadedLabel = Label(
            self.window, fg="#66b208", bg=BACK_COLOUR, font=self.ARIAL, text="")
        self.downloadedLabel.pack(pady=20)

    def rename_file(self):
        new = self.newNameEntry.get()
        os.rename(f"./Audio bin/{self.file}", "./Audio bin/" + str(new) + ".mp3")
        self.updateList()
        self.window.destroy()
        tkinter.messagebox.showinfo("Success", f"File name successfully changed.\nBefore: {self.file}\nAfter: {str(new) + '.mp3'}")

    def rename_window(self):
        input = filedialog.askopenfile(
            initialdir="./Audio bin/", title="Select an mp3 file", filetypes=(("MP3 files", "*.mp3"), ("all files", "*.*")))
        if input is None:
            return

        self.window = Toplevel()
        self.window.configure(bg=BACK_COLOUR)
        self.window.geometry("450x300")
        self.window.wm_title("Rename a file")
        self.window.resizable(True, False)
        self.window.iconbitmap("Assets/pencil.ico")

        label = Label(self.window, text="Rename a file",
                      font=Font(size=13, family="Cascadia Mono"), fg="white", bg=BACK_COLOUR)
        label.pack()

        selectedSongLabel = Label(self.window, text="Selected song:",
                              fg="white", bg=BACK_COLOUR, font=self.ARIAL)
        selectedSongLabel.pack()

        file = input.name.split("/")
        self.file = file[len(file)-1]

        selectedSong = Label(self.window, text=self.file[:-4],
                             fg="#2ca351", bg=BACK_COLOUR, font=Font(size=12, family="Cascadia Mono"))
        selectedSong.pack()

        newNameLabel = Label(self.window, text="Enter new name: (don't include .mp3)",
                             fg="white", bg=BACK_COLOUR, font=self.ARIAL)
        newNameLabel.pack(pady=10)

        self.newNameEntry = Entry(
            self.window, fg="white", bg=FORE_COLOUR, font=self.ARIAL)
        self.newNameEntry.pack()

        renameBtn = Button(self.window, fg="white", bg=FORE_COLOUR,
                                font=self.ARIAL, text="Done", command=self.rename_file)
        renameBtn.pack(pady=20)

def main():
    root = Tk()
    app = Window(root)
    app.configure(bg=BACK_COLOUR)
    if os.path.exists('Assets/musical_note.ico'):
        root.iconbitmap(r'Assets/musical_note.ico')
    root.wm_title('Music Player')
    root.geometry("600x400")
    root.resizable(False, False)
    root.mainloop()

if __name__ == "__main__":
    main()
