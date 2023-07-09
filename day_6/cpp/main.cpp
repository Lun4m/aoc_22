#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
#include <vector>

int main() {
  size_t len = 4; // or 14 for part 2
  std::ifstream infile("../input.txt");
  std::string line;
  std::getline(infile, line);
  for (size_t i = 0; i < line.length(); ++i) {
    std::string subline = line.substr(i, len);
    std::set<char> set(subline.begin(), subline.end());
    if (set.size() == len) {
      std::cout << i + len << std::endl;
      return 0;
    }
  }
  return 0;
}
