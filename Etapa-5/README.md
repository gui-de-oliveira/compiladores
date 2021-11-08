<!-- Grupo L -->
<!-- Guilherme de Oliveira (00278301) -->
<!-- Jean Pierre Comerlatto Darricarrere (00182408) -->

# Requisitos para compilação do código

Nosso código foi escrito em Rust por isso é necessário instalar sua toolchain (rustup) para compilá-lo.

Para realizar a instalação dele no Ubuntu, basta usar o seguinte comando no terminal :

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Para informações de instalação em outros sistemas, basta acessar o portal: https://www.rust-lang.org/tools/install

Além da toolchain, é necessário ter conexão à internet na primeira compilação para que o package manager cargo possa gerenciar o download das bibliotecas.

Esse processo está previso no Makefile, basta utilizar o comando `make`.

Utilizamos nesta etapa para análise léxica e parsing as bibliotecas grmtools, substituindo o Flex e o Bison das etapas anteriores.

Para informações sobre ela, acessar https://softdevteam.github.io/grmtools/master/book/index.html

Para mais informações sobre preparar o ambiente Rust, recomendamos o seguinte site: https://www.rust-lang.org/learn/get-started
