#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <chrono>
#include <thread>

#include "client.h"
#include "message.h"
#include "socket.h"

const auto IP   = std::string("127.0.0.1");
const auto PORT = 9091;

void run() {
    int  i = 0;
    auto client = Client(IP, PORT);
    const std::vector<std::string> data = {"Hello", "World"};

    client.connect();
    while(1) {
        auto m = Message();

        m.set_id(i);
        m.set_flag(MessageType::LIST);
        m.set_repeat(data.size());
        for (auto& str: data) m.add_data(str);

        client.send(m);
        auto r = client.recv();
        std::cout << "RECV: \n" << r.DebugString() << std::endl;

        if ( (++i) == 10 ) 
            break;

        std::this_thread::sleep_for(std::chrono::milliseconds(500));
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
