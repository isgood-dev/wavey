import tkinter as tk

import player.utils.data as data

class ScrollbarFrame(tk.Frame):
    """Extends class tk.Frame to support a scrollable Frame 
    This class is independent from the widgets to be scrolled and 
    can be used to replace a standard tk.Frame"""

    def __init__(self, parent, **kwargs):
        tk.Frame.__init__(self, parent, **kwargs)
        self.count = 0

        # The Canvas which supports the Scrollbar Interface, layout to the left
        self.canvas = tk.Canvas(self, borderwidth=0, background=data.view(
            "songlist_colour", "c"), highlightthickness=0, height=498, width=650)
        self.canvas.pack(side="left", fill="both", expand=True)
        self.canvas.bind_all("<MouseWheel>", self._on_mousewheel)

        # The Frame to be scrolled, layout into the canvas
        # All widgets to be scrolled have to use this Frame as parent
        self.scrolled_frame = tk.Frame(
            self.canvas, background=data.view("songlist_colour", "c"))
        self.canvas.create_window(
            (4, 4), window=self.scrolled_frame, anchor="nw")
        

        # Configures the scrollregion of the Canvas dynamically
        self.scrolled_frame.bind("<Configure>", self.on_configure)