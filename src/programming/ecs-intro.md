{{#include ../include/header014.md}}

# ECS Programming Introduction

This page will try to teach you the general ECS mindset/paradigm.

---

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

Also check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

ECS is a programming paradigm that separates data and behavior. Bevy will store
all of [your data][cb::ecs-intro-data] and manage all of [your individual pieces
of functionality][cb::ecs-intro-code] for you. The code will run when
appropriate. Your code can get access to whatever data it needs to do its thing.

This makes it easy to write game logic ([Systems][cb::system]) in a way that
is flexible and reusable. For example, you can implement:

- health and damage that works the same way for anything in the game,
  regardless of whether that's the player, an NPC, or a monster, or a vehicle
- gravity and collisions for anything that should have physics
- an animation or sound effect for all buttons in your UI

Of course, when you need specialized behavior only for specific entities (say,
player movement, which only applies to the player), that is naturally easy to
express, too.

If you are familiar with database programming, you will feel right at home. ECS
is conceptually very similar to a lightweight in-memory database.

[Read more about how to represent your data.][cb::ecs-intro-data]

[Read more about how to represent your functionality.][cb::ecs-intro-code]

bevy最核心的是ECS,ECS是一种将数据和逻辑完全分离的编程范式.
有很多称号,其中反oo和数据驱动开发用的比较多.

ECS的优势是逻辑解耦,小颗粒的组合提高了重用的可能.一定规模的项目比其他范式要简单很多.

- 生命值和伤害的工作方式,在一个游戏内对所有对象都是一样的.玩家/npc/boss/车辆等
- 物理中的引力和碰撞对所有物品,包括玩家,都是一样的
- 界面中按钮的音效和动画,这些运行原理都是一样的

只要是一样的,就可以单独成system.如果一个实体有多个逻辑,组合即可.

总之,ECS类似于内存中存储的轻量级数据库.
