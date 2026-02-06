default:entry.svg
entry.svg:entry.typ
	typst c $^ -f svg $@
solve:solve.cpp
	g++ $^ -o $@ -std=c++20
all:entry.svg solve
clean:
	rm -f entry.svg solve
