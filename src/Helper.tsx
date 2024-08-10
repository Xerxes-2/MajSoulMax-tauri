import { useEffect, useState } from "react";
import "./fonts/Noto_Sans_SC/NotoSansSC-VariableFont_wght.ttf";
import { listen } from "@tauri-apps/api/event";

const Helper = () => {
  const [text, setText] = useState("114514");

  useEffect(() => {
    let unlisten;
    document.documentElement.style.backgroundColor = "transparent";
    listen("helper", (event) => {
      console.log(event.payload);
      setText(event.payload as string);
    }).then((res) => {
      unlisten = res;
    });
    return unlisten;
  }, []);

  return <div className="helper">{text}</div>;
};

export default Helper;
