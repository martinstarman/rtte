# TODO

- [ ] fix player pathfinding on entity selection
- [x] fix sprite selection under another sprite (eg. select enemy under tree)
- [ ] use Commands for entity spawning (see `object.rs`)
- [ ] add possibility to wait in enemy path (should be in idle state)
- [x] replace `Directions` with Bevy CompasOctant
- [ ] use polygons as `PrimitiveObstacle` (vleue_navigator does not supports polygons right now)
- [ ] stop using gizmos for line of sight and debug (bevy does not suports polygons right now)
- [x] add player UI
- [ ] serialization/deserialization
- [ ] add possibility for player to kill enemies
- [ ] menu
- [ ] cursor is not reset on player deselect
