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

