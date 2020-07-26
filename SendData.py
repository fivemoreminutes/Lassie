import socket
from struct import pack, unpack
from bitstring import BitArray

# Initiate communication with ras-pi at it's IP and port number



def comm_init(IP, Port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(5)
    try:  # if there is an error it should return the error type to main
        s.connect((IP, Port))
        return s
    except ConnectionError:
        return -1
    except TimeoutError:
        return -2

def comm_close(s):
    s.close()


def InfoHandle(Data):
    CompiledData = [255.0, 0, 255.0]

    return CompiledData

# These functions are egregiously untested and need to be checked
 
def send_data(s, Data):
    l = len(Data)
    Data = InfoHandle(Data)
    try:
        # I am not sure which of these functions will be better, though I assume it will be this one
        s.send(pack('f', Data))

        # for x in range(l):
        #    info += s.send(pack('f', Data[x]))
        # return 1
    except ConnectionError:
        return -1
    except TimeoutError:
        return -2


def recieve_data(s, Data):
    BufferSize = 1024
    try:
        Buffer = s.recv(BufferSize)
        Data = unpack('f', Buffer)
        return Data
    except ConnectionError:
        return -1

# This function is just here right now for future debugging of binary data if needed


def binary_rep(Data):
    A = pack('f', Data)
    A = BitArray(A).bin
    return A
