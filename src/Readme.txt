Dungeon:地牢
迷宮探索（英語：dungeon crawl）是一種奇幻角色扮演遊戲劇本類型，勇士通過迷宮環境，和各種怪物戰鬥，洗劫所發現的任何寶箱。
樸素的迷宮探索讓遊戲主持人比複雜冒險管理更容易管理，而「砍殺」風格的遊戲更獲得專注於動作和戰鬥的玩家之青睞。
因迷宮探索缺乏有意義劇情或邏輯一致，該術語逐漸帶有貶義。例如模仿遊戲《Munchkin》是關於「的地牢經驗的精髓……殺死怪物，偷寶物，刺向你的好友。」[1]

第一個電腦迷宮探索遊戲是《pedit5》，由Rusty Rutherford於1975年在美國伊利諾斯州厄巴納的柏拉圖系統上開發。
儘管遊戲很快從系統中刪除，但後來出現了一些類似的遊戲，如《dnd》和《Moria》。

Roguelike中的「迷宮探索」和RPG在故事和角色交互上有所區別，但術語還用來描述任何包含大量迷宮探索的遊戲（包括魔域、薩爾達和桌上RPG）。
最近術語開始表示第一人稱RPG，特別是對齊到網狀系統可以映射到方格紙上的。

Include concepts:

1 ) struct State:
    - Date extend from bracket-lib
    - The data describing maps, progress, stats, and everything else
      you need to keep between frames is the game’s state.
    - The game loop runs by calling your application’s tick() function with every
      frame. The tick() function doesn’t know anything about your game, so you
      need a way to store the game’s current status, known as the game state.
      Everything you need to preserve between frames is in your game’s state. The
      state represents a snapshot of the current game.
2 ) Trait
    Traits are a way to define shared functionality for objects.2 Traits are similar
    to interfaces found in other languages, which you use to define a contract.
    If a trait is implemented, its functionality is available, and the structure meets
    trait requirements.
    Bracket-lib defines a trait for games state structures named GameState.
    GameState requires that the object implement a tick() function.

3 ) Version 0.1.2
    The adventurer is the player’s avatar in the dungeon. The player roams the
    dungeon, hopefully collecting loot and probably dying at the hands of vicious
    monsters. The game needs to know what tiles the adventurers can enter so
    they don’t walk through walls. It also needs to store where the player is,
    handle moving them with the keyboard, and perform the movement. This
    section will address each of these requirements.

    Step 1 : we need to know (x,y) coordinate is within the bounds of map, so we implement
             the bounds checking first.
             Add function to map
             fn in_bound() -> bool
    Step 2 : we need a second function to determine if the player can enter a tile. Players
             can walk on floors but not through walls. This function should call the in_bounds
             function you just wrote, to ensure that the move is valid both dimensionally
             and for the TileType.
    Step 3 : Add one more function determining a tile’s index coordinates, and indicate an error
             condition if the requested coordinates fall outside of the map boundaries.

4 ) Version 0.1.3
    Subject: Create the Player Structure
    The player is a logically distinct entity, so it belongs in its own module. Create
    a new file in your src directory, and name it player.rs.

    With Player defined, we need to add it to our State and constructor, and call the update and render
    function in the tick() function.

5 ) Version 0.2.0
    Subject: Building a Dungeon
    In this section, you’ll learn to randomly place rooms, connect them with corridors, and build a dungeon
    for your adventurer to explore.
    Because the random map creation can be separated as indivdual module.

5.1 ) Version 0.2.1
      Subject: Fill the Map With Walls
      The previous example started with a dungeon entirely comprised of floors. A
      room-carving algorithm works the other way around, starting with a solid
      block of rock and carving rooms and corridors from the stone.


