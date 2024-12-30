#include "term.h"

const std::vector<std::string> menuOptions = {
    "List Slots",
    "Login Session",
    "Encrypt",
};

Term::Term(void): menu() {
    initscr();

    // no need to press enter to capture input
    raw();

    // don't echo while user types
    noecho();


    // make cursor invisible
    noecho();

    // enable arrow keys and function keys
    keypad(stdscr, TRUE);

    // make cursor invisible
    curs_set(0);

    // get maxyx
    getmaxyx(stdscr, this->row, this->col);

    this->window = stdscr;
}

void Term::drawBorders() {
    box(window, 0, 0);
}

void Term::drawMenu() {
   for (size_t i = 0; i < menu.options.size(); ++i) {
        if (i == menu.cur) attron(A_REVERSE);
        mvprintw(2 + i, 2, menu.options[i].c_str());
        if (i == menu.cur) attroff(A_REVERSE);
    }
}

void Term::drawKeys() {
    mvprintw(this->row - 2, 2, "q: exit | j: up | k: down | Enter: select");
}

int Term::draw() {
    this->drawBorders();
    this->drawMenu();
    this->drawKeys();

    return 0;
}

int Term::refresh() {
    getmaxyx(stdscr, this->row, this->col);
    return wrefresh(this->window);
}

int Term::input() {
    return getch();
}

int Term::handle(int ch) {
    switch (ch) {
        case 'j':
            menu.cur = (menu.cur == (menu.options.size() - 1)) ? 0 : menu.cur + 1; 
            break;

        case 'k':
            menu.cur = (menu.cur == 0) ? menu.options.size() - 1 : menu.cur - 1;
            break;

        case 10:
            mvprintw(LINES - 2, 2, "Selected: %s", menu.options[menu.cur].c_str());
            break;
    }

    return 0;
}

Term::~Term() {
    endwin();
}
