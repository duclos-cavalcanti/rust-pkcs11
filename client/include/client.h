#ifndef __CLIENT__H
#define __CLIENT__H

#include "socket.h"
#include "message.h"
#include "log.h"

#define BUFFER_SIZE 1024

class Client {
public:
    Client(const std::string& ipaddr, int port);
    ~Client();

    int     connect();
    int     send(const Message& m);
    Message recv();
    Message exchange(const Message& m);

    Message list(void);

private:
    int     _tick;
    Socket _socket;
    Logger _logger;
    char   _buffer[BUFFER_SIZE];
};

#endif /* __CLIENT__H */
