/*

g++ -o main main.cpp -L. -ldescribable_color -ldl
*/
#include <iostream>
#include <cstring>

extern "C" {
    enum Color {
        Red,
        Green,
        Blue
    };

    void print_color(Color color);
    void print_complex_color_with_description(int tag, const char* custom_data);
}

int main() {
    print_complex_color_with_description(0, nullptr); // Red
    print_complex_color_with_description(1, nullptr); // Green
    print_complex_color_with_description(2, nullptr); // Blue
    print_complex_color_with_description(3, "Purple"); // Custom

    return 0;
}
