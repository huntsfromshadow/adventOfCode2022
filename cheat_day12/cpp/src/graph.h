#pragma once
#include <vector>
#include <memory>

class Node
{
private:
  int id;
  std::vector<Node*> edges;
  friend std::ostream &operator<<(std::ostream &os, const Node& p);

public:
  Node(int id);
  int getId();

  /*
   bool operator==(Paragraph const& rhs) const
        {
            return m_para == rhs.m_para;
        }
        bool operator!=(Paragraph const& rhs) const
        {
            // Define != operator in terms of the == operator
            return !(this->operator==(rhs));
        }
        bool operator<(Paragraph const& rhs) const
        {
            return  m_para < rhs.m_para;
        }*/
};


class Graph
{
  private:
    std::vector <std::shared_ptr<Node>> nodes;

  public:
    Graph() = default;
    //Node *createNode(int id);
    void addNode(Node* node);
    std::shared_ptr<Node> getNode(int id);
    void addEdge(int from, int to);
};

