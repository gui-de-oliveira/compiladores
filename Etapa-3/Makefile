# Grupo L

# Guilherme de Oliveira (00278301)
# Jean Pierre Comerlatto Darricarrere (00182408)

main:
	bison -d parser.y
	flex scanner.l

	gcc -c lex.yy.c
	gcc -c parser.tab.c
	gcc -c main.c

	gcc -o etapa3 lex.yy.o parser.tab.o main.o -lfl

clean:
	rm -f etapa3 etapa3.tgz lex.yy.c lex.yy.o parser.tab.c parser.tab.h parser.tab.o main.o parser.output
	rm *.temp

zip:
	tar cvzf etapa3.tgz parser.y scanner.l Makefile runTests.sh

run:
	./etapa3

test:
	node aslTests.js

debug:
	bison -d parser.y --debug -v
	flex scanner.l

	gcc -c lex.yy.c
	gcc -c parser.tab.c
	gcc -c main.c

	gcc -o etapa3 lex.yy.o parser.tab.o main.o -lfl
	./etapa3

log:
	make
	./etapa3 < .temp