import tkinter as tk
import SendData as SD
import time 
CONNECTION_ERROR = "Not Connected to Pi, cannot send data"
addr = "192.168.0.3"
port = 2000        
class Application(tk.Frame):
    def __init__(self, master=None): #initializes the window
        super().__init__(master)
        self.master = master #sets the master to root
        self.pack(fill = 'both', expand='true') #fills the background 
        self.text1 = tk.StringVar()
        self.text_color = tk.StringVar()
        self.text1.set("Connection Status: Not Connected")
        self.text_color.set("red")
        self.configure(bg = 'snow3')
        self.net = SD.Network(addr,port)
        for x in range(1,4):
            tk.Grid.columnconfigure(self,x,weight = 1)
            tk.Grid.rowconfigure(self,x,weight = 1) 
        
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
                                        command =self.connect, width = 20)
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
        if self.net.connection == True:
            print("Up Arrow Pressed")
            self.net.sdata[1] = 3.2
        else:
            print(CONNECTION_ERROR)

    def down_arrow_press(self):
        if self.net.connection == True:
            print("Down Arrow Pressed")
            self.net.sdata[2] = 3.2
        else:
            print(CONNECTION_ERROR)

    def right_arrow_press(self):
        if self.net.connection == True:
            print("right Arrow Pressed")
            self.net.sdata[3] = 3.2
        else:
            print(CONNECTION_ERROR)

    def left_arrow_press(self):
        if self.net.connection == True:
            print("left Arrow Pressed")
            self.net.sdata[4] = 3.2
        else:
            print(CONNECTION_ERROR)

    def test_button_press(self): 
        if self.net.sdata[0] > 3.0:
            self.net.sdata[0] = 0.01
        else:
            self.net.sdata[0] = 3.2

    def connect(self):
        if self.net.connection == False:
            try:
                self.net.comm_init()
                if self.net.connection == True:
                    self.text1.set("Connection Status: Connected") 
                    self.connection_status.config(fg = "green")
            except:
                print("There was an error in the connection")
    
    def disconnect(self):
        try:
            self.net.disconnect()
            print("Connection closed successfully")
            if self.net.connection == False:
                self.text1.set("Connection Status: Not Connected") 
                self.connection_status.config(fg = "red")
        except: 
            print("There was an error in closing the connection")

root = tk.Tk()
root.geometry("1000x900")

app = Application(master=root)

app.net.rdata = [0.02, 0.03, 0.04, 0.05, 0.06]
app.net.sdata = [0.02, 0.03, 0.04, 0.05, 0.06]

while True:
    root.update_idletasks()
    root.update()
    if app.net.connection == True:
        try:
            app.net.data_exchange()
            print(app.net.rdata)
        except:
            print("There was an error in sending data")
    time.sleep(5*0.001)
    

        
