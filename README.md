# Bevy 3d Animation Bug

This is just a minimal working example of what may be either a keyboard input or gltf animation bug

Animations do not work as expected when systems are decoupled:

ie:
```
            .add_system(keyboard_controls_look)// <--- Sets a state variable
            .add_system(set_animation_direction.after(keyboard_controls_look));// <--Should animate based on state variable but does not!
```

However, putting keyboard and animation in the same function appears to work

```
            .add_system(keyboard_directly_controls_animation); // <--- This works

```

Unclear when animations are supposed to work and not work under this framework, and inability to decouple components will tend to harm the long term architecture of the codebase (ie: giant systems, heavy coupling of controls with generic behavior such as movement).