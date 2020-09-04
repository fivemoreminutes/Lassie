import socket
import numpy as np
from struct import pack, unpack
from bitstring import BitArray

# Initiate communication with ras-pi at it's IP and port number
class Network():
    def __init__(self, addr, port):
        self.connection = False
        self.rdata = []
        self.sdata = []
        self.PORT = port
        self.addr = addr
        self.s = None 

    def comm_init(self):
        itr = 1
        while True:
            self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            try:  # if there is an error it should return the error type to main
                self.s.connect((self.addr,self.PORT))
                self.connection = True
            except ConnectionError:
                print("There was an error when connecting to pi")
                self.connection = False
            except TimeoutError:
                print("The connection timed out")
                self.connection = False
            except:
                print("There was an error when connecting to pi")
            if self.connection == True:
                break
            elif itr>5:
                print("There was an error when connecting to pi")
                break
            else:
                print("Connection Attempt: ", itr)
                itr += 1
    
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
            print("Connection Error")

        except TimeoutError:
            print("There was a timeout")
        except:
            print("There was an error in the data exchange function")
        
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

        except ConnectionError:
            print("Error Recieving data")
        except:
            print("There was an error in the data exchange file")
    
