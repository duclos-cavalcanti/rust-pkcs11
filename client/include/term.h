#ifndef __TERM__H
#define __TERM__H

#include <vector>
#include <string>

#include <ncurses.h>

class MENU {
public:
    MENU(void): cur(0) { };
    ~MENU() = default;

    int cur;
    const std::vector<std::string> options = {
        "List Slots",
        "Login Session",
        "Encrypt",
    };
};

class Term {
public:
    Term(void);
    ~Term();

    int draw(void);
    int refresh(void);
    int input(void);
    int handle(int ch);

private:
    void drawBorders(void);
    void drawMenu(void);
    void drawKeys(void);

    WINDOW* window;
    MENU menu;

    int row, col;
};

#endif /* __TERM__H */
