import graph_tool as gt
import graph_tool.collection


def main():

    # PGP strong
    g = gt.collection.data["pgp-strong-2009"]
    g.save("../test_data/pgp_graph_with_properties.gt")
    print(g.vp["valid"][0])

    g.properties.clear()
    g.save("../test_data/pgp_graph_without_properties.gt")


    # Very simple graph

    g = gt.Graph(directed=True)
    u = g.add_vertex()
    v = g.add_vertex()
    w = g.add_vertex()
    g.add_edge(u, v)
    g.add_edge(u, w)
    g.save("../test_data/vsg_graph.gt")


if __name__ == "__main__":
    main()
    

