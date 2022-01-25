import audioplayer

class Song():
    def __init__(self):
        """
        A class for managing & manipulating audio
        """
        self.song = None
        self.paused = False

    def play(self, file):
        if self.song and self.paused:
            self.song.resume()
            self.paused = False
            return

        self.song = audioplayer.AudioPlayer(file)
        self.song.play(loop=False)
    
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