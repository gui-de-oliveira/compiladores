# Grupo L

# Guilherme de Oliveira (00278301)
# Jean Pierre Comerlatto Darricarrere (00182408)

main:
	bison -d parser.y
	flex scanner.l

	gcc -c lex.yy.c
	gcc -c parser.tab.c
	gcc -c lexical_structures.c
	gcc -c main.c

	gcc -o etapa3 lex.yy.o parser.tab.o lexical_structures.o main.o -lfl

clean:
	rm -f etapa3 etapa3.tgz lex.yy.c lex.yy.o parser.tab.c parser.tab.h parser.tab.o main.o lexical_structures.o parser.output

zip:
	tar cvzf etapa3.tgz parser.y scanner.l Makefile lexical_structures.h lexical_structures.c main.c

test:
	make
	node aslTests.js
	node runSpecialTests.js
	node runMemTests.js
	make clean
