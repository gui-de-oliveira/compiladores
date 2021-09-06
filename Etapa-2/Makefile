main:
	bison -d parser.y
	flex scanner.l

	gcc -c lex.yy.c
	gcc -c parser.tab.c
	gcc -c main.c

	gcc -o etapa2 lex.yy.o parser.tab.o main.o -lfl

clean:
	rm etapa2 etapa2.tgz lex.yy.c lex.yy.o parser.tab.c parser.tab.h parser.tab.o main.o

zip:
	tar cvzf etapa2.tgz parser.y scanner.l Makefile

run:
	./etapa2

test:
	./runTests.sh