import pyglet
import os

import player.data as data

def interpolate_volume(vol):
    """Converts 0-100 to 0.0 to 1.0 for clean audio"""
    return round(vol / 100, 2)

class Audio():
    def __init__(self):
        """Control audio"""
        self.song = None
        self.paused = False
        self.volume = interpolate_volume(data.view("volume", "c"))
        self.queue = []

        self.player = None

    def _play(self, file, append_queue=False):
        """Plays or queues an audio source 
        
        append_queue - whether to append requested song to a queue or play now"""
        if not os.path.exists(file):
            return
        
        if self.paused and file == self.song:
            self.player.play()
            self.paused = False
            return
        
        if not self.player:
            self.player = pyglet.media.Player()

        src = pyglet.media.load(file)

        if append_queue:
            self.player.queue(src)
        else:
            self.player.delete()
            self.player = pyglet.media.Player()
            self.player.queue(src)    
            self.player.play()
            self.player.volume = self.volume

        self.song = file

    def _pause(self):
        """Pauses player"""
        if not self.song:
            return

        if self.paused:
            self.player.play()
            self.paused = False
        else:
            self.player.pause()
            self.paused = True
    
    def _stop(self):
        """Stops player and releases resources"""
        if not self.player:
            return
        
        self.player.delete()
        self.player = None
        self.song = None
    
    def _set_vol(self, amount):
        """Sets the volume as an integer, between 0 and 100 (also stores volume on disk)"""
        data.write("volume", amount, "c")
        if self.player:
            self.player.volume = interpolate_volume(amount)
    
    def pause_or_resume(self):
        """Pauses/resumes the player depending on whether player is paused or not"""
        if self.paused:
            self.player.play()
            self.paused = False
        else:
            self.player.pause()
            self.paused = True
