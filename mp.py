# pyright: reportMissingImports=false
# pyright: reportUndefinedVariable=false

import pafy
import shutil
import os
os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "hide"
import pygame
from moviepy.editor import *
from youtubesearchpython import VideosSearch

from tkinter import *
from tkinter.font import Font


class Window(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master)
        self.master = master
        self.pack(fill=BOTH, expand=1)

        self.npFont = Font(size='12', family='Helvetica')
        self.enterFont = Font(size='10', family='Helvetica')

        self.addsongLabel = Label(
            self, text='Add song by name:', bg='dark slate gray', fg='white')
        self.addsongLabel.place(x=10, y=80)
        self.addsongLabel['font'] = self.enterFont

        self.addsongbyurlLabel = Label(
            self, text='Add song by YT URL:', bg='dark slate gray', fg='white')
        self.addsongbyurlLabel.place(x=8, y=160)
        self.addsongbyurlLabel['font'] = self.enterFont

        self.play_icon = PhotoImage(file='Resources/playicon.png')
        self.pause_icon = PhotoImage(file='Resources/pauseicon.png')
        self.stop_icon = PhotoImage(file='Resources/stopicon.png')

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
        self.NowPlayingText.place(x=190, y=230)
        self.NowPlayingText['font'] = self.npFont

        self.nowPlayinglabel = Label(
            self, text='Nothing. Let\'s change that!', bg='dark slate gray', fg='white')
        self.nowPlayinglabel.place(x=280, y=230)
        self.nowPlayinglabel['font'] = self.npFont

        self.songInput = Entry(self, bg='slate gray', fg='black', borderwidth=0)
        self.songInput.place(x=10, y=100)
        self.songInput["font"] = self.enterFont
        self.songInput.insert(END, 'Enter song here')

        self.urlInput = Entry(self, bg='slate gray', fg='black', borderwidth=0)
        self.urlInput.place(x=10, y=180)
        self.urlInput["font"] = self.enterFont
        self.urlInput.insert(END, 'Enter URL here')

        self.enterUrlButton = Button(self, text='Add', command=lambda: self.enterURL(
            self.urlInput.get()), borderwidth=0, bg='slate gray', fg='black')
        self.enterUrlButton.place(x=12, y=205)

        self.enterButton = Button(self, text='Add', command=lambda: self.enterSong(
            self.songInput.get()), borderwidth=0, bg='slate gray', fg='black')
        self.enterButton.place(x=12, y=125)

    def playPlayer(self):
        song = self.songBox.get(ACTIVE)
        song = f"{os.getcwd()}/Audio bin/{song}"
        pygame.mixer.music.load(song)
        pygame.mixer.music.play(loops=0)
        self.updateNowPlaying()

    def updateNowPlaying(self):
        song = self.songBox.get(ACTIVE)
        self.nowPlayinglabel.configure(text=song[:-4])

    def pausePlayer(self):
        pygame.mixer.music.pause()

    def stopPlayer(self):
        pygame.mixer.music.stop()

    def updateList(self):
        songlist = os.listdir('./Audio bin/')
        self.songBox.delete(0, END)
        for file in songlist:
            self.songBox.insert(END, file)

    def enterURL(self, link):
        url = pafy.new(link)
        title = url.title
        high_quality = url.getbest()
        high_quality.download()
        shutil.move(f'{title}.mp4', 'Audio bin')
        mp4 = f'./Audio bin/{title}.mp4'
        mp3 = f'./Audio bin/{title}.mp3'
        clip = VideoFileClip(mp4)
        audioclip = clip.audio
        audioclip.write_audiofile(mp3)
        audioclip.close()
        clip.close()
        os.remove(f'./Audio bin/{title}.mp4')
        self.updateList()

    def enterSong(self, songname):
        videosearch = VideosSearch(songname, limit=1)
        vs = videosearch.result()
        link = vs["result"][0]["link"]
        title = vs["result"][0]["title"]
        print(title)
        print(link)
        op = pafy.new(link)
        hq = op.getbest()
        hq.download()
        if ":" in title:
            title = str(title).replace(":", "_")
        shutil.move(f'{title}.mp4', 'Audio bin')
        mp4 = f'./Audio bin/{title}.mp4'
        mp3 = f'./Audio bin/{title}.mp3'
        clip = VideoFileClip(mp4)
        audioclip = clip.audio
        audioclip.write_audiofile(mp3)
        audioclip.close()
        clip.close()
        os.remove(f'./Audio bin/{title}.mp4')
        self.updateList()


if __name__ == "__main__":
    pygame.mixer.init()
    root = Tk()
    app = Window(root)
    app.configure(bg='dark slate gray')
    if os.path.exists('Resources/musical_note.ico'):
        root.iconbitmap(r'Resources/musical_note.ico')
    root.wm_title('Music Player')
    root.geometry("700x500")
    root.mainloop()
