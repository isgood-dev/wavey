"""
This file will run the music player.

Please refer to the README.md for guidance on how to set up this music player - you may need to install some dependencies.

Please respect the GPL v3.0 license in the LICENSE file.

Created by acatia (acatia#5378)

Check out misc/help.mdown if you need help with something or join the Discord server:
https://discord.gg/p5bURjs
"""

class Runner:
    def __init__(self) -> None:
        import player.window.main as main

        self.mainwindow = main.MainWindow()

    def run(self):
        self.mainwindow._run()

runner = Runner()

if __name__ == "__main__":
    runner.run()