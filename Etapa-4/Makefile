# Grupo L

# Guilherme de Oliveira (00278301)
# Jean Pierre Comerlatto Darricarrere (00182408)

main:
	cargo build --release
	cp target/release/etapa_4 .
	mv etapa_4 etapa4

debug:
	cargo build
	cp target/debug/etapa_4 .
	mv etapa_4 etapa4

clean:
	rm -f etapa4 Cargo.lock
	rm -f -r target

zip:
	tar cvzf etapa4.tgz Makefile Cargo.toml build.rs semanticTests.js src/ 

allTests:
	make debug
	mkdir -p testFiles
	node semanticTests.js ALL

test:
	make debug
	mkdir -p testFiles
	node semanticTests.js