import subprocess
from tkinter import messagebox

branch = "origin"
repo = "https://github.com/acatiadroid/music-player"

def update():
    ask = messagebox.askyesno(
        title="Are you sure?",
        message="Are you sure?\n\nGit must be installed on your computer for this to work."
    )
    
    if not ask:
        return
    
    PIPE = subprocess.PIPE

    subprocess.run(["git", "remote", "add", branch, repo])

    process = subprocess.Popen(["git", "pull", branch], stdout=PIPE, sterr=PIPE)
    stdoutput, stderroutput = process.communicate()

    if "fatal" in stdoutput:
        messagebox.showerror(
            title="Git not installed!",
            message="Cannot perform update as Git is not installed on your machine.\n\nInstall it here: https://git-scm.com/download/win"
        )
    else:
        messagebox.showinfo(
            title="Update has been installed!",
            message="The music player has been updated! Please restart the music player."
        )