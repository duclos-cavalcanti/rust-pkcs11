#ifndef __CLIENT__H
#define __CLIENT__H

#include "socket.h"
#include "message.h"
#include "log.h"
#include "main.h"

#include <unordered_map>
#include <functional>

#define BUFFER_SIZE 1024

class Client {
public:
    Client(const std::string& ipaddr, int port);
    ~Client();

    int     connect();
    Message request(REQUEST& req);
private:
    int     send(const Message& m);
    Message recv();
    Message exchange(const Message& m);

    Message listRequest(void);

    int     _tick;
    Socket _socket;
    Logger _logger;
    char   _buffer[BUFFER_SIZE];

    std::unordered_map<STATE, std::function<Message(void)>> _map = {
        {STATE::LIST, [this]() { return this->listRequest(); }},
    };

};

#endif /* __CLIENT__H */
