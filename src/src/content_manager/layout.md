# layout

```markdown
+---------------------+
| Page Header         |  <- Metadata (e.g., page ID, free space pointer)
+---------------------+
| Row 1               |  <- Serialized row data
+---------------------+
| Row 2               |
+---------------------+
| ...                 |
+---------------------+
| Free Space          |  <- Empty area for new rows
+---------------------+
| Offsets Table       |  <- Array pointing to rows
| [Row 1 Offset]      |  <- Points to Row 1's starting location
| [Row 2 Offset]      |  <- Points to Row 2's starting location
+---------------------+


+---------------------+
| Page Id             | : u8 -> 1b
+---------------------+
| table name          | : [u8; 64] -> 64b
+---------------------+
| number of rows      | : u64 -> 8b
+---------------------+
| free space ptr      | : u64 -> 8b
+---------------------+

```

The data encoding format with binary approach useful when needing some piece of data which make's it a O(1) operation of just accessing the right index

