import React, { Component } from "react";

const Inventory = ({ player, drop, sell }) => {
  return (
    <div>
      <div>Inventory:</div>
      {player && player.gold && <div>Gold: {player.gold}</div>}
      {player.inventory.map((item, i) => {
        // matching the items as before for display
        switch (item) {
          case "01":
            return (
              <div className="item">
                {
                  // print the item cell
                }
                <div
                  // i : index is unit of the inventory
                  key={i}
                  className="cell sword"
                  tabIndex={0}
                />
                {
                  // Print the name of item kind on the right of item cell
                }
                <div className="item-name">Sword</div>
                {
                  // Add the button next ot the item to drop them, only with the
                  // click
                }
                <button onClick={drop(i)}>drop</button>
                <button onClick={sell(i)}>sell</button>
              </div>
            );
          case "02":
            return (
              <div className="item">
                <div
                  // i : index is unit of the inventory
                  key={i}
                  className="cell potion"
                  tabIndex={0}
                />
                <div className="item-name">Potion</div>
                <button onClick={drop(i)}>drop</button>
                <button onClick={sell(i)}>sell</button>
              </div>
            );
          default:
            return null;
        }
      })}
    </div>
  );
};

export default Inventory;
