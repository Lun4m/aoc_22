#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

class FileSystem {
public:
  FileSystem() : dir(nullptr){};

  void cd(std::string name) {
    if (dir == nullptr) {
      dir = new Directory(name, dir);
    } else if (name == "..") {
      dir = dir->parent;
    } else {
      Directory *new_dir = new Directory(name, dir);
      dir->subdirs.push_back(new_dir);
      dir = new_dir;
    }
  }

  void to_root() {
    while (dir->parent != nullptr) {
      dir = dir->parent;
    }
    // Calculate the correct sizes
    dir->add_subdirs_size();
  }

  void update_size(std::string s) { dir->size += std::stol(s); }
  uint32_t get_size() { return dir->size; }
  uint32_t sum_dirs_under(uint32_t s) { return dir->sum_dirs_under(s); }
  uint32_t smallest_dir_above(uint32_t s) { return dir->smallest_dir_above(s); }

private:
  struct Directory {
    Directory *parent;
    std::vector<Directory *> subdirs;
    std::string name;
    uint32_t size;

    Directory(std::string n, Directory *p)
        : parent(p), subdirs({}), name(n), size(0){};

    void add_subdirs_size() {
      for (auto d : subdirs) {
        d->add_subdirs_size();
        size += d->size;
      }
    }
    uint32_t sum_dirs_under(uint32_t s) {
      uint32_t result = 0;

      for (auto d : subdirs) {
        result += d->sum_dirs_under(s);
      }

      if (size < s) {
        result += size;
      }
      return result;
    }

    uint32_t smallest_dir_above(uint32_t s) {
      uint32_t result = size;
      uint32_t temp;

      for (auto d : subdirs) {
        temp = d->smallest_dir_above(s);
        if (temp >= s && temp < result) {
          result = temp;
        }
      }
      return result;
    }
  };

  Directory *dir; // current directory
};

int main() {
  size_t idx;
  std::ifstream infile("../input.txt");
  std::string line;
  std::vector<std::string> split;
  FileSystem filesystem;

  while (std::getline(infile, line)) {
    // Parse line
    while ((idx = line.find(' ')) != std::string::npos) {
      split.push_back(line.substr(0, idx));
      line.erase(0, idx + 1);
    }
    split.push_back(line);

    // Build filesystem tree
    if (split[0] == "$") {
      if (split[1] == "cd") {
        filesystem.cd(split[2]);
      }
    } else if (split[0] != "dir") {
      filesystem.update_size(split[0]);
    }
    split.clear();
  }
  infile.close();
  filesystem.to_root();

  // Part 1: total sum of dir with sizes less that 100K
  uint32_t result = filesystem.sum_dirs_under(100'000);
  std::cout << "Part 1: " << result << "\n";

  // Part 2: find smallest directory size that frees at least 'req_size'
  uint32_t req_size = filesystem.get_size() - 40'000'000;
  result = filesystem.smallest_dir_above(req_size);
  std::cout << "Part 2: " << result << "\n";
  return 0;
}
