#include <algorithm>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

int main() {
  std::vector<std::vector<char>> crates;
  std::vector<int> move_order(3);
  std::ifstream infile("../input.txt");
  std::string line;
  std::string result = "";
  std::regex regex(R"(\d+)");

  while (std::getline(infile, line)) {
    if (crates.empty()) {
      crates.resize((line.length() + 1) / 4);
    }
    if (line[0] == '[' || line[0] == ' ') {
      for (size_t i = 1, c = 0; i < line.length(); i += 4, ++c) {
        if (isalpha(line[i])) {
          crates[c].push_back(line[i]);
        }
      }
    } else if (line == "") {
      for (size_t i = 0; i < crates.size(); ++i) {
        std::reverse(crates[i].begin(), crates[i].end());
      }
    } else {
      // parse line for ints
      std::smatch match;
      size_t i = 0;
      while (std::regex_search(line, match, regex)) {
        move_order[i] = std::stoi(match.str());
        line = match.suffix();
        ++i;
      }
      // move crates
      while (move_order[0] > 0) {
        crates[move_order[2] - 1].push_back(crates[move_order[1] - 1].back());
        crates[move_order[1] - 1].pop_back();
        --move_order[0];
      }
    }
  }
  infile.close();

  for (auto c : crates) {
    result += c.back();
  }
  std::cout << result << "\n";

  return 0;
}
