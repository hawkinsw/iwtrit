all: episode3 loop

episode3:
	cargo build --release
loop: src/loop.cpp
	g++ -O3 -funroll-loops -o loop src/loop.cpp
clean:
	cargo clean
	rm -f core a.out loop
