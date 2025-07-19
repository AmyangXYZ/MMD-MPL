# PoPo

**MPL (MMD Pose Language)** - A rule-based DSL for controlling MMD bones using natural language commands.

## Why MPL?

Instead of manually calculating quaternions and dealing with bone direction complexities, MPL provides intuitive pose descriptions:

```
"look at the sky" → "head bend outward 60, neck bend outward 30"
"wave hello" → "arm_r bend outward 90, wrist_r bend inward 30"
"point forward" → "arm_r bend outward 45, index_r_1 bend outward 10"
```

## Syntax

**Format:** `bone action direction degrees`

**Actions:** `bend`, `turn`, `twist`  
**Directions:** `inward`, `outward`, `left`, `right`

**Examples:**

```
head turn left 30
arm_r bend outward 90
elbow_l bend inward 120
wrist_r twist left 45
thumb_r_0 bend inward 60
```

## Built-in Safety

- **Anatomical constraints**: Prevents impossible poses (elbows can't bend backward)
- **Range limits**: Automatically clamps values to realistic human movement ranges
- **Action validation**: Only allows anatomically correct bone movements

## 📄 License

GPL-3.0 License - see LICENSE for details.
