import audioplayer
import os

from .config import view

class Audio():
    def __init__(self):
        """
        A class for managing & controlling audio
        """
        self.song = None
        self.paused = False
        self.volume = view("volume")
    

    def play(self, file=None):
        if not os.path.exists(file):
            return

        if self.paused:
            self.paused = False
        

        if self.song and self.paused:
            self.song.resume()
            self.paused = False
            return

        if self.song:
            self.song.stop()
            self.song.close()
        
        self.song = None # Prevents glitchy sound
        self.song = audioplayer.AudioPlayer(file)
        self.song.play(loop=False)
        self.song.volume = self.volume
    
    def pause(self):
        if not self.song:
            return
        
        if self.paused:
            self.song.resume()
            self.paused = False
        else:
            self.song.pause()
            self.paused = True
    
    def stop(self):
        if not self.song:
            return
        
        self.song.stop()
        self.song.close()