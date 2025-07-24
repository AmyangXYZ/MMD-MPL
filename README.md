# MPL - MMD Pose Language

MPL is a rule-based Domain-Specific Language for controlling MMD bones and make animations using natural language commands.

Example:

```
@pose bend {
    waist bend forward 90;
    head bend backward 15;
}

@pose look_right {
    head turn right 30;
}

@animation {
    bend_over 0.5s;
    look_right 1s;
}

main {
    animation;
}
```
