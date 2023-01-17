#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <iterator>
#include <stdexcept>

struct Point
{
  int x;
  int y;
};

std::vector<Point> points;
int floor_y = 0;

void print_points()
{
  for (const Point &p : points)
  {
    std::cout << "( " << p.x << ", " << p.y << " ) -- ";
  }
  std::cout << "\n";
}

bool point_vec_contains(Point a)
{
  for (const Point &p : points)
  {
    if (a.x == p.x && a.y == p.y)
    {
      return true;
    }
  }
  return false;
}

bool compare_point(Point p1, Point p2)
{
  if (p1.x == p2.x && p1.y == p2.y)
  {
    return true;
  }
  else
  {
    return false;
  }
}

bool is_point_zero(Point p1)
{
  Point z;
  z.x = 0;
  z.y = 0;
  return compare_point(p1, z);
}

void build_line_2(Point a, Point b)
{

  std::cout << a.x << "," << a.y << "->" << b.x << "," << b.y << "\n";

  if (a.x == b.x)
  {
    int s = 0;
    int e = 0;
    if (a.y < b.y)
    {
      s = a.y;
      e = b.y;
    }
    else
    {
      s = b.y;
      e = a.y;
    }

    for (int i = s; i < (e + 1); i++)
    {
      Point p;
      p.x = a.x;
      p.y = i;

      if (!point_vec_contains(p))
      {
        if( p.y > floor_y )
        {
          floor_y = p.y;
        }
        points.push_back(p);
      }
    }
  }
  else
  {
    int s = 0;
    int e = 0;
    if (a.x < b.x)
    {
      s = a.x;
      e = b.x;
    }
    else
    {
      s = b.x;
      e = a.x;
    }

    for (int i = s; i < (e + 1); i++)
    {
      Point p;
      p.x = i;
      p.y = a.y;

      if (!point_vec_contains(p))
      {
        if( p.y > floor_y )
        {
          floor_y = p.y;
        }
        points.push_back(p);
      }
    }
  }
}

void build_line(std::vector<Point> v)
{
  Point a;
  a.x = 0;
  a.y = 0;

  for (const Point &p : v)
  {
    if (is_point_zero(a))
    {
      a.x = p.x;
      a.y = p.y;
    }
    else
    {
      build_line_2(a, p);

      a.x = p.x;
      a.y = p.y;
    }
  }
  std::cout << "\n";
}

void parse_file()
{
  char ch;
  std::string wrk = "";
  int cord1, cord2;

  std::vector<Point> v;

  //std::fstream fin("short-input.txt", std::fstream::in);
  std::fstream fin("input.txt", std::fstream::in);
  while (fin >> std::noskipws >> ch)
  {

    if (ch != ' ' && ch != '-')
    {
      if (ch == ',')
      {
        cord1 = std::stoi(wrk);
        wrk = "";
      }
      else if (ch == '>' || ch == '\n')
      {
        cord2 = std::stoi(wrk);
        Point p;
        p.x = cord1;
        p.y = cord2;
        v.push_back(p);
        wrk = "";

        if (ch == '\n')
        {
          // We need to handle the build from the vec
          build_line(v);
          v.clear();
        }
      }
      else
      {
        wrk = wrk + ch;
      }
    }
  }

  floor_y = floor_y + 2;
  //floor_y = floor_y + 1;
}

bool can_move_down(Point p)
{
  Point z;
  z = p;
  z.y = z.y + 1;

  // First check if it is hitting floor
  if( z.y == floor_y ) 
  {
    return false;
  }
  else 
  {
    return !point_vec_contains(z);
  }
}

bool can_move_down_left(Point p)
{
  Point z;
  z = p;
  z.y = z.y + 1;
  z.x = z.x - 1;

  // First check if it is hitting floor
  if( z.y == floor_y ) 
  {
    return false;
  }
  else 
  {
    return !point_vec_contains(z);
  }
}

bool can_move_down_right(Point p)
{
  Point z;
  z = p;
  z.y = z.y + 1;
  z.x = z.x + 1;

  // First check if it is hitting floor
  if( z.y == floor_y ) 
  {
    return false;
  }
  else 
  {
    return !point_vec_contains(z);
  }
}

void run_simulation()
{
  bool sand_in_motion = false;
  bool sim_active = true;
  int sand_count = 0;
  Point sand_cord;

  while( sim_active == true )
  {
    if(sand_in_motion == false)
    {
      std::cout << "Spawning Sand\n";
      sand_cord.x = 500;
      sand_cord.y = 0;
      sand_in_motion = true;
      sand_count++;
    }

    
    if( can_move_down(sand_cord) == true )
    {
      sand_cord.y = sand_cord.y + 1;
      std::cout << "Moved sand down (" << sand_cord.x << "," <<
        sand_cord.y << ")\n";
    }
    else
    {
      std::cout << "Can't move down\n";
      if( can_move_down_left(sand_cord) == true )
      {
        sand_cord.y = sand_cord.y + 1;
        sand_cord.x = sand_cord.x - 1;
        std::cout << "Moved sand down left ("
          << sand_cord.x << "," << sand_cord.y << ")\n";
      }
      else
      {
        if( can_move_down_right(sand_cord) == true)
        {
          sand_cord.y = sand_cord.y + 1;
          sand_cord.x = sand_cord.x + 1;
          std::cout << "Moved sand down right (" 
            << sand_cord.x << "," << sand_cord.y << ")\n";
        }
        else {
          std::cout << "Blocked - Stopping at (" << sand_cord.x << "," << sand_cord.y << ")\n";

          if( sand_cord.x == 500 && sand_cord.y == 0)
          {
            std::cout << "Sand At 500,0. -- " << sand_count << "\n";
            return;
          } 
          else {
            points.push_back(sand_cord);
            sand_in_motion = false;
          }
        }
      }
    }
  }
}

int main()
{
  parse_file();
  run_simulation();

}