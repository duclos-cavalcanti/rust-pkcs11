#include "client.h"

#include <iostream>
#include <fstream>

Client::Client(const std::string& ipaddr, int port)
    : _tick(0), _socket(ipaddr, port), _logger("CLIENT.LOG") {
    this->_logger.log("CLIENT ON");
    this->connect();
}

int Client::connect() {
    int ret = this->_socket.connect();
    this->_logger.log("CLIENT CONNECTED TO " + this->_socket.addr());
    return ret;
}

int Client::send(const Message& m) {
    std::string data;
    if (!m.SerializeToString(&data)) {
        throw std::runtime_error("Failed to serialize message");
    }
    int n = this->_socket.send(data);
    this->_logger.log("CLIENT SENT: \n" + m.DebugString(), Level::EVENT);
    return n;
}


Message Client::recv() {
    int  n = this->_socket.recv(this->_buffer, BUFFER_SIZE);
    auto m = Message();

    if (!m.ParseFromArray(this->_buffer, n)) {
        throw std::runtime_error("Failed to deserialize message");
    }

    this->_logger.log("CLIENT RECV: \n" + m.DebugString(), Level::EVENT);
    return m;
}

Message Client::exchange(const Message& m) {
    this->send(m);
    return this->recv();
}

Message Client::list(void) {
    auto m = Message();
    m.set_id(1);
    m.set_flag(MessageType::LIST);
    m.set_repeat(0);
    m.set_err(false);
    return m;
}

Client::~Client() {
    this->_logger.log("CLIENT CLOSED SOCKET");
}
