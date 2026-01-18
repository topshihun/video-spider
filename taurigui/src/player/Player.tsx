import { Episode, Series } from "../App";
import { listen } from "@tauri-apps/api/event";
import { useState, useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { Plyr } from "plyr-react";
import "plyr-react/plyr.css";

export type PlayerData = {
  series: Series;
  selectedEpisode: Episode;
};

function Player() {
  const [player_data, setPlayerData] = useState<PlayerData | null>(null);
  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const label = searchParams.get("label");
    const unlisten = listen(`play-episode-${label}`, (event) => {
      setPlayerData(event.payload as PlayerData);
    });

    emit(`player-inited-${label}`, null);

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const plyrProps = {
    controls: true,
    source: {
      type: "video" as "video",
      sources: [
        {
          src: player_data?.selectedEpisode.url || "",
        },
      ],
    },
  };

  return (
    <div>
      <h3>{player_data?.selectedEpisode?.name}</h3>
      <Plyr {...plyrProps} />
    </div>
  );
}

export default Player;
