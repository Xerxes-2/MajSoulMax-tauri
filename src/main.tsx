import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Helper from "./Helper";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {window.location.pathname.match("helper") ? <Helper /> : <App />}
  </React.StrictMode>
);
