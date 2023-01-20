#include <iostream>
#include <fstream>
#include <string>
#include <regex>

struct Sensor
{
    int sx;
    int sy;
    int bx;
    int by;

    friend std::ostream& operator<< (std::ostream &o, const Sensor &i)
    {
      return o  << "------  " << '\n' 
                << "sx = " << i.sx << '\n'
                << "sy = " << i.sy << '\n'
                << "bx = " << i.bx << '\n'
                << "by = " << i.by << '\n'
                << "------- " << "\n";
    }
};

std::vector<Sensor> sensors;



void parse_input_file()
{
    std::ifstream myfile;
    myfile.open("short_input.txt");
    std::string myline;

    if (myfile.is_open())
    {
        while (myfile)
        { 
            std::getline(myfile, myline);

            const std::regex pieces_regex(".*=(-?\\d+).*=(-?\\d+).*=(-?\\d+).*=(-?\\d+)");
            std::smatch pieces_match;

            if (std::regex_match(myline, pieces_match, pieces_regex)) {
                Sensor s;
                s.sx = stoi(pieces_match[1].str());
                s.sy = stoi(pieces_match[2].str());
                s.bx = stoi(pieces_match[3].str());
                s.by = stoi(pieces_match[4].str());
                sensors.push_back(s);
            }   
        }
    }
    else
    {
        std::cout << "Couldn't open file\n";
    }
}

int main()
{
    parse_input_file();

    

    return 0;
}
