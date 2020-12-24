all: episode3 loop

episode3:
	cargo build --release
loop: src/loop.cpp
	g++ -O3 -funroll-loops -o loop src/loop.cpp

obj: episode3 loop
	objdump -d --demangle target/release/episode3 > episode3.obj
	objdump -d --demangle loop > loop.obj

clean:
	cargo clean
	rm -f core a.out loop *.obj
