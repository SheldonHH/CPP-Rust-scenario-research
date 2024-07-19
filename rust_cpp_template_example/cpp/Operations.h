// cpp/Operations.h
#ifndef OPERATIONS_H
#define OPERATIONS_H

template<typename T>
class Operations {
public:
    Operations(T value);
    T add(T other);
    T subtract(T other);

private:
    T value_;
};

#endif // OPERATIONS_H
