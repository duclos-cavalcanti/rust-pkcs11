#ifndef __LOG__H
#define __LOG__H

#include <iostream>
#include <fstream>
#include <memory>
#include <string>
#include <stdexcept>

enum class Level {
    INFO,
    EVENT,
    URGENT
};

std::string levelToString(Level level);

class Writer {
public:
    virtual ~Writer() = default;
    virtual void write(const std::string& message, Level level) = 0;
};

class FileWriter: public Writer {
    private: 
        std::ofstream file;
    
    public: 
        explicit FileWriter(const std::string& fpath) {
            file.open(fpath, std::ios::out | std::ios::app);
            if (!file.is_open()) {
                throw std::runtime_error("Failed to open file: " + fpath);
            }
        }

        void write(const std::string& text, Level level) override {
            file << "[" << levelToString(level) << "]: " << text << std::endl;

        }
};

class ConsoleWriter: public Writer {
    public: 
        void write(const std::string& text, Level level) override {
            std::cout << "[" << levelToString(level) << "]: " << text << std::endl;

        }
};


class Logger {
private:
    std::unique_ptr<Writer> writer;

public:
    explicit Logger(const std::string& fpath = "") {
        if (!fpath.empty()) this->writer = std::unique_ptr<FileWriter>{new FileWriter(fpath)};
        else                this->writer = std::unique_ptr<ConsoleWriter>{new ConsoleWriter};
    }

    void log(const std::string& message, Level level = Level::INFO) {
        writer->write(message, level);
    }
};

#endif /* __LOG__H */
