import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { attachConsole } from "@tauri-apps/plugin-log";
import "./App.css";
import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getVersion } from "@tauri-apps/api/app";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const [update, setUpdate] = useState<Update | null>();
  const [currentVersion, setCurrentVersion] = useState<string>();
  const [downloadInfo, setDownloadInfo] = useState({
    downloaded: 0,
    contentLength: 0,
  });

  const checkForUpdates = async () => {
    const update = await check();
    setUpdate(update);
  };

  const downloadUpdate = async () => {
    if (update) {
      let downloaded = 0;
      let contentLength = 0;
      // alternatively we could also call update.download() and update.install() separately
      await update.download((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength ?? 0;
            console.log(
              `started downloading ${event.data.contentLength} bytes`
            );
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            break;
          case "Finished":
            console.log("download finished");
            setUpdate(update);
            break;
        }

        // only update the state if downloaded increased by more than 5mb
        if (
          downloadInfo.downloaded == 0 ||
          downloaded > downloadInfo.downloaded + 5 * 1024 * 1024
        ) {
          setDownloadInfo({
            downloaded: downloaded,
            contentLength: contentLength,
          });
        }
      });
    }
  };

  const applyUpdate = async () => {
    if (update) {
      await update.install();
      await relaunch();
    }
  };

  useEffect(() => {
    const initConsole = async () => {
      return await attachConsole();
    };

    let detach: (() => void) | undefined;
    initConsole().then((unlisten) => {
      detach = unlisten;
    });

    getVersion().then((version) => {
      setCurrentVersion(version);
    });

    return () => {
      if (detach) {
        detach();
      }
    };
  }, []);

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <div className="row">
        <div style={{ display: "flex", flexDirection: "column", gap: "20px" }}>
          <div>
            <p>Current version: {currentVersion}</p>
            <button
              type="button"
              onClick={async () => {
                await checkForUpdates();
              }}
            >
              Check for Updates
            </button>
          </div>

          {update ? (
            <div>
              <p>Update available: {update.version}</p>
              {downloadInfo.contentLength > 0 &&
              downloadInfo.contentLength == downloadInfo.downloaded ? (
                <button
                  type="button"
                  onClick={async () => {
                    await applyUpdate();
                  }}
                >
                  Apply Update
                </button>
              ) : (
                <button
                  type="button"
                  onClick={async () => {
                    await downloadUpdate();
                  }}
                >
                  {downloadInfo.contentLength > 0 ? (
                    <>
                      Downloading Update{" "}
                      {(
                        (downloadInfo.downloaded / downloadInfo.contentLength) *
                        100
                      ).toFixed(2)}
                      %
                    </>
                  ) : (
                    <>Download Update</>
                  )}
                </button>
              )}
            </div>
          ) : (
            <></>
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
