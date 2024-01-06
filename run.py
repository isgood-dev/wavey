# Entry point to run the music player

import logging
import argparse

import player.utils.sys_logger as logger

_log = logging.getLogger("app.runner")

parser = argparse.ArgumentParser()
parser.add_argument('--updated', action="store_true")
args = parser.parse_args()

class Runner:
    def __init__(self) -> None:
        import player.main as main

        self.mainwindow = main.MainWindow()

    def run(self):
        self.mainwindow._run(args.updated)

logger.setup_logging()
runner = Runner()

if __name__ == "__main__":
    _log.info("Runner has been initialized")
    runner.run()