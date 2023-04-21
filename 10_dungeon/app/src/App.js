import logo from "./logo.svg";
import "./App.css";
import { useEffect, useState } from "react";

const App = () => {
  // 32 * 32
  const [player, updatePlayer] = useState({ x: 0, y: 0 });

  useEffect(() => {
    // starting an interval
    const interval = setInterval(async () => {
      // Fetching the player x position
      // res1 is the http answer
      const res1 = await fetch(
        "http://127.0.0.1:8080/state/value?path=/state/player/x_pos"
      );
      // Getting the response as text
      const x_pos_bytes = await res1.text();
      // Converting the text as number
      const x_pos = Number.parseInt(x_pos_bytes, 16);

      const res2 = await fetch(
        "http://127.0.0.1:8080/state/value?path=/state/player/y_pos"
      );
      const y_pos_bytes = await res2.text();
      const y_pos = Number.parseInt(y_pos_bytes, 16);

      updatePlayer({
        x: x_pos,
        y: y_pos,
      });
    }, 500); // The interval duration is 500ms
    return () => {
      // When the component umount, or refreshed we remove the interval
      setInterval(interval);
    };
  }, []);

  const move = (data) => async () => {
    const action = { data: data };

    const headers = new Headers();
    headers.append("Content-Type", "application/json");

    const res = await fetch("http://localhost:8080/operations", {
      body: JSON.stringify(action),
      method: "POST",
      headers,
    });
  };

  const moveDown = move("02");
  const moveUp = move("01");
  const moveLeft = move("03");
  const moveRight = move("04");

  return (
    <div className="App">
      <header className="App-header">
        <div className="title">Dungeon sequencer</div>
        <div id="map">
          {Array(32)
            .fill(0)
            .map((_, map_y) => {
              return (
                <div key={map_y} className="line">
                  {Array(32)
                    .fill(0)
                    .map((_, map_x) => {
                      if (map_x === player.x && map_y === player.y) {
                        return (
                          <div
                            key={`${map_x},${map_x}`}
                            className="cell player"
                          ></div>
                        );
                      } else {
                        return (
                          <div key={`${map_x},${map_x}`} className="cell"></div>
                        );
                      }
                    })}
                </div>
              );
            })}
        </div>

        <div className="buttons">
          <button onClick={moveLeft}>left</button>
          <div className="up-down">
            <button onClick={moveUp}>up</button>
            <button onClick={moveDown}>down</button>
          </div>
          <button onClick={moveRight}>right</button>
        </div>
      </header>
    </div>
  );
};

export default App;