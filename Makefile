CFLAGS=-std=c11 -g -static

small-cc: small-cc.c

test: small-cc
	./test.sh
clean:
	rm -f small-cc *.o *~ tmp*

.PHONY: test clean
