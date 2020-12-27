#include <iostream>
#include <numeric>
#include <vector>

int main() {
	volatile int j = 0;
	// This is not the modern way of writing a loop in C++. Done
	// this way for expository purposes only.
	for (auto i = 0; i<100000000; i++) {
		// Cannot do j++ because pre/post increment
		// operations on volatile-qualified operations
		// are deprecated in C++20.
		int temp = j;
		temp++;
		j = temp;
	}
}
