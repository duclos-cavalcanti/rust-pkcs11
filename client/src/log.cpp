#include "log.h"

std::string levelToString(Level level) {
    switch (level) {
        case Level::INFO:   return "INFO";
        case Level::EVENT:  return "EVENT";
        case Level::URGENT: return "URGENT";
        default:            throw std::invalid_argument("Invalid log level");
    }
}
