import tkinter as tk

class HoverButton(tk.Button):
    """
    A class to change the colour of buttons to the accent colour when hovered over.
    """
    def __init__(self, master, **kw):
        tk.Button.__init__(self, master=master, **kw)
        self.defaultForeground = self["foreground"]
        self.bind("<Enter>", self.on_enter)
        self.bind("<Leave>", self.on_leave)

    def on_enter(self, e):
        self['foreground'] = self["activeforeground"]

    def on_leave(self, e):
        self['foreground'] = self.defaultForeground