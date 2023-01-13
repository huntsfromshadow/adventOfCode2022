#include <iostream>
#include <vector>
#include <iterator>
#include <algorithm>
#include "graph.h"

void Graph::addNode(Node* node) {
    auto ptr = std::make_shared<Node>(node);
    nodes.push_back(ptr);
}

//void Graph::addEdge(int from, int to) {
//  std::cout << from << " -> " << to << "\n";

  //Node* n = getNode(from);
  //std::cout << n << "\n";
//}

std::shared_ptr<Node> Graph::getNode(int id) {

  auto it = std::find_if(nodes.begin(), nodes.end(), [&](std::shared_ptr<Node> const& p) {
    return p->getId() == id; // assumes MyType has operator==
  });
  if (it != nodes.end()) {
    std::cout << *it << "\n";
  }

  return nullptr;

}


Node::Node(int n) {
  id = n;
}

int Node::getId()
{
  return id;
}

std::ostream &operator<<(std::ostream &os, const Node& p)
{
  return os << p.id;
}


