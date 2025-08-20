# MPL - Motion Programming Language

MPL is a domain-specific language that revolutionizes 3D motion and animation through human-readable, semantic syntax. Designed to bridge the gap between natural language and 3D movement, MPL transforms complex mathematical representations into intuitive, code-like commands that both humans and AI systems can easily understand and generate.

**Current Implementation:** MMD (MikuMikuDance) format support with plans for broader 3D animation ecosystems.

![](./screenshots/1.png)

[Gallery](https://popo.love/gallery) and [playground](https://popo.love/playground)

## Why MPL?

MPL democratizes 3D motion creation and unlocks powerful AI capabilities:

### 🎯 **Human-Centric Design**

- **Semantic commands**: Express complex 3D movements through intuitive, readable syntax
- **Natural language alignment**: Bridge the gap between human intent and 3D mathematics
- **Built-in safety**: Anatomically-aware constraints prevent impossible poses automatically

### 🤖 **AI & Machine Learning Ready**

- **LLM-friendly syntax**: Structured, predictable grammar enables language models to understand and generate motion
- **Training-optimized**: Semantic tokens create rich datasets for AI motion synthesis
- **Compositional intelligence**: Modular pose components allow AI to learn and recombine movement patterns
- **Cross-modal potential**: Text-to-motion, motion-to-text, and motion-to-motion transformations

### 🔧 **Developer Benefits**

- **Composable architecture**: Reuse and combine animation building blocks
- **Version control friendly**: Text-based format integrates seamlessly with development workflows
- **Extensible framework**: Domain-agnostic design supports future 3D animation formats

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

### 🎬 **Creative Applications**

- **Natural language to motion**: Transform descriptions like "wave hello" or "sit down" into 3D animations
- **Procedural animation**: Generate variations and combinations of existing movement patterns
- **Interactive storytelling**: Create dynamic character animations through conversational interfaces

### 🤖 **AI & Research Applications**

- **Motion synthesis training**: Use MPL's semantic structure to train generative models for 3D animation
- **Cross-modal learning**: Enable AI systems to understand relationships between language, motion, and visual content
- **Behavioral modeling**: Research human movement patterns through structured, analyzable motion data
- **Animation assistance**: AI-powered tools for pose correction, completion, and stylistic adaptation

### 🛠️ **Development & Production**

- **Rapid prototyping**: Quickly iterate on character animations without complex 3D software
- **Automated content creation**: Generate animation assets programmatically for games and applications
- **Motion capture enhancement**: Post-process and refine motion capture data using semantic editing
- **Cross-platform compatibility**: Bridge different 3D animation formats through MPL's universal syntax

### 📚 **Education & Accessibility**

- **Animation learning**: Teach 3D animation concepts through intuitive, code-like syntax
- **Accessibility tools**: Enable motion creation for users without traditional 3D animation expertise
- **Documentation**: Create readable, maintainable animation specifications

## 🚀 Future Vision

MPL is designed as a foundational language for the next generation of AI-powered motion synthesis:

- **Universal Motion Representation**: Expand beyond MMD to support industry-standard formats (FBX, BVH, USD, etc.)
- **Large Motion Models (LMMs)**: Enable training of specialized AI models that understand human movement semantics
- **Multimodal AI Integration**: Seamless integration with vision, language, and audio AI systems
- **Real-time Motion Generation**: Live animation synthesis for gaming, VR/AR, and interactive media
- **Collaborative AI Tools**: Human-AI partnerships in creative motion design and animation production

## 📄 License

GPL-3.0 License - see LICENSE for details.
