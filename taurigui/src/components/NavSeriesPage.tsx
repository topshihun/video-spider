import { listen } from "@tauri-apps/api/event";
import { Episode, Series } from "../App";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { PlayerData } from "../player/Player";

const Style = {
  margin: "0",
  display: "flex",
  flexDirection: "column" as "column",
  width: "100%",
  height: "100vh",
};

const StyleHeader = {
  margin: "1rem",
  display: "flex",
  flexDirection: "row" as "row",
  justifyContent: "space-between",
  alignItems: "center",
  padding: "1rem",
  border: "1px solid #ccc",
  borderRadius: "4px",
  height: "200px",
};

const StyleContent = {
  overflowY: "auto" as "auto",
};

const StyleList = {
  display: "flex",
  flexDirection: "row" as "row",
  flexWrap: "wrap" as "wrap",
  alignItems: "flex-start",
  justifyContent: "flex-start",
  gap: "1rem",
  padding: "1rem",
  listStyle: "none",
};

const StyleListItem = {
  background: "#f0f0f0",
  borderRadius: "6px",
  padding: "15px",
  boxSizing: "border-box" as "border-box",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  textAlign: "center" as "center",
  cursor: "pointer",
};

var player_window_id = 0;
async function play(selected_series: Series, episode: Episode) {
  console.log("Playing episode:", episode.name);
  const player_window_label = `player-${player_window_id++}`;
  const player_window = new WebviewWindow(player_window_label, {
    url: `player.html?label=${player_window_label}`,
    width: 800,
    height: 600,
  });
  player_window.once("tauri://error", (e) => {
    console.error("created player window failed:", e);
  });
  player_window.once("tauri://webview-created", () => {
    player_window.show();
  });
  const player_data: PlayerData = {
    series: selected_series,
    selectedEpisode: episode,
  };
  const unlisten = await listen(`player-inited-${player_window_label}`, () => {
    player_window.emit(`play-episode-${player_window_label}`, player_data);
    unlisten();
  });
}

function NavSeriesPage(props: { selected_series: Series }) {
  const { selected_series } = props;

  return (
    <div style={Style}>
      <div style={StyleHeader}>
        <img
          height="100vh"
          src={selected_series.imageUrl}
          alt={selected_series.title}
        />
        <h2>{selected_series.title}</h2>
      </div>
      <div style={StyleContent}>
        <ul style={StyleList}>
          {selected_series.episodes.map((episode) => (
            <li
              key={episode.url}
              style={StyleListItem}
              onClick={() => play(selected_series, episode)}
            >
              {episode.name}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default NavSeriesPage;
