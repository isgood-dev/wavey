# pyright: reportMissingImports=false
# pyright: reportUndefinedVariable=false

import audioplayer
import pafy
import shutil
import os
import threading
from moviepy.editor import *
from youtubesearchpython import VideosSearch

from tkinter import *
from tkinter.font import Font

DOWNLOAD_NOISE = False # Set to true if you DON'T want the the bytes, download rate and download ETA to be shown in terminal. (Recommended to keep to False for debugging and shows nice stats.)


class Window(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master)
        self.master = master
        self.pack(fill=BOTH, expand=1)  
        self.song = None

        self.npFont = Font(size='12', family='Helvetica')
        self.enterFont = Font(size='10', family='Helvetica')

        self.paused = False

        self.addsongLabel = Label(
            self, text='Add song by name:', bg='dark slate gray', fg='white')
        self.addsongLabel.place(x=10, y=80)
        self.addsongLabel['font'] = self.enterFont

        self.addsongbyurlLabel = Label(
            self, text='Add song by YT URL:', bg='dark slate gray', fg='white')
        self.addsongbyurlLabel.place(x=8, y=160)
        self.addsongbyurlLabel['font'] = self.enterFont

        self.play_icon = PhotoImage(file='Assets/playicon.png')
        self.pause_icon = PhotoImage(file='Assets/pauseicon.png')
        self.stop_icon = PhotoImage(file='Assets/stopicon.png')

        self.playSong = Button(self, image=self.play_icon, bg='dark slate gray',
                               fg='slate gray', command=self.playPlayer, borderwidth=0)
        self.playSong.place(x=210, y=270)

        self.stopSong = Button(self, image=self.stop_icon, bg='dark slate gray',
                               fg='slate gray', borderwidth=0, command=self.stopPlayer)
        self.stopSong.place(x=310, y=270)

        self.pauseSong = Button(
            self, image=self.pause_icon, bg='dark slate gray', fg='slate gray', command=self.pausePlayer, borderwidth=0)
        self.pauseSong.place(x=410, y=270)

        self.songBox = Listbox(self, bg='slate gray', fg='black', width=60)
        self.songBox.place(x=160, y=60)
        self.updateList()

        self.NowPlayingText = Label(
            self, text='Now playing:', bg='dark slate gray', fg='white')
        self.NowPlayingText.place(x=160, y=230)
        self.NowPlayingText['font'] = self.npFont

        self.nowPlayinglabel = Label(
            self, text='Nothing. Let\'s change that!', bg='dark slate gray', fg='white')
        self.nowPlayinglabel.place(x=252, y=230)
        self.nowPlayinglabel['font'] = self.npFont

        self.songInput = Entry(self, bg='slate gray',
                               fg='black', borderwidth=0)
        self.songInput.place(x=10, y=100)
        self.songInput["font"] = self.enterFont
        self.songInput.insert(END, 'Enter song here')

        self.urlInput = Entry(self, bg='slate gray', fg='black', borderwidth=0)
        self.urlInput.place(x=10, y=180)
        self.urlInput["font"] = self.enterFont
        self.urlInput.insert(END, 'Enter URL here')

        self.downloadingLabel = Label(
            self, text='', bg='dark slate gray', fg='white')
        self.downloadingLabel.place(x=10, y=230)
        self.downloadingLabel["font"] = self.npFont

        self.enterUrlButton = Button(self, text='Add', command=lambda: threading.Thread(target=lambda: self.fetch_by_link(
            self.urlInput.get())).start(), borderwidth=0, bg='slate gray', fg='black')
        self.enterUrlButton.place(x=12, y=205)

        self.enterButton = Button(self, text='Add', command=lambda: threading.Thread(target=lambda: self.fetch_by_name(
            self.songInput.get())).start(), borderwidth=0, bg='slate gray', fg='black')
        self.enterButton.place(x=12, y=125)

    def playPlayer(self):
        if self.song:
            self.song.stop()
            self.song = None

        song = self.songBox.get(ACTIVE)
        song = f"{os.getcwd()}/Audio bin/{song}"
        self.song = audioplayer.AudioPlayer(song)
        self.song.play(loop=False)

        self.updateNowPlaying()

    def updateNowPlaying(self):
        song = self.songBox.get(ACTIVE)
        self.nowPlayinglabel.configure(text=song[:-4])

    def pausePlayer(self):
        if self.paused:
            self.song.resume()
            self.paused = False
        else:
            self.song.pause()
            self.paused = True

    def stopPlayer(self):
        self.song.stop()
        self.song.close() # Releases resources and provents memory leaks
        self.nowPlayinglabel.configure(text='Nothing. Let\'s change that!')

    def updateList(self):
        songlist = os.listdir('./Audio bin/')
        self.songBox.delete(0, END)
        for file in songlist:
            self.songBox.insert(END, file)

    def fetch_by_link(self, link):
        self.downloadingLabel.configure(text='Downloading...')
        url = pafy.new(link)
        title = url.title
        link = url.watchv_url
        print(f"Downloading: {title} ({link})")
        hq = url.getbest()
        hq.download(filepath="./Audio bin/", quiet=DOWNLOAD_NOISE, remux_audio=True)
        self.mp4_to_mp3(title)
        self.updateList()
        self.downloadingLabel.configure(text='')

    def fetch_by_name(self, name):
        self.downloadingLabel.configure(text="Downloading...")
        videosearch = VideosSearch(name, limit=1)
        vs = videosearch.result()
        link = vs["result"][0]["link"]
        title = vs["result"][0]["title"]
        print(f"Downloading: {title} ({link})")
        op = pafy.new(link)
        hq = op.getbest()
        hq.download(filepath=f"./Audio bin/", quiet=False, remux_audio=True)
        self.mp4_to_mp3(title)
        self.updateList()
        self.downloadingLabel.configure(text='')

    def mp4_to_mp3(self, title):
        mp4 = f'./Audio bin/{title}.mp4'
        mp3 = f'./Audio bin/{title}.mp3'
        clip = VideoFileClip(mp4)
        audioclip = clip.audio
        audioclip.write_audiofile(mp3)
        audioclip.close()
        clip.close()
        os.remove(f'./Audio bin/{title}.mp4')

if __name__ == "__main__":
    root = Tk()
    app = Window(root)
    app.configure(bg='dark slate gray')
    if os.path.exists('Assets/musical_note.ico'):
        root.iconbitmap(r'Assets/musical_note.ico')
    root.wm_title('Music Player')
    root.geometry("600x400")
    root.mainloop()
