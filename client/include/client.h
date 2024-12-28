#ifndef __CLIENT__H
#define __CLIENT__H

#include "socket.h"
#include "message.h"

#define BUFFER_SIZE 1024

class Client {
public:
    Client(const std::string& ipaddr, int port);
    ~Client();

    int     connect();
    int     send(const Message& m);
    Message recv();

private:
    Socket _socket;
    char   _buffer[BUFFER_SIZE];
};

#endif /* __CLIENT__H */
