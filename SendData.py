#temporary file that is currently incomplete

import socket
from struct import pack, unpack
from bitstring import BitArray

def comm_init(IP,Port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(5)
    try:
        s.connect(IP,Port)
        return s
    except ConnectionError:
        return -1
    except TimeoutError:
        return -2


def send_data(s, Data):
    l = len(Data)
    try:
        for x in range(l):
            info += s.send(pack('f',Data[x]))
        return 1
    except ConnectionError:
        return -1
    except TimeoutError:
        return -2
        

def recieve_data(s, Data):
    BufferSize = 1024
    try:
        Buffer= s.recv(BufferSize)
        Data = unpack('f',Buffer)
        return Data
    except ConnectionError:
        return -1

def binary_rep(Data):
    A = pack('f',Data)
    A = BitArray(A).bin
    return A



