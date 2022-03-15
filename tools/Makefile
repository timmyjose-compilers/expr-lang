CC := gcc
CFLAGS := -Wall -O2

PEG := peg
RM := rm -f

EXE := expr

.PHONY: all clean

all: $(EXE)

$(EXE): 
	@$(PEG) -o parser.c expr.peg 2>&1
	@$(CC) $(CFLAGS) -c -o parser.o parser.c > /dev/null 2>&1
	@$(CC) $(CFLAGS) -c -o main.o main.c > /dev/null 2>&1
	@$(CC) $(CFLAGS) -o $@ parser.o main.o 2> /dev/null >&1

clean:
	@$(RM) $(EXE) parser.c *.o 