import logo from "./logo.svg";
import "./App.css";
import { useEffect, useState } from "react";

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

// Player actions
const moveUp = move("01");
const moveDown = move("02");
const moveLeft = move("03");
const moveRight = move("04");
const pickUp = move("05");

/**
 * Split a string into n slices
 */
const splitNChars = (txt) => {
  let result = [];
  for (let i = 0; i < txt.length; i += 2) {
    // 2 is the length of the bytes
    result.push(txt.substr(i, 2));
  }
  return result;
};

const App = () => {
  // handlekey
  const onKeyDown = (e) => {
    // 37 = left, 38 = up, 39 = right, 40 = down
    if (e.key === 38) {
      move("01");
    } else if (e.key === 40) {
      move("02");
    } else if (e.key === 37) {
      move("03");
    } else if (e.key === 39) {
      move("04");
    }
  };

  // 32 * 32
  const [player, updatePlayer] = useState({ x: 0, y: 0 });

  // place item on map, tiles are: 01, 02, 03, 04 is a binary update_state
  // correspond with the one define in rust
  // 0x01 => Some(TileType::Floor(Some(Sword))),
  // 0x02 => Some(TileType::Floor(Some(Potion))),
  // 0x03 => Some(TileType::Floor(None)),
  // 0x04 => Some(TileType::Wall),
  const [map, updateMap] = useState({ tiles: ["01", "02", "03", "04"] });

  useEffect(() => {
    // starting an interval
    const interval = setInterval(async () => {
      // Player
      // Fetching the player x position, res1 is the http answer
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

      // Map -> Item
      const res3 = await fetch(
        "http://127.0.0.1:8080/state/value?path=/state/map"
      );

      const map_bytes = await res3.text();
      // spliting the array to a list of string
      const map = splitNChars(map_bytes);

      // we can put new value to the map
      updateMap(map);
    }, 500); // The interval duration is 500ms
    return () => {
      // When the component umount, or refreshed we remove the interval
      setInterval(interval);
    };
  }, []);

  // App interface
  return (
    <div className="App">
      <header className="App-header">
        <div className="title">Dungeon sequencer</div>
        <div id="map">
          {Array(32)
            .fill(0)
            .map((_, map_y) => {
              return (
                // draw the map vertical
                <div key={map_y} className="line">
                  {Array(32)
                    .fill(0)
                    // draw the map horizontal
                    .map((_, map_x) => {
                      // define map_idx as in the kernel
                      let idx = map_y * 32 + map_x;

                      // place the player position on the map
                      if (map_y === player.y && map_x === player.x) {
                        return (
                          <div
                            key={`${map_x},${map_x}`}
                            className="cell player"
                            tabIndex={0}
                            onKeyDown
                          ></div>
                        );
                      }

                      // define the tile is the map_idx
                      let tile = map[idx];
                      // matching the tile
                      switch (tile) {
                        // checking each case for the items on the map
                        case "01":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell sword"
                              tabIndex={0}
                            ></div>
                          );
                        // potion
                        case "02":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell potion"
                              tabIndex={0}
                            ></div>
                          );
                        case "03":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell"
                              tabIndex={0}
                            ></div>
                          );
                        case "04":
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell wall"
                              tabIndex={0}
                            ></div>
                          );

                        // if other cases return the white cell for floor
                        default:
                          return (
                            <div
                              key={`${map_x},${map_x}`}
                              className="cell"
                              tabIndex={0}
                            ></div>
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
          <button onClick={pickUp}>pick up</button>
        </div>
      </header>
    </div>
  );
};

export default App;
