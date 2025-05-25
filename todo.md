# TODO

- [ ] fix player movement on different action - https://github.com/bevyengine/bevy/issues/3570
- [x] fix sprite selection under another sprite (eg. select enemy under tree)
- [ ] use Commands for entity spawning (see `object.rs`)
- [ ] add possibility to wait in enemy path (should be in idle state)
- [x] replace `Directions` with Bevy CompasOctant
- [ ] use polygons as `PrimitiveObstacle` (vleue_navigator does not supports polygons right now)
- [ ] stop using gizmos for line of sight and debug (bevy does not suports polygons right now)
- [x] add player UI
- [ ] serialization/deserialization
- [x] add possibility for player to kill enemies
- [ ] menu
- [x] enemy dead state
- [ ] action target
- [x] reset movement only for selected players
- [ ] fix sometimes wrong first frame for dead enemy
- [x] visualize selected action in ui
- [ ] see trough blocks
- [ ] fix path_draw() system
