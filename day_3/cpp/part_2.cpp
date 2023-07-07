#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

// Use function instead of goto to exit the loop
char get_group_type(std::vector<std::string> group) {
  // could optimize by sorting loop size?
  for (char &i : group[0]) {
    for (char &j : group[1]) {
      for (char &k : group[2]) {
        if (i == j && i == k) {
          return i;
        }
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
  std::vector<std::string> group(3);
  int group_counter = 0;

  std::ifstream infile("../input.txt");
  std::string line;
  while (std::getline(infile, line)) {
    group[group_counter] = line;
    ++group_counter;
    if (group_counter == 3) {
      priority_sum += item_priority[get_group_type(group)];
      group_counter = 0;
    }
  }
  infile.close();
  std::cout << "The total priority sum is " << priority_sum << "\n";
  return 0;
}
