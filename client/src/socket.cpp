#include "socket.h"

#include <iostream>
#include <fstream>

Socket::Socket(const std::string& ipaddr, int port) {
    int ret;
    if ( (ret = socket(AF_INET, SOCK_STREAM, 0)) < 0 ) {
        throw std::runtime_error("Failed to create socket");
    }

    std::memset(&(this->_sockaddr), 0, sizeof(this->_sockaddr));

    this->_sockfd                    = ret;
    this->_sockaddr.sin_family       = AF_INET;
    this->_sockaddr.sin_port         = htons(port);
    this->_sockaddr.sin_addr.s_addr  = inet_addr(ipaddr.c_str());

    if (inet_pton(AF_INET, ipaddr.c_str(), &(this->_sockaddr.sin_addr)) <= 0) {
        throw std::runtime_error("Failed address format");
    }
}

int Socket::connect() {
    int ret = ::connect(_sockfd, (struct sockaddr*)&(this->_sockaddr), sizeof(this->_sockaddr));
    if (ret < 0) {
        throw std::runtime_error("Failed connection");
    }

    return ret;
}

int Socket::send(const std::string& data) {
    int ret;
    if ( (ret = ::send(this->_sockfd, data.data(), data.size(), 0)) < 0 ) {
        throw std::runtime_error("Failed to send");
    }
    return ret;
}

int Socket::recv(char* buf, int size) {
    int ret = read(this->_sockfd, buf, size);
    if ( ret < 0 ) {
        throw std::runtime_error("Failed to read");
    }
    return ret;
}

std::string Socket::addr() {
    return std::string{inet_ntoa(this->_sockaddr.sin_addr)};
}

Socket::~Socket() {
    if (this->_sockfd >= 0) {
        close(this->_sockfd);
    }
}
