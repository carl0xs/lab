#include<iostream>

using namespace std;

struct List {
	int val;
	List* next;

	void print() const {
		cout << "val: " << val << " next: " << next;
	}
};

int main() {
	List linked_list = { 1, &{2, nullptr } };

	linked_list.print();

	return 0;
}
