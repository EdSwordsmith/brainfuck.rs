#!/usr/bin/python3
tape = {}
ptr = 0

current_input = ''

def inc_value(value):
    if ptr not in tape:
        tape[ptr] = 0
    tape[ptr] += value

def read_stdin():
    global current_input
    if len(current_input) == 0:
        current_input = input()

    if len(current_input) > 0:
        tape[ptr] = ord(current_input[0])
        current_input = current_input[1:]

    else:
        tape[ptr] = 0

def should_loop():
    if ptr not in tape:
        return False
    return tape[ptr] != 0

inc_value(8)
while should_loop():
    ptr += 1
    inc_value(4)
    while should_loop():
        ptr += 1
        inc_value(2)
        ptr += 1
        inc_value(3)
        ptr += 1
        inc_value(3)
        ptr += 1
        inc_value(1)
        ptr += -4
        inc_value(-1)
    ptr += 1
    inc_value(1)
    ptr += 1
    inc_value(1)
    ptr += 1
    inc_value(-1)
    ptr += 2
    inc_value(1)
    while should_loop():
        ptr += -1
    ptr += -1
    inc_value(-1)
ptr += 2
print(chr(tape[ptr]), end='')
ptr += 1
inc_value(-3)
print(chr(tape[ptr]), end='')
inc_value(7)
print(chr(tape[ptr]), end='')
print(chr(tape[ptr]), end='')
inc_value(3)
print(chr(tape[ptr]), end='')
ptr += 2
print(chr(tape[ptr]), end='')
ptr += -1
inc_value(-1)
print(chr(tape[ptr]), end='')
ptr += -1
print(chr(tape[ptr]), end='')
inc_value(3)
print(chr(tape[ptr]), end='')
inc_value(-6)
print(chr(tape[ptr]), end='')
inc_value(-8)
print(chr(tape[ptr]), end='')
ptr += 2
inc_value(1)
print(chr(tape[ptr]), end='')
ptr += 1
inc_value(2)
print(chr(tape[ptr]), end='')
