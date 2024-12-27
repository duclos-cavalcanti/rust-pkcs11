#include <iostream>
#include <fstream>
#include <string>
#include <chrono>
#include <thread>
#include "message.h"

#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>

#define PORT 9091
#define BUFFER_SIZE 1024

static struct sockaddr_in addr;
static char rx[BUFFER_SIZE] = { 0 };

int send(int sockfd, const std::string& data) {
    if ( (send(sockfd, data.data(), data.size(), 0)) < 0 ) {
        std::cerr << "Failed to send" << std::endl;
        exit(EXIT_FAILURE);
    }
    return 0;
}

int recv(int sockfd) {
    int n = 0;
    if ( (n = read(sockfd, rx, BUFFER_SIZE)) < 0) {
        std::cerr << "Failed to read" << std::endl;
        exit(EXIT_FAILURE);
    }
    return n;
}

int connect() {
    int sockfd;
    if ( (sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0 ) {
        std::cerr << "Failed to create socket" << std::endl;
        exit(EXIT_FAILURE);
    }

    addr.sin_family       = AF_INET;
    addr.sin_port         = htons(PORT);
    addr.sin_addr.s_addr  = inet_addr("127.0.0.1");

    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        std::cerr << "Failed TCP connection" << std::endl;
        exit(EXIT_FAILURE);
    }

    std::cout << "CONNECTED TO IP: " << inet_ntoa(addr.sin_addr) << std::endl;
    return sockfd;
}

int client() {
    int  i = 0, n, sockfd = connect();
    std::vector<std::string> data = { "Hello World" };

    while(1) {
        auto m = message(i, ProtoMessage::ProtoMessageType::ACK, data=data);
        auto r = ProtoMessage::ProtoMessage();
        std::string data;

        if (!m.SerializeToString(&data)) {
            std::cerr << "Failed message serialization" << std::endl;
            exit(EXIT_FAILURE);
        }

        send(sockfd, data);

        n = recv(sockfd);
        if (!r.ParseFromArray(rx, n)) {
            std::cerr << "Failed message deserialization" << std::endl;
            exit(EXIT_FAILURE);
        }

        std::cout << "RECV: " << r.DebugString() << std::endl;
        if ( (++i) == 10 ) 
            break;

        std::this_thread::sleep_for(std::chrono::milliseconds(500));
    }

    close(sockfd);
    std::cout << "CLOSED SOCKET" << std::endl;

    return 0;
}

int main(int argc, char* argv[]) {
    int ret = client();
    google::protobuf::ShutdownProtobufLibrary();
    return ret;
}
