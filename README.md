# PoPo

**MPL (MMD Pose Language)** - A rule-based DSL for controlling MMD bones using natural language commands.

## Why MPL?

Instead of manually calculating quaternions and dealing with bone direction complexities, MPL provides intuitive pose descriptions:

```
"look at the sky" → "head bend backward 60, neck bend backward 30"
"wave hello" → "arm_r bend backward 90, wrist_r bend forward 30"
"point forward" → "arm_r bend backward 45, index_r_1 bend backward 10"
```

## Syntax

**Format:** `bone action direction degrees`

**Actions:** `bend`, `turn`, `twist`  
**Directions:** `forward`, `backward`, `left`, `right`

**Examples:**

```
head turn left 30
arm_r bend backward 90
elbow_l bend forward 120
wrist_r twist left 45
thumb_r_0 bend forward 60
```

## Built-in Safety

- **Anatomical constraints**: Prevents impossible poses (elbows can't bend backward)
- **Range limits**: Automatically clamps values to realistic human movement ranges
- **Action validation**: Only allows anatomically correct bone movements

## 📄 License

GPL-3.0 License - see LICENSE for details.
