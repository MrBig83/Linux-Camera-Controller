import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type CheckStatus = "ready" | "attention";

type PreflightCheck = {
  id: string;
  label: string;
  status: CheckStatus;
  summary: string;
  nextStep: string | null;
};

type PreflightResult = {
  ready: boolean;
  summary: string;
  checks: PreflightCheck[];
};

function App() {
  const [preflight, setPreflight] = useState<PreflightResult | null>(null);
  const [isChecking, setIsChecking] = useState(true);
  const [error, setError] = useState<string | null>(null);

  async function checkPreflight() {
    try {
      setIsChecking(true);
      setError(null);
      setPreflight(await invoke<PreflightResult>("get_preflight"));
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setIsChecking(false);
    }
  }

  useEffect(() => {
    void checkPreflight();
  }, []);

  return (
    <main className="container">
      <p className="eyebrow">Linux desktop utility</p>
      <h1>Linux Camera Controller</h1>
      <p className="intro">
        Rotate and route webcams through a virtual camera without living in the terminal.
      </p>

      <section className="status-card" aria-live="polite">
        <h2>System readiness</h2>
        <p>
          Check the local tools and virtual camera before starting a camera pipeline.
        </p>
        <button type="button" onClick={checkPreflight} disabled={isChecking}>
          {isChecking ? "Checking…" : "Refresh readiness"}
        </button>

        {preflight && (
          <div className="preflight-results">
            <p className={`readiness ${preflight.ready ? "ready" : "attention"}`}>
              {preflight.summary}
            </p>
            <dl>
              {preflight.checks.map((check) => (
                <div key={check.id}>
                  <dt>
                    <span className={`check-indicator ${check.status}`} aria-hidden="true" />
                    {check.label}
                  </dt>
                  <dd>
                    <span>{check.summary}</span>
                    {check.nextStep && <small>{check.nextStep}</small>}
                  </dd>
                </div>
              ))}
            </dl>
          </div>
        )}

        {error && <p className="error">Could not check system readiness: {error}</p>}
      </section>
    </main>
  );
}

export default App;
