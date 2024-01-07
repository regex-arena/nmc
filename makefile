CC = gcc
MAIN = src/main.c
BIN = bin/nmc
CFLAGS = -Wall
LFLAGS = 

build:
	$(CC) $(CFLAGS) $(LFLAGS) $(MAIN) -o $(BIN)

b: build
