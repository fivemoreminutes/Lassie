import socket
from struct import pack, unpack
from bitstring import BitArray

# Initiate communication with ras-pi at it's IP and port number

class Network():
    def __init__(self):
        self.connection = False
        self.rdata = []
        self.sdata = []
        self.PORT = 2000
        self.addr = "192.168.1.3"
    
    def comm_init(self):
        itr = 1
        while True:
            self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            try:  # if there is an error it should return the error type to main
                self.s.connect((self.addr,self.PORT))
                self.connection = True
            except ConnectionError:
                print("There was an error connecting to pi")
                self.connection = False
            except TimeoutError:
                print("The connection timed out")
                self.connection = False

            if self.connection == True:
                break
            else:
                print("Connection Attempt: ", itr)
                itr += 1
    
    def disconnect(self):
        if self.connection == True:
            self.s.close()
            self.connection = False
    
    def data_exchange(self):
        info = 0
        bytes_s = len(self.sdata)*4
        try:   

            info += self.s.send(b'star')
            for x in range(len(self.sdata)):
                info += self.s.send(pack('f', self.sdata[x]))  
            info += self.s.send(b'done')   
                
        except ConnectionError:
            print("Connection Error")
            #self.connection = False
        except TimeoutError:
            print("There was a timeout")
            #self.connection = False

        BufferSize = 4
        try:
            buffer = self.s.recv(BufferSize)
            first = buffer
            temp_data = []
            if first == b'star':
                while True:
                    buffer = self.s.recv(BufferSize)
                    last = buffer
                    if last == b'done':
                        self.rdata = temp_data
                        break
                    elif last ==b'star':
                        break
                    else:   
                        temp = list(unpack('f',buffer))                     
                        temp_data.append(temp[0])                     

        except ConnectionError:
            print("Error Recieving data")
    
    # This function is just here right now for future debugging of binary data if needed
    def binary_rep(Data):
        A = pack('f', Data)
        A = BitArray(A).bin
        return A


