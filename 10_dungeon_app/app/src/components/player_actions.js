import React from "react";

const Player_actions = ({ move, signer }) => {
  const moveUp = move("01", signer);
  const moveDown = move("02", signer);
  const moveLeft = move("03", signer);
  const moveRight = move("04", signer);
  const pickUp = move("05", signer);

  return (
    <div className="buttons">
      <button onClick={moveLeft}>left</button>

      <div className="up-down">
        <button onClick={moveUp}>up</button>
        <button onClick={moveDown}>down</button>
      </div>
      <button onClick={moveRight}>right</button>
      <button onClick={pickUp}>pick up (y)</button>
    </div>
  );
};

export default Player_actions;
