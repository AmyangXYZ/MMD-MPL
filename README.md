# MPL - MMD Pose Language

MPL is a domain-specific language that makes 3D pose and animation accessible through code-like syntax and natural language style. It also enables LLMs to understand and generate animations by providing structured, semantic commands instead of complex quaternion calculations.

![](./screenshots/1.png)

[Gallery](https://popo.love/gallery) and [playground](https://popo.love/playground)

## Why MPL?

MPL bridges natural language and 3D animation with:

- **Semantic commands**: Write poses like code, not math
- **LLM-friendly syntax**: Predictable structure for AI generation
- **Built-in safety**: Prevents impossible poses automatically
- **Composable poses**: Reuse and combine animation components

## Syntax

### Pose Definitions

```
@pose kick_left {
    leg_l bend forward 30;
    knee_l bend backward 0;
    leg_r bend backward 20;
    knee_r bend backward 15;
}

@pose kick_right {
    leg_r bend forward 30;
    knee_r bend backward 0;
    leg_l bend backward 20;
    knee_l bend backward 15;
}
```

### Animation Sequences

```
@animation walk {
    0: kick_left;
    0.3: kick_right;
    0.6: kick_left;
    0.9: kick_right;
}
```

### Main Execution

```
main {
    walk;
}
```

## Bone Commands

**Format:** `bone action direction amount`

**Actions:** `bend`, `turn`, `sway`, `move`  
**Directions:** `forward`, `backward`, `left`, `right`, `up`, `down`

## Supported Bones

**Body Core:** `base`, `center`, `upper_body`, `waist`, `neck`, `head`  
**Arms:** `shoulder_l/r`, `arm_l/r`, `arm_twist_l/r`, `elbow_l/r`, `wrist_l/r`, `wrist_twist_l/r`  
**Legs:** `leg_l/r`, `knee_l/r`, `ankle_l/r`, `toe_l/r`  
**Fingers:** `thumb_0/1/2_l/r`, `index_0/1/2_l/r`, `middle_0/1/2_l/r`, `ring_0/1/2_l/r`, `pinky_0/1/2_l/r`

## Use Cases

- Convert natural language to animations ("wave hello")
- Modify existing poses and sequences
- Generate new poses by combining components
- Create complex animation timelines
- Validate and fix pose errors

## 📄 License

GPL-3.0 License - see LICENSE for details.
