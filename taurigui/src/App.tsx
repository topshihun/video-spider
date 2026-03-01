import { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import Home from "./components/Home";
import Search from "./components/Search";
import NavSeriesPage from "./components/NavSeriesPage";
import CloseableButton from "./components/CloseableButton";

const StyleSelectSeries = {
  display: "flex",
  flexDirection: "column" as "column",
  margin: "0",
  width: "100%",
  padding: "0.5rem",
  border: "1px solid #ccc",
  borderRadius: "4px",
  overflowY: "auto" as "auto",
  overflowX: "hidden" as "hidden",
};

enum Tab {
  HOME,
  SEARCH,
  SERIES,
}

export type Episode = {
  name: string;
  url: string;
};

export type Series = {
  id: number;
  title: string;
  description: string;
  imageUrl: string;
  episodes: Episode[];
};

type SearchError = {
  from: string;
  message: string;
};

function App() {
  const [input_text, setInputText] = useState("");
  const [series, setSeries] = useState<Series[]>([]);
  const [selected_series, setSelectedSeries] = useState<Series[]>([]);
  const [selected_series_page, setSelectedSeriesPage] = useState<Series>({
    id: 0,
    title: "This something wrong!",
    description: "This is a initil series tab.",
    imageUrl: "",
    episodes: [],
  });
  const [tab, setTab] = useState(Tab.HOME);

  const seriesRef = useRef<Series[]>(series);

  function pageShow() {
    if (tab === Tab.HOME) {
      return <Home />;
    } else if (tab === Tab.SEARCH) {
      return (
        <Search
          input_text={input_text}
          setInputText={setInputText}
          series={series}
          selected_series={selected_series}
          setSeries={setSeries}
          setSelectedSeries={setSelectedSeries}
        />
      );
    } else if (tab === Tab.SERIES) {
      console.log("selected_series", selected_series_page);
      return <NavSeriesPage selected_series={selected_series_page} />;
    } else {
      return <h1>Unknown page</h1>;
    }
  }

  useEffect(() => {
    const unlisten_series = listen<Series>("search_result", (event) => {
      setSeries([...seriesRef.current, event.payload]);
    });

    const unlisten_error = listen<SearchError>("search_error", (event) => {
      console.log(`Error from ${event.payload.from}: ${event.payload.message}`);
    });

    return () => {
      unlisten_series.then((f) => f());
      unlisten_error.then((f) => f());
    };
  }, []);

  useEffect(() => {
    seriesRef.current = series;
  }, [series]);

  return (
    <main className="container">
      <div className="nav">
        <button onClick={() => setTab(Tab.HOME)}>Home</button>
        <button onClick={() => setTab(Tab.SEARCH)}>Search</button>
        <div style={StyleSelectSeries}>
          {selected_series.map((series) => (
            <div>
              <CloseableButton
                content={series.title}
                onClick={() => {
                  setSelectedSeriesPage(series);
                  setTab(Tab.SERIES);
                }}
                close={() => {
                  setTab(Tab.SEARCH);
                  setSelectedSeries(
                    selected_series.filter((s) => s.id !== series.id),
                  );
                }}
              />
            </div>
          ))}
        </div>
      </div>
      <div className="content">{pageShow()}</div>
    </main>
  );
}

export default App;
