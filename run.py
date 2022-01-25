"""
This file will run the music player.

Please refer to the README.md for guidance on how to set up this music player - you may need to install some dependencies.

Please respect the GPL v3.0 license in the LICENSE file.

Created by acatia (acatia#5378)

Check out misc/help.mdown if you need help with something or join the Discord server:
https://discord.gg/p5bURjs
"""

from tkinter import Tk
from music_player.config import view
def main():
    root = Tk()
    from music_player.main_window import MainWindow
    app = MainWindow(root)
    app.configure(bg=view("back_colour"))
    root.iconbitmap("Assets/musical_note.ico")
    root.wm_title("Music Player")
    root.geometry("600x400")
    root.resizable(True, True)
    root.mainloop()

if __name__ == "__main__":
    main()
