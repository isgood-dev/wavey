import logging
from time import sleep

class Timer:
    def __init__(self, duration=None):
        self._log = logging.getLogger("app.timer")
        self._log.info("Timer has been initialized")

        self.minutes = 0
        self.seconds = 0
        self.current_time = ""

        self.total_duration = duration
        self.paused = False
        self.is_active = False
        self.end_reached = False
        self.stop = False

        self.first_iteration = True

    def start(self, progress_label):
        """Starts the progress timer."""
        self._log.info("Timer has started")
        self.is_active = True
        formatted_mins = str(self.minutes).zfill(2)
        formatted_secs = str(self.seconds).zfill(2)
        formatted_duration = f"{formatted_mins}:{formatted_secs}"

        while formatted_duration != self.total_duration:
            if not self.first_iteration:
                sleep(1)

            if self.paused:
                continue

            if self.stop:
                break

            if self.first_iteration:
                self.first_iteration = False
            
            if self.seconds == 59:
                self.minutes += 1
                self.seconds = 0
            else:
                self.seconds += 1

            new_secs = str(self.seconds).zfill(2)
            new_mins = str(self.minutes).zfill(2)
            self.current_time = f"{new_mins}:{new_secs}"

            progress_label.configure(text=f"{self.current_time} / {self.total_duration}")

            if self.current_time == self.total_duration:
                self.end_reached = True
                break
