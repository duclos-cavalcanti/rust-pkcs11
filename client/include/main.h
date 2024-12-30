#ifndef __MAIN__H
#define __MAIN__H

#include <string>
#include <map>

enum STATE {
  START = 0,
  LIST,
  LOGIN, 
  ENCRYPT,
  EXIT
};

struct REQUEST {
    STATE state;
    std::string input;
};

#endif /* __MAIN__H */
