# Turma L

# Guilherme de Oliveira (00278301)
# Jean Pierre Comerlatto Darricarrere (00182408)

main:
	bison -d parser.y
	flex scanner.l

	gcc -c lex.yy.c
	gcc -c parser.tab.c
	gcc -c main.c

	gcc -o etapa2 lex.yy.o parser.tab.o main.o -lfl

clean:
	rm etapa2 etapa2.tgz lex.yy.c lex.yy.o parser.tab.c parser.tab.h parser.tab.o main.o parser.output

zip:
	tar cvzf etapa2.tgz parser.y scanner.l Makefile runTests.sh main.c

run:
	./etapa2

test:
	./runTests.sh

debug:
	bison -d parser.y --debug -v
	flex scanner.l

	gcc -c lex.yy.c
	gcc -c parser.tab.c
	gcc -c main.c

	gcc -o etapa2 lex.yy.o parser.tab.o main.o -lfl
	./etapa2