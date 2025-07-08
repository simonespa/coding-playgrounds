#include <iostream>

// Recursive Fibonacci function
int fibonacci(int n) {
  if (n <= 1)
    return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
  int n;
  std::cout << "Enter an integer number to compute its Fibonacci value: ";
  std::cin >> n;
  int result = fibonacci(n);
  std::cout << "Fibonacci(" << n << ") = " << result << std::endl;
  return 0;
}
