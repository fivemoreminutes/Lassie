import socket
from struct import pack, unpack
from bitstring import BitArray

# Initiate communication with ras-pi at it's IP and port number

class Network():
    def __init__(self):
        self.connection = False
        self.rdata = []
        self.sdata = []
        self.PORT = 1000
        self.addr = "192.168.1.3"
    
    def comm_init(self):
        itr = 1
        while True:
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            try:  # if there is an error it should return the error type to main
                s.connect((self.addr,self.PORT))
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
    
    def data_exhcange(self):
        info = 0
        bytes_s = len(self.sdata)*4
        try:   
            print("Sending")

            info += self.s.send(pack('s',"star"))
            for x in range(len(Data)):
                info += self.s.send(pack('f', Data[x]))  
            info += self.s.send(pack('s',"done"))   

"""             if info == bytes_s+2:
                self.connection = True
            else:
                self.connection = True """
                
        except ConnectionError:
            self.connection = False
        except TimeoutError:
            self.connection = False

        BufferSize = 4
        try:
            print("Recieving")
            temp_data = []
            buffer = self.s.recv(BufferSize)
            first = unpack('s', buffer)
            if buffer == "star"
                while True:
                    buffer = self.s.recv(BufferSize)
                    last = unpack('s', buffer)
                    if last == "done"
                        break
                    elif last =="star"
                        break
                    else:
                        temp_data.append(unpack('f',buffer))
            if first == "star" & last == "done"
                self.rdata = temp_data

        except ConnectionError:
            print("Error Recieving data")

        if self.rdata[0] == 255.0 & self.rdata[-1] == 255.0
    
    # This function is just here right now for future debugging of binary data if needed
    def binary_rep(Data):
        A = pack('f', Data)
        A = BitArray(A).bin
        return A


