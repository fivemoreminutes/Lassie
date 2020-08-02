import tkinter as tk
import SendData as SD

IP = '192.168.0.1'
PORT = 55555
ConectionError = "Not Connected to Pi, cannot send data"

class Application(tk.Frame):
    def __init__(self, master=None): #initializes the window
        super().__init__(master)
        self.master = master #sets the master to root
        self.pack(fill = 'both', expand='true') #fills the background 
        self.text1 = tk.StringVar()
        self.text_color = tk.StringVar()
        self.text1.set("Connection Status: Not Connected")
        self.text_color.set("red")
        self.Connected = False
        self.configure(bg = 'snow3')
        self.Data = [0.01,0.01,0.01,0.01,0.01]
        for x in range(1,4):
            tk.Grid.columnconfigure(self,x,weight = 1)
            tk.Grid.rowconfigure(self,x,weight = 1) 

        self.create_widgets()

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

        self.connect_button = tk.Button(self, text = "Connect To Pi", 
                                        command =self.connection, width = 20)
        self.disconnect_button = tk.Button(self, text = "Disconnect from Pi", 
                                        command =self.disconnect, width = 20)
        self.connection_status = tk.Label(self, textvariable = self.text1, fg = self.text_color.get(),
                                         width = 20)

        self.quit = tk.Button(self, text="QUIT", fg="red",
                              command=self.master.destroy)
        
        self.quit.grid(row = 0, column = 0)
        self.right_arrow.grid(row = 2, column = 3, sticky = 'nswe')
        self.left_arrow.grid(row = 2, column = 1, sticky = 'nswe')
        self.up_arrow.grid(row = 1, column = 2, sticky = 'nswe')
        self.down_arrow.grid(row = 3, column = 2, sticky = 'nswe')
        self.test_button.grid(row=2, column = 2, sticky = 'nswe')
        self.connect_button.grid(row = 0, column = 3, sticky = 'nswe')
        self.disconnect_button.grid(row = 0, column = 2, sticky = 'nswe')
        self.connection_status.grid(row = 0, column = 1, sticky = 'nswe')

    def up_arrow_press(self):
        if self.Connected == True:
            print("Up Arrow Pressed")
            self.Data[1] = 3.2
        else:
            print(ConectionError)

    def down_arrow_press(self):
        if self.Connected == True:
            print("Down Arrow Pressed")
            self.Data[2] = 3.2
        else:
            print(ConectionError)

    def right_arrow_press(self):
        if self.Connected == True:
            print("right Arrow Pressed")
            self.Data[3] = 3.2
        else:
            print(ConectionError)

    def left_arrow_press(self):
        if self.Connected == True:
            print("left Arrow Pressed")
            self.Data[4] = 3.2
        else:
            print(ConectionError)

    def test_button_press(self):
        print("test")
        self.Data[0] = 3.2

    def disconnect(self):
        if self.Connected == True:
            self.s.close()
            self.Connected = False
            self.text1.set("Connection Status: Not Connected") 
            self.connection_status.config(fg = "red")
            s = a
        else:
            print("Not Connected to Pi")

    def connection(self):
        a = SD.comm_init(IP,PORT)

        if a == -1:
            print("There was an error connecting to pi")
        elif a == -2:
            print("The connection timed out")
        else:
            self.Connected = True
            self.text1.set("Connection Status: Connected") 
            self.connection_status.config(fg = "green")
            s = a

root = tk.Tk()
root.geometry("1000x900")

app = Application(master=root)

IP = "192.168.1.3"
PORT = 2000

app.s = SD.comm_init(IP,PORT)
app.Connected = True
app.text1.set("Connection Status: Connected") 
app.connection_status.config(fg = "green")

while True:
    root.update_idletasks()
    root.update()
    if app.Connected == True:
        try:
            rec_data = SD.data_exhcange(app.s,app.Data)
            print(rec_data)
        except:
            print("There was an error in sending data")
            print(app.s)

        
    #app.Data = [0.01,0.01,0.01,0.01,0.01]
        
