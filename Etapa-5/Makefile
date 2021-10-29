# Grupo L

# Guilherme de Oliveira (00278301)
# Jean Pierre Comerlatto Darricarrere (00182408)

main:
	cargo build --release
	cp target/release/etapa_5 .
	mv etapa_5 etapa5

debug:
	cargo build
	cp target/debug/etapa_5 .
	mv etapa_5 etapa5

clean:
	rm -f etapa5 Cargo.lock
	rm -f -r target

zip:
	tar cvzf etapa5.tgz Makefile Cargo.toml build.rs semanticTests.js src/ README.md

allTests:
	make debug
	mkdir -p testFiles
	node semanticTests.js ALL

test:
	make debug
	python3 runSyntaxTests.py
