default:chain.svg
chain.svg:chain.typ
	typst c $^ -f svg $@
solve:solve.cpp
	g++ $^ -o $@ -std=c++20
all:chain.svg solve
clean:
	rm -f chain.svg solve
