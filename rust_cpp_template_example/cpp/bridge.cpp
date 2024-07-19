// cpp/bridge.cpp
#include "Operations.h"
#include "rust/cxx.h"

extern "C" {
    Operations<int>* new_operations_int(int value) {
        return new Operations<int>(value);
    }

    int operations_int_add(Operations<int>* self, int other) {
        return self->add(other);
    }

    int operations_int_subtract(Operations<int>* self, int other) {
        return self->subtract(other);
    }

    void delete_operations_int(Operations<int>* self) {
        delete self;
    }
}
