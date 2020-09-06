import socket
import numpy as np
from struct import pack, unpack
from bitstring import BitArray
import tkinter as tk

SCRIPT = "[Send Data] "
INFO = "[INFO] "
WARNING = "[WARNING] "


# Initiate communication with ras-pi at it's IP and port number
class Network():
    def __init__(self, addr, port):
        self.connection = False
        self.rdata = []
        self.sdata = []
        self.PORT = port
        self.addr = addr
        self.s = None 

    def comm_init(self,itr):
        while True:
            self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            try:  # if there is an error it should return the error type to main
                self.s.connect((self.addr,self.PORT))
                self.connection = True
                return "T"
            except ConnectionError:
                Message = "There was a Connection Error"
                self.connection = False
                return WARNING + SCRIPT + Message + "\n>>"
            except TimeoutError:
                Message = "There was a Timeout Error"
                self.connection = False
                return WARNING + SCRIPT + Message + "\n>>"
            except:
                Message = "Unclassified Error"
                return WARNING + SCRIPT + Message + "\n>>"
    
    def disconnect(self):
        if self.connection == True:
            self.s.close()
            self.connection = False
    
    def data_exchange(self):

        try:   
            buffer = b'star'
            for x in range(len(self.sdata)):
                buffer = buffer+pack('f', self.sdata[x])
            buffer = buffer + b'done'  
            self.s.send(buffer)
                
        except ConnectionError:
            Message = "There was a Connection Error"
            return WARNING + SCRIPT + Message + "\n>>"

        except TimeoutError:
            Message = "There was a timeout Error"
            return WARNING + SCRIPT + Message + "\n>>"
        except:
            Message = "Unclassified Error"
            return WARNING + SCRIPT + Message + "\n>>"
        
        buffer = []
        BufferSize = 512
        try:
            temp_data = []
            buffer = self.s.recv(BufferSize)

            first = buffer[0:4]
            last = buffer[-4:]
            temp = []
            i = 4
            j = 8
            if bytes(first) == b'star':
                while True:
                    buf = buffer[i:j]

                    i += 4
                    j += 4
                    if buf == b'done':
                        self.rdata = temp_data
                        break
                    elif last ==b'star':
                        break
                    else:
                        temp = list(unpack('f',buf))
                        temp_data.append(temp[0])
            return None
        except ConnectionError:
            Message = "There was an Connection Error"
            return WARNING + SCRIPT + Message + "\n>>"
        except:
            Message = "Unclassified Error"
            return WARNING + SCRIPT + Message + "\n>>"
    
