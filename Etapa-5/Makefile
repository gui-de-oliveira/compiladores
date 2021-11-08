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
	tar cvzf etapa5.tgz Makefile Cargo.toml build.rs runSynthesisTests.ts package.json tsconfig.json ilocsim.py src/ README.md

test:
	make debug
	npm testBuild
	node runSynthesisTests.js
	rm -f -r runSynthesisTests.js

testZip:
	rm -rf test
	make zip
	mkdir test
	mv etapa5.tgz test/etapa5.tgz
	cd test; \
	tar -xvzf etapa5.tgz; \
	npm install; \
	make; \
	npm run buildTest; \
	node runSynthesisTests.js; \
	rm -f -r runSynthesisTests.js;
	rm -rf test;
