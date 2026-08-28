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

type PipelineConfiguration = {
  sourceName: string;
  sourceAvailable: boolean;
  virtualCameraAvailable: boolean;
  transform: string;
};

type PipelineStatus = {
  state: "running" | "stopped";
  message: string;
};

function App() {
  const [preflight, setPreflight] = useState<PreflightResult | null>(null);
  const [pipelineConfiguration, setPipelineConfiguration] =
    useState<PipelineConfiguration | null>(null);
  const [pipelineStatus, setPipelineStatus] = useState<PipelineStatus | null>(null);
  const [isChecking, setIsChecking] = useState(true);
  const [isTogglingPipeline, setIsTogglingPipeline] = useState(false);
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

  async function loadPipelineDetails() {
    try {
      const [configuration, status] = await Promise.all([
        invoke<PipelineConfiguration>("get_pipeline_configuration"),
        invoke<PipelineStatus>("get_pipeline_status"),
      ]);
      setPipelineConfiguration(configuration);
      setPipelineStatus(status);
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : String(reason));
    }
  }

  async function refreshReadiness() {
    await checkPreflight();
    await loadPipelineDetails();
  }

  async function togglePipeline() {
    if (!pipelineStatus) {
      return;
    }

    try {
      setIsTogglingPipeline(true);
      setError(null);
      const command = pipelineStatus.state === "running" ? "stop_pipeline" : "start_pipeline";
      setPipelineStatus(await invoke<PipelineStatus>(command));
      await checkPreflight();
      await loadPipelineDetails();
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : String(reason));
      await loadPipelineDetails();
    } finally {
      setIsTogglingPipeline(false);
    }
  }

  useEffect(() => {
    void refreshReadiness();
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
        <button type="button" onClick={refreshReadiness} disabled={isChecking}>
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

      <section className="pipeline-card" aria-live="polite">
        <div className="section-heading">
          <div>
            <h2>Camera pipeline</h2>
            <p>First verified setup: StreamCam at 720p/30 with a 180° rotation.</p>
          </div>
          <span className={`pipeline-state ${pipelineStatus?.state ?? "stopped"}`}>
            {pipelineStatus?.state === "running" ? "Running" : "Stopped"}
          </span>
        </div>

        <dl className="pipeline-settings">
          <div>
            <dt>Input</dt>
            <dd>
              {pipelineConfiguration?.sourceAvailable
                ? `${pipelineConfiguration.sourceName} (detected)`
                : "Logitech StreamCam not detected"}
            </dd>
          </div>
          <div>
            <dt>Transform</dt>
            <dd>{pipelineConfiguration?.transform ?? "180° rotation"}</dd>
          </div>
          <div>
            <dt>Output</dt>
            <dd>
              {pipelineConfiguration?.virtualCameraAvailable
                ? "Configured virtual camera"
                : "Virtual camera not detected"}
            </dd>
          </div>
        </dl>

        <button
          type="button"
          className={pipelineStatus?.state === "running" ? "stop-button" : undefined}
          onClick={togglePipeline}
          disabled={!preflight?.ready || !pipelineConfiguration?.sourceAvailable || !pipelineConfiguration?.virtualCameraAvailable || isTogglingPipeline}
        >
          {isTogglingPipeline
            ? "Working…"
            : pipelineStatus?.state === "running"
              ? "Stop camera"
              : "Start rotated camera"}
        </button>

        {pipelineStatus && <p className="pipeline-message">{pipelineStatus.message}</p>}
      </section>
    </main>
  );
}

export default App;
