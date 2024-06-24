{{#include ../include/header013.md}}

# UV coordinates in Bevy

In Bevy, the vertical axis for the pixels of textures / images, and when
sampling textures in a shader, points *downwards*, from top to bottom. The
origin is at the top left.

This is inconsistent with the [World-coordinate system used everywhere else
in Bevy][cb::coords], where the Y axis points up.

It is, however, consistent with how most image file formats store pixel data,
and with how most graphics APIs work (including DirectX, Vulkan, Metal,
WebGPU, but *not* OpenGL).

OpenGL (and frameworks based on it) is different. If your prior experience
is with that, you may find that your textures appear flipped vertically.

---

If you are using a mesh, make sure it has the correct UV values. If it was
created with other software, be sure to select the correct settings.

If you are writing a custom shader, make sure your UV arithmetic is correct.

3d中的UV是将2d纹理映射到3d模型表面的一种方式,UV坐标用于定位纹理坐标,
和3d空间的xyz对应.

在 Bevy 中，纹理/图像像素的垂直轴以及在着色器中采样纹理时，从上到下向下指向。
原点位于左上角。

大部分图片格式都是符合这种设计的, 包括DirectX, Vulkan, Metal, WebGPU,
但不包括OpenGL. OpenGL中看起来是垂直翻转的.


## Sprites

If the images of your 2D sprites are flipped (for whatever reason), you can
correct that using Bevy's sprite-flipping feature:

```rust,no_run,noplayground
{{#include ../code011/src/pitfalls/uv_coordinates.rs:sprite-flipping}}
```

如果2d中的精灵图像看起来翻转了,可通过上面的例子处理一下.
