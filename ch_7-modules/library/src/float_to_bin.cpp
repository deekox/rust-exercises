#include <iostream>
#include <limits>

void print_bin(void* mem, size_t len) {
  unsigned char i = 3;
  do {
    unsigned char c = reinterpret_cast<unsigned char*>(mem)[i];
    for (int s = 7; s >= 0; --s) {
      if (c & 1 << s)
	std::cout << "1";
      else
	std::cout << "0";
    }
    std::cout << " ";
  } while (--i != std::numeric_limits<unsigned char>::max());
}


int main() {
  std::cout << __cplusplus << std::endl;
  
  int i = 123;
  print_bin(&i, 4);
  std::cout << i << "\n";

  float f = 123;
  print_bin(&f, 4);
  std::cout << f << "\n";

  float f2 = -0.123;
  print_bin(&f2, 4);
  std::cout << f2 << "\n";
}
