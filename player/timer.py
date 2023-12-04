from time import sleep

class Timer:
    def __init__(self, duration):
        self.minutes = 0
        self.seconds = 0
        self.current_time = ""

        self.duration = duration
        self.paused = False
        self.is_active = False
        self.end_reached = False

    def start(self):
        formatted_mins = str(self.minutes).zfill(2)
        formatted_secs = str(self.seconds).zfill(2)
        formatted_duration = f"{formatted_mins}:{formatted_secs}"

        while formatted_duration != self.duration:
            self.is_active = True

            if self.paused:
                break
            
            if self.seconds == 59:
                self.minutes += 1
                self.seconds = 0
            else:
                self.seconds += 1

            sleep(1)

            new_secs = str(self.seconds).zfill(2)
            new_mins = str(self.minutes).zfill(2)
            self.current_time = f"{new_mins}:{new_secs}"

            if self.current_time == self.duration:
                self.end_reached = True
                self.is_active = False
                return
