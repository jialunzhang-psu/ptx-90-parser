### Description

Query an attribute of a surface. Operand a is a .surfref variable or a .u64 register.

### Syntax

```
suq.query.b32   d, [a];

.query = { .width, .height, .depth,
           .channel_data_type, .channel_order,
           .array_size, .memory_layout };
```

### Examples

```
suq.width.b32       %r1, [surf_A];
```

