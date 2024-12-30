#include "term.h"

#include <fstream>

const std::vector<std::string> menuOptions = {
    "List Slots",
    "Login Session",
    "Encrypt",
};

TERM::TERM(void): menu(), state(STATE::START) {
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
    getmaxyx(stdscr, this->rows, this->cols);

    this->window = stdscr;
}

void TERM::drawBorders() {
    box(window, 0, 0);
}

void TERM::drawMenu() {
   for (size_t i = 0; i < menu.options.size(); ++i) {
        if (i == menu.cur) attron(A_REVERSE);
        mvprintw(2 + i, 2, menu.options[i].c_str());
        if (i == menu.cur) attroff(A_REVERSE);
    }
}

void TERM::drawKeys() {
    mvprintw(this->rows - 2, 2, "q: exit | j: up | k: down | c: clear");
}

int TERM::draw() {
    this->drawBorders();
    this->drawMenu();
    this->drawKeys();
    return 0;
}

int TERM::refresh() {
    getmaxyx(stdscr, this->rows, this->cols);
    if (this->window) wrefresh(this->window);
    if (this->out)    wrefresh(this->out);
    return 0;
}

void TERM::clear(WINDOW* win) {
    werase(win);
}

int TERM::input() {
    return getch();
}

REQUEST TERM::handle(int ch) {
    switch (ch) {
        case 'j':
            menu.cur = (menu.cur == (menu.options.size() - 1)) ? 0 : menu.cur + 1; 
            break;

        case 'k':
            menu.cur = (menu.cur == 0) ? menu.options.size() - 1 : menu.cur - 1;
            break;

        case 'c':
            this->state = STATE::START;
            if (this->out) this->clear(this->out);
            break;

        case 10:
            this->state = static_cast<STATE>(menu.cur + 1);
            break;

        case 'q':
            this->state = STATE::EXIT;
            break;
    }

    return REQUEST{this->state, ""};
}

std::string TERM::capture(std::string prompt, int height, int width) {
    char str[256] = { 0 };
    WINDOW* win = newwin(height, 
                         width, 
                         (this->cols / 2) - (height / 2), 
                         (this->rows / 2) - (width / 2));

    box(win, 0, 0);
    mvwprintw(win, 1, 2, prompt.c_str());
    wmove(win, 2, 2);
    curs_set(1);
    wgetnstr(win, str, sizeof(str) - 1);
    curs_set(0);
    delwin(win);

    this->refresh();
    return std::string{str};
}

void TERM::output(REQUEST& req, Message& m) {
    if (!this->out)
        this->out = newwin(this->rows, 
                           this->cols / 2, 
                           0, 
                           this->rows / 2);

    box(this->out, 0, 0);
    for (int i=0; i<m.data_size(); i++) {
        mvwprintw(this->out, i + 1, 2, m.data(i).c_str());
    }
}

TERM::~TERM() {
    endwin();
}

