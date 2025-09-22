#include <cstdlib>
#include <iostream>
#include <fstream>
#include <vector>

std::vector<std::string> get_name(std::string path){
    std::string line;
    std::ifstream in(path);
    std::vector<std::string> confignamebox;
    if (in.is_open())
        {
            while (std::getline(in, line))
        {
            confignamebox.push_back(line);
        }
        }
    in.close();
    return confignamebox;
    };

std::vector<int> boxwithrandom() {
    std::vector<int> tmp;
    for (int i = 0; i<10; i++) {
        tmp.push_back(1 + rand() % 10);
    }
    return tmp;
}

int main(){
    std::vector<std::string> name = get_name("config");
    for (std::string i: name ){
        std::ofstream fd(i);
        if (fd.is_open()){
            for (int i: boxwithrandom()){
                fd << i << " ";
            }
            fd << std::endl;
        }
        fd.close();
    }
}
