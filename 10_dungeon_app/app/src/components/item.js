import React from "react";

export const Item_Sword = ({ i, drop, sell }) => {
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
};

export const Item_Potion = ({ i, drop, sell }) => {
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
};
