import logo from "./logo.svg";
import "./App.css";
import { useState } from "react";

const App = () => {
  // 32 * 32
  const [player, updatePlayer] = useState({ x: 0, y: 0 });

  return (
    <div className="App">
      <header className="App-header">
        <div id="map">
          {Array(32)
            .fill(0)
            .map((_, map_y) => {
              return (
                <div>
                  {Array(32)
                    .fill(0)
                    .map((_, map_x) => {
                      console.log({ map_x, map_y });
                      console.log(player);
                      if (map_x === player.x && map_y === player.y) {
                        console.log("hello");
                        return <div className="cell player"></div>;
                      } else {
                        return <div className="cell"></div>;
                      }
                    })}
                </div>
              );
            })}
        </div>
      </header>
    </div>
  );
};

export default App;
