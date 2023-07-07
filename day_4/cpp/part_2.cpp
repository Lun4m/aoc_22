#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main() {
  int result = 0;
  std::vector<int> ranges(4);
  std::vector<char> delimiter = {'-', ',', '-'};

  std::ifstream infile("../input.txt");
  std::string line;
  while (std::getline(infile, line)) {
    for (int i = 0; i < 3; ++i) {
      int delimiter_pos = line.find(delimiter[i]);
      ranges[i] = stoi(line.substr(0, delimiter_pos));
      line.erase(0, line.find(delimiter[i]) + 1);
    }
    ranges[3] = stoi(line);

    if (ranges[0] <= ranges[3] && ranges[1] >= ranges[2]) {
      ++result;
      continue;
    }
    if (ranges[0] >= ranges[3] && ranges[1] <= ranges[2]) {
      ++result;
    }
  }
  infile.close();

  std::cout << result << "\n";
  return 0;
}
