# Todo

## Next potential features

1. Bind buttons (or function) to the arrow keys of the keyboard (React App) (done)
2. Display items inside the inventory (done)
3. Add walls: fix wall inside map (wip, add wall inside the new)
4. Later Add more information for item:
   1. kind of items: portion, weapon (done)
   2. function for portion (player can have level)
5. Map editor: convert string -> map? (with this we can have many kind of maps)

## Connect to wallet

Simplify version in kernel (not production ready but nice for demo with multiplayers game):

- Remove signature verification so there will be no verification.
- Player address (address), player action (address, action).

1. One player per address (done)
   - /state/player/x_pos => /state/players/{tz1...}/player/x_pos
   - /state/player/x_pos => /state/players/{tz1...}/inventory/ (wip)
2. Exchanges/transfer
   - Player should be able to exhange items between each others
     2.1 "drop_item" => removes from the inventory and add the item on the floor
     2.2 Exchange an interface where the player (state machine) selling items: into marketplace
     2.3 switching items
     - p1 open a trade with p2
     - p1 add an item
     - p1 validate what he is offering
     - p2 add an item
     - p1 valid the offer
     - the trade is done

## Randomness

BIG-STEP: Difficult but very nice to achive this

Random for rollup, chasing monster.
