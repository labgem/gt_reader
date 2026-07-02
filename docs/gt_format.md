# The `gt` file format

The `gt` file format is a simple binary format designed to store
graph-tool {class}`~graph_tool.Graph` instances in a compact and fast
manner, including all types of property maps supported by the
library. It serves as an alternative to the text-based [graphml](http://graphml.graphdrawing.org/) format for very large graphs where
I/O may become very time and space consuming.

Here we describe the encoding in detail, using as an example the
`lesmis` network from the {mod}`graph_tool.collection` module.

The header begins with the magic string `⛾ gt` in utf-8 encoding,
totaling 6 bytes, followed by the version number (currently `0x01`) in
a single byte, and a Boolean (also a single byte) determining the
[endianness](https://en.wikipedia.org/wiki/Endianness) (`0x00`:
little-endian, `0x01`: big-endian):

```none
00000000  e2 9b be 20 67 74 01 00                           |... gt..|
00000008
```

This is followed by a comment string. Strings are stored by a length (8
bytes, `uint64_t`) and the corresponding sequence of bytes, not
null-terminated. The comment may be empty, but graph-tool outputs a
human readable summary of the file, which can be inspected with tools
like `hexdump`:

```none
00000000  e2 9b be 20 67 74 01 00  dd 00 00 00 00 00 00 00  |... gt..........|
00000010  67 72 61 70 68 2d 74 6f  6f 6c 20 62 69 6e 61 72  |graph-tool binar|
00000020  79 20 66 69 6c 65 20 28  68 74 74 70 3a 3a 2f 2f  |y file (http:://|
00000030  67 72 61 70 68 2d 74 6f  6f 6c 2e 73 6b 65 77 65  |graph-tool.skewe|
00000040  64 2e 64 65 29 20 67 65  6e 65 72 61 74 65 64 20  |d.de) generated |
00000050  62 79 20 76 65 72 73 69  6f 6e 20 32 2e 32 2e 33  |by version 2.2.3|
00000060  32 64 65 76 20 28 63 6f  6d 6d 69 74 20 64 34 66  |2dev (commit d4f|
00000070  31 66 31 62 66 2c 20 4d  6f 6e 20 41 75 67 20 31  |1f1bf, Mon Aug 1|
00000080  31 20 31 36 3a 32 36 3a  35 34 20 32 30 31 34 20  |1 16:26:54 2014 |
00000090  2b 30 32 30 30 29 20 73  74 61 74 73 3a 20 37 37  |+0200) stats: 77|
000000a0  20 76 65 72 74 69 63 65  73 2c 20 32 35 34 20 65  | vertices, 254 e|
000000b0  64 67 65 73 2c 20 75 6e  64 69 72 65 63 74 65 64  |dges, undirected|
000000c0  2c 20 32 20 67 72 61 70  68 20 70 72 6f 70 73 2c  |, 2 graph props,|
000000d0  20 32 20 76 65 72 74 65  78 20 70 72 6f 70 73 2c  | 2 vertex props,|
000000e0  20 31 20 65 64 67 65 20  70 72 6f 70 73           | 1 edge props|
000000ed
```

The adjacency list now follows, beginning with a Boolean byte specifying
whether or not the graph is directed (`0x00`: undirected, `0x01`:
directed), and 8 bytes (`uint64_t`) containing the number of nodes,
`N`. It is followed by the list of out-neighbors of all `N` nodes
in sequence. The sequence itself determines implicitly the index of the
nodes, in the range from `0` to `N-1`. The list of out-neighbors of
a given node is composed of a length (8 bytes, `uint64_t`) and a
sequence of node indices with this length. The number of bytes `d`
used to encode the node indices in this list is determined by the value
of `N`, and will be the smallest value of the set `{1, 2, 4, 8}`
(i.e. `{uint8_t, uint16_t, uint32_t, uint64_t}`, respectively) which
is sufficient to accommodate all `N` nodes. For undirected graphs,
here it is important that each edge appears only once, i.e. if node
`u` appears in the list of neighbors of `v`, then `v` **should
not** appear in the list of `u` again, otherwise it will be considered
as a different (parallel) edge. In this way, the total number of bytes
used for the adjacency is `1 + 8 + N * 8 + E * d` with `E` being the
number of edges:

```none
00000000  e2 9b be 20 67 74 01 00  dd 00 00 00 00 00 00 00  |... gt..........|
00000010  67 72 61 70 68 2d 74 6f  6f 6c 20 62 69 6e 61 72  |graph-tool binar|
00000020  79 20 66 69 6c 65 20 28  68 74 74 70 3a 3a 2f 2f  |y file (http:://|
00000030  67 72 61 70 68 2d 74 6f  6f 6c 2e 73 6b 65 77 65  |graph-tool.skewe|
00000040  64 2e 64 65 29 20 67 65  6e 65 72 61 74 65 64 20  |d.de) generated |
00000050  62 79 20 76 65 72 73 69  6f 6e 20 32 2e 32 2e 33  |by version 2.2.3|
00000060  32 64 65 76 20 28 63 6f  6d 6d 69 74 20 64 34 66  |2dev (commit d4f|
00000070  31 66 31 62 66 2c 20 4d  6f 6e 20 41 75 67 20 31  |1f1bf, Mon Aug 1|
00000080  31 20 31 36 3a 32 36 3a  35 34 20 32 30 31 34 20  |1 16:26:54 2014 |
00000090  2b 30 32 30 30 29 20 73  74 61 74 73 3a 20 37 37  |+0200) stats: 77|
000000a0  20 76 65 72 74 69 63 65  73 2c 20 32 35 34 20 65  | vertices, 254 e|
000000b0  64 67 65 73 2c 20 75 6e  64 69 72 65 63 74 65 64  |dges, undirected|
000000c0  2c 20 32 20 67 72 61 70  68 20 70 72 6f 70 73 2c  |, 2 graph props,|
000000d0  20 32 20 76 65 72 74 65  78 20 70 72 6f 70 73 2c  | 2 vertex props,|
000000e0  20 31 20 65 64 67 65 20  70 72 6f 70 73 00 4d 00  | 1 edge props.M.|
000000f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 01 00  |................|
00000100  00 00 00 00 00 00 00 01  00 00 00 00 00 00 00 00  |................|
[...]
00000440  00 00 00 00 00 00 45 44  19 30 29 46 47 07 00 00  |......ED.0)FG...|
00000450  00 00 00 00 00 40 41 42  3f 3e 30 3a              |.....@AB?>0:|
0000045c
```

The adjacency is followed by a list of property maps. The list begins
with a total number of property maps (8 bytes, `uint64_t`), and then
the individual records. Each property map begins with a key type (1
byte, `uint8_t`) specifying whether it is a graph (`0x00`), a vertex
(`0x01`) or an edge (`0x02`) property map, followed by a string (8
byte length + length bytes) containing the name of the property
map. This is then followed by a byte (`uint8_t`) specifying the value
type index, from the following table:

```{eval-rst}
.. tabularcolumns:: |l|l|
```

```{eval-rst}
.. table::

    ========================     ===================  ========
     Type name                   Bytes                Index
    ========================     ===================  ========
    ``bool``                     ``1``                ``0x00``
    ``int16_t``                  ``2``                ``0x01``
    ``int32_t``                  ``4``                ``0x02``
    ``int64_t``                  ``8``                ``0x03``
    ``double``                   ``8``                ``0x04``
    ``long double``              ``16``               ``0x05``
    ``string``                   ``8 + length``       ``0x06``
    ``vector<bool>``             ``8 + length``       ``0x07``
    ``vector<int16_t>``          ``8 + 2 * length``   ``0x08``
    ``vector<int32_t>``          ``8 + 4 * length``   ``0x09``
    ``vector<int64_t>``          ``8 + 8 * length``   ``0x0a``
    ``vector<double>``           ``8 + 8 * length``   ``0x0b``
    ``vector<long double>``      ``8 + 16 * length``  ``0x0c``
    ``vector<string>``           ``8 + <variable>``   ``0x0d``
    ``python::object``           ``8 + length``       ``0x0e``
    ========================     ===================  ========
```

The values of the property map follow in the order of the vertex indices
(for vertex properties) or in the same order in which the edges appear
in the preceding adjacency list (for edge properties). For graph
properties only one value follows. Strings and vectors are encoded with
a length prefix of 8 bytes (`uint64_t`) followed by a sequence of that
size with the appropriate element size. The elements of
`vector<string>` are encoded as pairs of (8 byte length, bytes) as
usual. Values of type `python::object` are encoded just as strings,
with the string content encoded or decoded via {mod}`pickle`.
