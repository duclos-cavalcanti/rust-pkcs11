#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <chrono>
#include <thread>

#include "client.h"
#include "term.h"
#include "message.h"
#include "socket.h"

const auto IP   = std::string("127.0.0.1");
const auto PORT = 9091;

void run() {
    // Client client(IP, PORT);
    Term   term{};

    while(1) {
        term.draw();
        term.refresh();
        int ch = term.input();
        
        if (ch == 'q') 
            break;

        term.handle(ch);
    }
}

int main(int argc, char* argv[]) {
    try {
        run();
        google::protobuf::ShutdownProtobufLibrary();
        return 0;

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
}
