import tkinter as tk
import tkinter.scrolledtext as st
import SendData as SD
import time 
CONNECTION_ERROR = "Not Connected to Pi, cannot send data"
addr = "192.168.0.3"
port = 2000        
SCRIPT = "[GUI] "
INFO = "[INFO] "
WARNING = "[WARNING] "

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

        for x in range(1,10):
            tk.Grid.columnconfigure(self,x,weight = 1)
        for x in range(1,10):
            tk.Grid.rowconfigure(self,x,weight = 1) 
        
        self.right_arrow = tk.Button(self, text = "\u2192", 
                                     command=self.right_arrow_press, 
                                     font = ("Courier",40) )
        self.left_arrow = tk.Button(self, text = "\u2190", 
                                     command=self.left_arrow_press, 
                                     font = ("Courier",40) ) 
        self.up_arrow = tk.Button(self, text = "\u2191", 
                                     command=self.up_arrow_press, 
                                     font = ("Courier",40) ) 
        self.down_arrow = tk.Button(self, text = "\u2193",
                                     command=self.down_arrow_press, 
                                     font = ("Courier",40) )
        self.test_button = tk.Button(self, text = "Test", 
                                     command=self.test_button_press,
                                     font = ("Courier",40) )


        self.connect_button = tk.Button(self, text = "Connect To Pi", command =self.connect)
        self.disconnect_button = tk.Button(self, text = "Disconnect from Pi", command =self.disconnect)
        self.connection_status = tk.Label(self, textvariable = self.text1, fg = self.text_color.get())
        self.output_term = st.ScrolledText(self, wrap = tk.WORD)

        self.quit = tk.Button(self, text="QUIT", fg="red",
                              command=self.master.destroy)
        
        self.quit.grid(row = 0, column = 0)

        self.connect_button.grid(row = 0, column = 9, sticky = 'nswe',columnspan = 4)
        self.disconnect_button.grid(row = 0, column = 5, sticky = 'nswe', columnspan = 4)
        self.connection_status.grid(row = 0, column = 1, sticky = 'nswe', columnspan = 4)

        self.right_arrow.grid(row = 4, column = 5, sticky = 'nswe', rowspan = 2, columnspan = 2)
        self.left_arrow.grid(row = 4, column = 1, sticky = 'nswe', rowspan = 2, columnspan = 2)
        self.up_arrow.grid(row = 2, column = 3, sticky = 'nswe', rowspan = 2, columnspan = 2)
        self.down_arrow.grid(row = 6, column = 3, sticky = 'nswe', rowspan = 2, columnspan = 2)
        self.test_button.grid(row=4, column = 3, sticky = 'nswe', rowspan = 2, columnspan = 2)
        
        self.output_term.grid(row = 1, column = 8, sticky = 'nswe',columnspan = 4, rowspan = 8)

        for x in range(1,12):
            self.grid_columnconfigure(x, minsize = 100)
        for x in range(1,9):   
            self.grid_rowconfigure(x, minsize = 100)
        
        self.output_term.insert(tk.INSERT, ">>")


    def up_arrow_press(self):
        
        if self.net.connection == True:
            message = "Up Arrow Pressed"
            output = INFO + SCRIPT + message + "\n" +">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

            self.net.sdata[1] = 3.2
        else:
            message = CONNECTION_ERROR
            output = WARNING + SCRIPT + message + "\n" +">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

    def down_arrow_press(self):

        if self.net.connection == True:
            message = "down Arrow Pressed"
            output = INFO + SCRIPT + message + "\n" +">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

            self.net.sdata[2] = 3.2
        else:
            message = CONNECTION_ERROR
            output = WARNING + SCRIPT + message + "\n"+">>"
            self.output_term.insert(tk.INSERT, output)   
            self.output_term.see(tk.END)         

    def right_arrow_press(self):

        if self.net.connection == True:
            message = "right Arrow Pressed"
            output = INFO + SCRIPT + message + "\n"+">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

            self.net.sdata[3] = 3.2
        else:
            message = CONNECTION_ERROR
            output = WARNING + SCRIPT + message + "\n"+">>"
            self.output_term.insert(tk.INSERT, output)  
            self.output_term.see(tk.END)          


    def left_arrow_press(self):

        if self.net.connection == True:
            message = "left Arrow Pressed"
            output = INFO + SCRIPT + message + "\n"  +">>"          
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

            self.net.sdata[4] = 3.2
        else:
            message = CONNECTION_ERROR
            output = WARNING + SCRIPT + message + "\n"+">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)            

    def test_button_press(self): 

        message = "Test Pressed"
        output = INFO + SCRIPT + message + "\n"+">>"
        self.output_term.insert(tk.INSERT, output)
        self.output_term.see(tk.END)
        
        
        if self.net.sdata[0] > 3.0:
            self.net.sdata[0] = 0.01
        else:
            self.net.sdata[0] = 3.2

    def connect(self):
        if self.net.connection == False:
            try:
                itr = 1
                while True:
                    script = "[Send Data] "
                    message = "Connection attempt: " + str(itr)
                    output = INFO + script + message + "\n"+">>"
                    self.output_term.insert(tk.INSERT, output)
                    self.output_term.see(tk.END)
                    self.update()

                    output = self.net.comm_init(itr)
                    if output == "T":
                        break
                    else:
                        itr += 1
                        self.output_term.insert(tk.INSERT, output)
                        self.output_term.see(tk.END)
                        self.update()
                    
                    if itr > 5:
                        message = "Could not connect to Lassie"
                        output = WARNING + SCRIPT + message + "\n"+">>"

                        self.output_term.insert(tk.INSERT, output)
                        self.output_term.see(tk.END)
                        break
                
                if self.net.connection == True:
                    message = "Connected to Lassie"
                    output = INFO + SCRIPT + message + "\n"+">>"
                    self.output_term.insert(tk.INSERT, output)
                    self.output_term.see(tk.END)

                    self.text1.set("Connection Status: Connected") 
                    self.connection_status.config(fg = "green")
            except:
                message = "There was an error connecting to Lassie"
                output = WARNING + SCRIPT + message + "\n"+">>"

                self.output_term.insert(tk.INSERT, output)
                self.output_term.see(tk.END)

    
    def disconnect(self):
        try:
            self.net.disconnect()

            message = "Disconnected from Lassie"
            output = INFO + SCRIPT + message + "\n"+">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

            if self.net.connection == False:
                self.text1.set("Connection Status: Not Connected") 
                self.connection_status.config(fg = "red")
        except: 
            message = "There was an error in closing the connection"
            output = INFO + SCRIPT + message + "\n"+">>"
            self.output_term.insert(tk.INSERT, output)
            self.output_term.see(tk.END)

root = tk.Tk()
#root.geometry("1300x900")

app = Application(master=root)

app.net.rdata = [0.02, 0.03, 0.04, 0.05, 0.06]
app.net.sdata = [0.02, 0.03, 0.04, 0.05, 0.06]

while True:
    root.update_idletasks()
    root.update()

    if app.net.connection == True:
        try:
            output = app.net.data_exchange()
            if output != None
                root.output_term.insert(tk.INSERT, output)
                root.output_term.see(tk.END)

        except:
            label = "[WARNING] "
            message = "There was an error in data exchange"
            output = label + SCRIPT + message + "\n"+">>"

            app.output_term.insert(tk.INSERT, output)
            app.output_term.see(tk.END)
    time.sleep(5*0.001)