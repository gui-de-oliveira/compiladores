# Grupo L

# Guilherme de Oliveira (00278301)
# Jean Pierre Comerlatto Darricarrere (00182408)

# !!!WIP!!!

main:
	cargo build
	cp target/debug/etapa_4 .
	mv etapa_4 etapa4

clean:
	rm -f etapa4 Cargo.lock -lfl
	rm -r src/target

zip:
	tar cvzf etapa4.tgz Makefile Cargo.toml build.rs src/parser.y src/scanner.l src/lexical_structures.rs src/lexical_structures.rs src/ast_node.rs src/main.rs

test:
	make
	node aslTests.js
	node runSpecialTests.js
	python runSyntaxTests.py
