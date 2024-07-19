// cpp/Operations.cpp
#include "Operations.h"

template<typename T>
Operations<T>::Operations(T value) : value_(value) {}

template<typename T>
T Operations<T>::add(T other) {
    return value_ + other;
}

template<typename T>
T Operations<T>::subtract(T other) {
    return value_ - other;
}

// 实例化模板
template class Operations<int>;
