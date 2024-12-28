#include "client.h"

#include <iostream>
#include <fstream>

Client::Client(const std::string& ipaddr, int port): _socket(ipaddr, port) {
    std::cout << "CLIENT UP" << std::endl;
}

int Client::connect() {
    int ret = this->_socket.connect();
    std::cout << "CLIENT CONNECTED TO: " << this->_socket.addr() << std::endl;
    return ret;
}

int Client::send(const Message& m) {
    std::string data;
    if (!m.SerializeToString(&data)) {
        throw std::runtime_error("Failed to serialize message");
    }
    return this->_socket.send(data);
}


Message Client::recv() {
    int  n = this->_socket.recv(this->_buffer, BUFFER_SIZE);
    auto m = Message();

    if (!m.ParseFromArray(this->_buffer, n)) {
        throw std::runtime_error("Failed to deserialize message");
    }
    return m;
}

Client::~Client() {
    std::cout << "CLIENT CLOSED SOCKET" << std::endl;
}
