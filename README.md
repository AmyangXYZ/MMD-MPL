# PoPo

**MPL (MMD Pose Language)** - A rule-based DSL for controlling MMD bones using natural language commands.

## Why MPL?

Instead of manually calculating quaternions and dealing with bone direction complexities, MPL provides intuitive pose descriptions:

```
"look at the sky" → "head bend backward 60; neck bend backward 30"
"wave hello" → "arm_r bend forward 90; wrist_r bend forward 30"
"point forward" → "arm_r bend forward 45; index_r_0 bend forward 90"
```

## Syntax

**Format:** `bone action direction degrees`

**Actions:** `bend`, `turn`, `sway`  
**Directions:** `forward`, `backward`, `left`, `right`

**Single Statement Examples:**

```
head turn left 30
arm_r bend forward 90
elbow_l bend forward 120
wrist_r sway left 45
thumb_r_0 bend forward 60
```

**Multi-Statement Examples:**

```
head turn right 30; head bend backward 20
arm_l bend forward 60; arm_l sway left 30; shoulder_l bend forward 20
leg_l bend forward 45; leg_l sway left 25; ankle_l bend forward 15
head turn left 20; neck turn left 15; upper_body turn left 10
```

## Multi-Statement Support

MPL supports multiple statements separated by semicolons (`;`), allowing complex pose creation:

- **Multiple bones**: `"arm_l bend forward 45; arm_r bend forward 45; head turn left 15"`
- **Multiple actions on same bone**: `"head turn right 30; head bend backward 20"`
- **Joint chains**: `"head bend forward 25; neck bend forward 20; upper_body bend forward 15"`

## Built-in Safety

- **Anatomical constraints**: Prevents impossible poses (elbows can't bend backward)
- **Range limits**: Automatically clamps values to realistic human movement ranges
- **Action validation**: Only allows anatomically correct bone movements
- **Quaternion mathematics**: Properly combines multiple rotations on the same bone

## Supported Bones

**Body Core:**

- `base`, `center`, `upper_body`, `waist`, `neck`, `head`

**Arms:**

- `shoulder_l/r`, `arm_l/r`, `arm_twist_l/r`, `elbow_l/r`, `wrist_l/r`, `wrist_twist_l/r`

**Legs:**

- `leg_l/r`, `knee_l/r`, `ankle_l/r`, `forefoot_l/r`

**Fingers:**

- `thumb_l/r_0/1/2`, `index_l/r_0/1/2`, `middle_l/r_0/1/2`, `ring_l/r_0/1/2`, `pinky_l/r_0/1/2`

## 📄 License

GPL-3.0 License - see LICENSE for details.
