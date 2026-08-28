import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type AppStatus = {
  application: string;
  phase: string;
  cameraAccess: boolean;
};

function App() {
  const [status, setStatus] = useState<AppStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function checkAppStatus() {
    try {
      setError(null);
      setStatus(await invoke<AppStatus>("get_app_status"));
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : String(reason));
    }
  }

  return (
    <main className="container">
      <p className="eyebrow">Linux desktop utility</p>
      <h1>Linux Camera Controller</h1>
      <p className="intro">
        Rotate and route webcams through a virtual camera without living in the terminal.
      </p>

      <section className="status-card" aria-live="polite">
        <h2>Foundation status</h2>
        <p>
          The Tauri, React and Rust connection is the first verified building block.
          Camera access comes next.
        </p>
        <button type="button" onClick={checkAppStatus}>
          Check app status
        </button>

        {status && (
          <dl>
            <div>
              <dt>Application</dt>
              <dd>{status.application}</dd>
            </div>
            <div>
              <dt>Phase</dt>
              <dd>{status.phase}</dd>
            </div>
            <div>
              <dt>Camera access</dt>
              <dd>{status.cameraAccess ? "Available" : "Not implemented yet"}</dd>
            </div>
          </dl>
        )}

        {error && <p className="error">Could not read app status: {error}</p>}
      </section>
    </main>
  );
}

export default App;
