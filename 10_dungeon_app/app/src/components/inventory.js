import React, { Component } from "react";
import { Item_Potion, Item_Sword } from "./item";

const Inventory = ({ player, drop, sell }) => {
  return (
    <div>
      <div>Inventory:</div>
      {player && player.gold && <div>Gold: {player.gold}</div>}
      {player.inventory.map((item, i) => {
        // matching the items as before for display
        switch (item) {
          case "01":
            return <Item_Sword i={i} drop={drop} sell={sell} />;
          case "02":
            return <Item_Potion i={i} drop={drop} sell={sell} />;
          default:
            return null;
        }
      })}
    </div>
  );
};

export default Inventory;
