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
        fprintf(stderr, "Failed to send\n");
        exit(EXIT_FAILURE);
    }
    return 0;
}

int recv(int sockfd) {
    int n = 0;
    if ( (n = read(sockfd, rx, BUFFER_SIZE)) < 0) {
        fprintf(stderr, "Failed to read\n");
        exit(EXIT_FAILURE);
    }
    return n;
}

int connect() {
    int sockfd;
    if ( (sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0 ) {
        fprintf(stderr, "Failed to create socket\n");
        exit(EXIT_FAILURE);
    }

    addr.sin_family       = AF_INET;
    addr.sin_port         = htons(PORT);
    addr.sin_addr.s_addr  = inet_addr("127.0.0.1");

    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        fprintf(stderr, "Failed TCP connection\n");
        exit(EXIT_FAILURE);
    }

    printf("CONNECTED: IP: %s\n", inet_ntoa(addr.sin_addr));
    return sockfd;
}

int client() {
    int  i = 0, n, sockfd = connect();
    std::cout << "CLIENT CONNECTED: " << std::endl;

    while(1) {
        ProtoMessage::ProtoMessage m;

        m.set_id(i);
        m.set_data("Hello World");

        std::string data;
        if (!m.SerializeToString(&data)) {
            fprintf(stderr, "Failed message serialization\n");
            exit(EXIT_FAILURE);
        }

        send(sockfd, data);

        if ( (++i) == 10 ) 
            break;

        std::this_thread::sleep_for(std::chrono::milliseconds(500));
    }

    close(sockfd);
    return 0;
}

int main(int argc, char* argv[]) {
    int ret = client();
    google::protobuf::ShutdownProtobufLibrary();
    return ret;
}
