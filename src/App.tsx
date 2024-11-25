import { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";
import ReactAnsi from "react-ansi";

enum LogLevel {
  Trace = 1,
  Debug,
  Info,
  Warn,
  Error,
}

interface RecordPayload {
  level: LogLevel;
  message: string;
}

function App() {
  const [logs, setLogs] = useState("");
  const [filter, setFilter] = useState<
    "All" | "INFO" | "WARN" | "ERROR" | "TRACE" | "DEBUG"
  >("All");
  const logEndRef = useRef<HTMLDivElement>(null);

  // Function to start the proxy
  async function startProxy() {
    try {
      await invoke("start_proxy");
    } catch (error) {
      console.error("Failed to start proxy:", error);
    }
  }

  // Function to stop the proxy
  async function stopProxy() {
    try {
      await invoke("stop_proxy");
    } catch (error) {
      console.error("Failed to stop proxy:", error);
    }
  }

  // Function to add log messages to the state
  const addLog = (log: string) => {
    setLogs((prevLogs) => prevLogs + log + "\n");
  };

  // Set up event listener for 'log://log'
  useEffect(() => {
    const unlisten = listen("log://log", (event: Event<RecordPayload>) => {
      const { message } = event.payload;
      addLog(message);
    });

    return () => {
      unlisten
        .then((fn) => fn())
        .catch((err) => console.error("Failed to unlisten:", err));
    };
  }, []);

  // Auto-scroll to the latest log
  useEffect(() => {
    if (logEndRef.current) {
      logEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [logs]);

  return (
    <main
      className="container"
      style={{
        padding: "20px",
        fontFamily: "Arial, sans-serif",
        maxWidth: "800px",
        margin: "0 auto",
      }}
    >
      <h1>Proxy Controller</h1>

      <div className="button-group" style={{ marginBottom: "20px" }}>
        <button
          type="button"
          onClick={startProxy}
          style={{
            padding: "10px 20px",
            marginRight: "10px",
            backgroundColor: "#4CAF50",
            color: "white",
            border: "none",
            borderRadius: "4px",
            cursor: "pointer",
          }}
        >
          Start Proxy
        </button>
        <button
          type="button"
          onClick={stopProxy}
          style={{
            padding: "10px 20px",
            backgroundColor: "#f44336",
            color: "white",
            border: "none",
            borderRadius: "4px",
            cursor: "pointer",
          }}
        >
          Stop Proxy
        </button>
      </div>

      <h2>Rust Logs</h2>

      <div style={{ marginBottom: "10px" }}>
        <label htmlFor="filter">Filter: </label>
        <select
          id="filter"
          value={filter}
          onChange={(e) => setFilter(e.target.value as any)}
          style={{ padding: "5px", marginRight: "10px" }}
        >
          <option value="All">All</option>
          <option value="TRACE">TRACE</option>
          <option value="DEBUG">DEBUG</option>
          <option value="INFO">INFO</option>
          <option value="WARN">WARN</option>
          <option value="ERROR">ERROR</option>
        </select>
        <button
          type="button"
          onClick={() => setLogs("")}
          style={{
            padding: "5px 10px",
            backgroundColor: "#555",
            color: "white",
            border: "none",
            borderRadius: "4px",
            cursor: "pointer",
          }}
        >
          Clear Logs
        </button>
      </div>

      <div
        style={{
          padding: "10px",
          height: "300px",
          overflowY: "scroll",
          fontFamily: "monospace",
          fontSize: "12px",
          borderRadius: "5px",
          marginTop: "10px",
        }}
      >
        <ReactAnsi log={logs} />
        <div ref={logEndRef} />
      </div>
    </main>
  );
}

export default App;
