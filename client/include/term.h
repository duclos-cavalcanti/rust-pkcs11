#ifndef __TERM__H
#define __TERM__H

#include "main.h"
#include "message.h"

#include <vector>
#include <memory>
#include <string>

#include <ncurses.h>

class TERM {
public:
    TERM(void);
    ~TERM();

    int draw(void);
    int refresh(void);
    int input(void);
    void clear(WINDOW* win);
    REQUEST handle(int ch);
    void    output(REQUEST& req, Message& m);

private:
    std::string capture(std::string prompt, int height = 5, int width = 40);

    void drawBorders(void);
    void drawMenu(void);
    void drawKeys(void);

    struct MENU {
        int cur;
        const std::vector<std::string> options = {
            "List Slots",
            "Login Session",
            "Encrypt",
        };
    };

    WINDOW* window;
    WINDOW* out = nullptr;
    MENU    menu;
    STATE   state;
    int     rows, cols;
};

#endif /* __TERM__H */
