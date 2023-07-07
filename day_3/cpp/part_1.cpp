#include <fstream>
#include <iostream>
#include <map>
#include <string>

// Use function instead of goto to exit the loop
char get_repeated_item(std::string line) {
  int compartment_size = line.length() / 2;

  for (char &i : line.substr(0, compartment_size)) {
    for (char &j : line.substr(compartment_size)) {
      if (i == j) {
        return i;
      }
    }
  }
  return '0';
}

int main(int argc, char *argv[]) {
  int priority_sum = 0;
  std::map<char, int> item_priority;
  std::string alphabet = "abcdefghijklmnopqrstuvwxyz";
  for (int i = 0; i < 26; ++i) {
    item_priority[alphabet[i]] = i + 1;
    item_priority[toupper(alphabet[i])] = i + 27;
  }

  std::ifstream infile("../input.txt");
  std::string line;
  while (std::getline(infile, line)) {
    priority_sum += item_priority[get_repeated_item(line)];
  }
  infile.close();
  std::cout << "The total priority sum is " << priority_sum << "\n";
  return 0;
}
