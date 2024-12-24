#include <iostream>
#include <fstream>
#include <string>
#include "message.pb.h"

#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>

int main(int argc, char* argv[]) {
  std::cout << "Hello World" << std::endl;

  system("sleep 10s");

  std::cout << "After Sleep" << std::endl;

  return 0;
}
