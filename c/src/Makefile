CC = mpicc

CFLAGS = -Wall -O3

SRCS = $(wildcard *.c)

PROGS = $(patsubst %.c,%,$(SRCS))

all: $(PROGS)

clean:
	rm -f $(PROGS)

%: %.c
	$(CC) $(CFLAGS) -o $@ $<
