#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <vector>

struct directory {
  std::string name;
  int parent;
  std::vector<int> subdirs;
  int depth;
  long size;

  directory() {
    name = "";
    parent = -1;
    subdirs = {};
    depth = 0;
    size = 0;
  }
};

int main() {
  long result = 0;
  std::ifstream infile("../input.txt");
  std::string line;
  size_t idx;
  std::vector<std::string> split;

  directory dir;
  std::vector<directory> filesystem;
  int depth = -1, max_depth = 0;
  int parent_dir = -1, index = -1;

  while (std::getline(infile, line)) {
    // Split line
    split.clear();
    while ((idx = line.find(' ')) != std::string::npos) {
      split.push_back(line.substr(0, idx));
      line.erase(0, idx + 1);
    }
    split.push_back(line);

    if (split[0] == "$") {
      if (split[1] == "cd") {
        if (split[2] == "..") {
          // Reset parent index when going up
          --depth;
          parent_dir = filesystem[parent_dir].parent;
        } else {
          // Update directory
          ++depth;
          ++index;
          dir.name = split[2];
          dir.parent = parent_dir;
          dir.depth = depth;
          dir.size = 0;
          dir.subdirs = {};
          filesystem.push_back(dir);
          // Link subdir to parent
          if (dir.parent >= 0) {
            filesystem[parent_dir].subdirs.push_back(index);
          }
          // Update directory depth
          if (depth > max_depth) {
            max_depth = depth;
          }
        }
      } else if (split[1] == "ls") {
        parent_dir = index;
      }
    } else if (split[0] == "dir") {
      continue;
    } else {
      filesystem[index].size += std::stol(split[0]);
    }
  }
  infile.close();

  // Update dir sizes starting from deeper subdirs
  while (max_depth >= 0) {
    for (size_t i = 0; i < filesystem.size(); ++i) {
      if (filesystem[i].depth == max_depth) {
        for (size_t j = 0; j < filesystem[i].subdirs.size(); ++j) {
          filesystem[i].size += filesystem[filesystem[i].subdirs[j]].size;
        }
        // Part 1: total sum of dir with sizes less that 100K
        // if (filesystem[i].size <= 100000) {
        //   result += filesystem[i].size;
        // }
      }
    }
    --max_depth;
  }

  // Part 2: find smallest directory size that frees at least 'req_size'
  long req_size = filesystem[0].size - 40000000;
  result = filesystem[0].size;
  for (size_t i = 1; i < filesystem.size(); ++i) {
    if (filesystem[i].size >= req_size) {
      if (result > filesystem[i].size) {
        result = filesystem[i].size;
      }
    }
  }
  std::cout << result << "\n";
  return 0;
}
