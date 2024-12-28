#ifndef SOCKET_H
#define SOCKET_H

#include <string>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <stdexcept>
#include <cstring>

class Socket {
public:
    Socket(const std::string& ipaddr, int port);
    ~Socket();

    int connect();
    int send(const std::string& data);
    int recv(char* buf, int size);

    std::string addr();
private:
    int  _sockfd;
    struct sockaddr_in _sockaddr;
};

#endif // SOCKET_H
