import socket
import sys
from thread import *

#HOST = 'flip3.engr.oregonstate.edu' #symbolic name meaning all available interfaces
#PORT = 30021
HOST = ''
PORT = 8888

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print 'Socket created'

try:
    server_socket.bind((HOST, PORT))    #bind to a address(and port)
except socket.error, msg:
    print 'Bind failed. Error Code : ' + str(msg[0]) + ' Message ' + msg[1]
    sys.exit()

print 'Socket bind complete'

#put the socket in listening mode
server_socket.listen(10)     #maximum 10 connections
print 'TCP Server Waiting for client on port 8888'

#wait to accept a connection - blocking call
client, addr = server_socket.accept()
#display client information
print 'Connected with ' + addr[0] + ':' + str(addr[1])

#keep talking with the client

    #Receiving from client

reply = 'wrong command\r\n'

client.send(reply)
print "sent"

client.close()
server_socket.close()