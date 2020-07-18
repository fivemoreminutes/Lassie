import tkinter as tk
import SendData as SD

class Application(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)
        self.master = master
        self.pack(fill = 'both', expand='true')
        self.create_widgets()
        self.configure(bg = 'snow3')
        for x in range(1,4):
            tk.Grid.columnconfigure(self,x,weight = 1)
            tk.Grid.rowconfigure(self,x,weight = 1) 

    def create_widgets(self):
        
        self.right_arrow = tk.Button(self, text = "\u2192", 
                                     command=self.right_arrow_press, height= 5, width = 10 )
        self.left_arrow = tk.Button(self, text = "\u2190", 
                                     command=self.left_arrow_press, height= 5, width = 10 ) 
        self.up_arrow = tk.Button(self, text = "\u2191", 
                                     command=self.up_arrow_press, height= 5, width = 10 ) 
        self.down_arrow = tk.Button(self, text = "\u2193",
                                     command=self.down_arrow_press, height= 5, width = 10 )
        self.test_button = tk.Button(self, text = "Test", 
                                     command=self.test_button_press, height= 5, width = 10 )

        self.quit = tk.Button(self, text="QUIT", fg="red",
                              command=self.master.destroy)
        self.quit.grid(row = 0, column = 0)

        self.right_arrow.grid(row = 2, column = 3, sticky = 'nswe')
        self.left_arrow.grid(row = 2, column = 1, sticky = 'nswe')
        self.up_arrow.grid(row = 1, column = 2, sticky = 'nswe')
        self.down_arrow.grid(row = 3, column = 2, sticky = 'nswe')
        self.test_button.grid(row=2, column = 2, sticky = 'nswe')

    def up_arrow_press(self):
        print("Up Arrow Pressed")

    def down_arrow_press(self):
        print("Down Arrow Pressed")

    def right_arrow_press(self):
        print("right Arrow Pressed")

    def left_arrow_press(self):
        print("left Arrow Pressed")

    def test_button_press(self):
        print("test")
    

root = tk.Tk()
root.geometry("500x400")

app = Application(master=root)

app.mainloop()
#New Environment New Me