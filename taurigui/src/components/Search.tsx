import { invoke } from "@tauri-apps/api/core";
import { Series } from "../App";

const StyleSearchPage = {
  margin: "0",
  display: "flex",
  flexDirection: "column" as "column",
  width: "100%",
  height: "100vh",
};

const StyleHeader = {
  display: "flex",
  flexDirection: "row" as "row",
  justifyContent: "space-between",
  alignItems: "center",
  padding: "1rem",
  border: "1px solid #ccc",
  borderRadius: "4px",
};

const ResultSearch = {
  display: "flex",
  flexDirection: "column" as "column",
  justifyContent: "flex-start",
  padding: "1rem",
  border: "1px solid #ccc",
  borderRadius: "4px",
  height: "100%",
  overflowY: "auto" as "auto",
};

const StyleSeries = {
  display: "flex",
  flexDirection: "row" as "row",
  justifyContent: "flex-start",
  border: "1px solid #ccc",
  borderRadius: "4px",
  margin: "1rem",
};

function Search(props: {
  input_text: string;
  setInputText: (text: string) => void;
  series: Series[];
  selected_series: Series[];
  setSeries: (series: Series[]) => void;
  setSelectedSeries: (series: Series[]) => void;
}) {
  const {
    input_text,
    setInputText,
    series,
    selected_series,
    setSeries,
    setSelectedSeries,
  } = props;

  async function search(keyword: string) {
    // print sereis
    console.log(series);
    // clean sereis
    setSeries([]);
    invoke("search", { keyword: keyword });
  }

  return (
    <div style={StyleSearchPage}>
      <div style={StyleHeader}>
        <input
          id="search-input"
          type="text"
          value={input_text}
          onChange={(e) => setInputText(e.target.value)}
        />
        <button onClick={() => search(input_text)}>search</button>
        <div>number: {series.length}</div>
      </div>
      <div className="result" style={ResultSearch}>
        <ul>
          {series.map((series) => (
            <li
              key={series.id}
              style={StyleSeries}
              onClick={() => {
                if (!selected_series.includes(series)) {
                  const _selected_series = [...selected_series, series];
                  setSelectedSeries(_selected_series);
                }
              }}
            >
              <img width="200" src={series.imageUrl} alt={series.title} />
              <h3>{series.title}</h3>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default Search;
