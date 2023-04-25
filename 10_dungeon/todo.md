# Todo

## Next potential features

1. Bind buttons (or function) to the arrow keys of the keyboard (React App)
2. Display items inside the inventory
3. Add walls: fix wall inside map
4. Later Add more information for item:
   1. kind of items: portion, weapon
   2. function for portion (player can have level)
5. Map editor: convert string -> map? (with this we can have many kind of maps)


## Connect to wallet

Simplify version in kernel (not production ready but nice for demo with multiplayers game):

- Remove signature verification so there will be no verification.
- Player address (address), player action (address, action).


1. One player per address
   - /state/player/x_pos => /state/players/{tz1...}/player/x_pos
   - /state/player/x_pos => /state/players/{tz1...}/inventory/
2. Exchanges
   - Player should be able to exhange items between each others

## Randomness

BIG-STEP: Difficult but very nice to achive this

Random for rollup, chasing monster.
