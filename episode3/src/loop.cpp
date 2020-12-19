#include <iostream>
#include <numeric>
#include <vector>

int main() {
	volatile int j = 0;
	std::vector<int> range(100000000);
	std::iota(range.begin(), range.end(), 0);

	for (auto i : range) {
		// Cannot do j++ because pre/post increment
		// operations on volatile-qualified operations
		// are deprecated in C++20.
		int temp = j;
		temp++;
		j = temp;
	}
}
