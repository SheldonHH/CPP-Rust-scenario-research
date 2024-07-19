#include <iostream>

extern "C" {
    enum Color {
        Red,
        Green,
        Blue
    };
    
    void print_color(Color color);
}

int main() {
    print_color(Red);
    print_color(Green);
    print_color(Blue);
    return 0;
}
