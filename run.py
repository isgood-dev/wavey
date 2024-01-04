# Entry point to run the music player

import player.utils.sys_logger as logger
import logging

_log = logging.getLogger("app.runner")

class Runner:
    def __init__(self) -> None:
        import player.main as main

        self.mainwindow = main.MainWindow()

    def run(self):
        self.mainwindow._run()

logger.setup_logging()
runner = Runner()

if __name__ == "__main__":
    _log.info("Runner has been initialized")
    runner.run()