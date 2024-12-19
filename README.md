# basic text adventure engine
- reusable framework, just need to implement unique commands, rooms and items
---

I mainly wanted to make a text adventure game from scratch as a rust learning project, and I'll mostly likely be building a game or two using this, both of which will be compiled to wasm and playable online, likely through github pages.

I plan on adding a little more item interactivity, maybe some npc conversation ability as well. Commands are fairly straightforward for unique use cases, as are items and rooms.

Rooms are stored in a vector in the world, so when creating a map, I suggest drawing one out and numbering the rooms. You can create rooms in any order, and start the player in any room. see the example in the main fn.
