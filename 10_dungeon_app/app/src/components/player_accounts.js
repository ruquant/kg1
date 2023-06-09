import React from "react";
import { ALICE_SECRET, BOB_SECRET } from "./player_adds";

const Player_accounts = ({ setSecret }) => {
  return (
    <div className="buttons-players">
      <button onClick={() => setSecret(ALICE_SECRET)}>SAM</button>
      <button onClick={() => setSecret(BOB_SECRET)}>GIMLI</button>
    </div>
  );
};

export default Player_accounts;
