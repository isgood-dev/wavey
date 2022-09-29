import pyglet
import os

from player.config import view, write

class Audio():
    def __init__(self):
        """
        A class for managing & controlling audio
        """
        self.song = None
        self.paused = False
        self.volume = view("volume")
        self.queue = []

        self.player = None

    def _play(self, file, append_queue=False):
        if not os.path.exists(file):
            return
        
        if self.paused:
            self.paused = False
        
        if self.song and self.paused:
            self.song.play()
            self.paused = False
            return
        
        if self.song:
            self.player.delete()
        
        if not append_queue:
            self.player.delete()
            self.player = pyglet.media.Player()
        
        src = pyglet.media.load(file)
        
        self.player.queue(src)
        self.player.play()
        self.player.volume = self.volume

        self.song = src

    def _pause(self):
        if not self.song:
            return

        if self.paused:
            self.player.play()
            self.paused = False
        else:
            self.player.pause()
            self.paused = True
    
    def _stop(self):
        if not self.player:
            return
        
        self.player.delete()
        self.player = None
    
    def _set_vol(self, amount):
        write("volume", amount)
        if self.player:
            self.player.volume = amount
    
    def pause_or_resume(self, lbl):
        if self.paused:
            self.player.play()
            self.paused = False
