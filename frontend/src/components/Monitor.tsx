import type { Commitment, Evaluation } from '../client';
import { EvidenceScene } from '../three/EvidenceScene';
import { pct, riskPalette, scenarios, type Scenario } from '../theme';
import { Button, Hash, Panel, Stat } from './ui';

type Props = {
  items: Commitment[];
  selected: string;
  onSelect: (id: string) => void;
  results: Record<string, Evaluation>;
  scenario: Scenario;
  onScenario: (s: Scenario) => void;
  onEvaluate: () => void;
  onException: (asViewer: boolean) => void;
  onRegister: () => void;
  busy: boolean;
  loaded: boolean;
  policy: { allowed: boolean; message: string } | null;
};

export function Monitor(p: Props) {
  const commitment = p.items.find((c) => c.id === p.selected);
  const result = p.results[p.selected] ?? null;
  const band = riskPalette[result?.risk_band ?? 'idle'];

  return (
    <div className="grid grid-cols-1 gap-5 lg:grid-cols-[minmax(260px,320px)_1fr]">
      <Panel kicker={`${p.items.length} tracked`} title="Commitments" className="lg:row-span-2 lg:self-start">
        {p.loaded && p.items.length === 0 ? (
          <div className="rounded-2xl border border-dashed border-white/10 p-6 text-center text-sm text-slate-400">
            <p>No commitments yet.</p>
            <Button variant="primary" className="mt-4" onClick={p.onRegister}>Register the first promise</Button>
          </div>
        ) : (
          <ul className="flex flex-col gap-2">
            {p.items.map((item) => {
              const r = p.results[item.id];
              const color = riskPalette[r?.risk_band ?? 'idle'].css;
              const active = item.id === p.selected;
              return (
                <li key={item.id}>
                  <button
                    onClick={() => p.onSelect(item.id)}
                    className={`group w-full cursor-pointer rounded-2xl border p-3.5 text-left transition ${
                      active ? 'border-teal-300/50 bg-teal-300/[0.07]' : 'border-white/[0.06] bg-white/[0.02] hover:border-white/20'
                    }`}
                  >
                    <span className="flex items-center justify-between gap-3">
                      <strong className="truncate font-medium text-white">{item.name}</strong>
                      <span className="size-2 shrink-0 rounded-full" style={{ background: color, boxShadow: `0 0 12px ${color}` }} />
                    </span>
                    <span className="mt-1.5 flex justify-between font-mono text-xs text-slate-400">
                      <span className="truncate">{item.owner}</span>
                      <span>{pct(item.target)} · {item.window_minutes}m</span>
                    </span>
                  </button>
                </li>
              );
            })}
          </ul>
        )}
      </Panel>

      <section className="glass animate-rise relative overflow-hidden">
        <div className="relative h-[380px] sm:h-[440px]">
          <EvidenceScene points={p.scenario.points} target={commitment?.target ?? 0.95} evaluation={result} />
          <div className="pointer-events-none absolute inset-x-0 top-0 flex items-start justify-between gap-4 bg-gradient-to-b from-slate-950/80 to-transparent p-5 sm:p-6">
            <div>
              <p className="font-mono text-[0.68rem] tracking-[0.2em] text-teal-300/80 uppercase">Evidence run</p>
              <h2 className="mt-1 text-xl font-semibold tracking-tight text-white">{commitment?.name ?? 'Select a commitment'}</h2>
              {commitment && <p className="text-sm text-slate-400">Target {pct(commitment.target)} over {commitment.window_minutes} min</p>}
            </div>
            <span
              className="shrink-0 rounded-full border px-3 py-1 font-mono text-[0.68rem] tracking-[0.16em] whitespace-nowrap uppercase"
              style={{ color: band.css, borderColor: `${band.css}55`, background: `${band.css}14` }}
            >
              {band.label}
            </span>
          </div>
          {!result && commitment && (
            <p className="pointer-events-none absolute inset-x-0 bottom-28 text-center text-sm text-slate-400">
              Run an evaluation to project where this promise is heading.
            </p>
          )}
          <div className="absolute inset-x-0 bottom-0 flex flex-col gap-3 bg-gradient-to-t from-slate-950/90 to-transparent p-5 sm:flex-row sm:items-end sm:justify-between sm:p-6">
            <div role="radiogroup" aria-label="Synthetic scenario" className="flex gap-1 rounded-xl border border-white/10 bg-slate-950/60 p-1 backdrop-blur">
              {scenarios.map((s) => (
                <button
                  key={s.key}
                  role="radio"
                  aria-checked={s.key === p.scenario.key}
                  title={s.hint}
                  onClick={() => p.onScenario(s)}
                  className={`cursor-pointer rounded-lg px-3 py-1.5 text-xs font-medium transition ${
                    s.key === p.scenario.key ? 'bg-white/10 text-white' : 'text-slate-400 hover:text-white'
                  }`}
                >
                  {s.label}
                </button>
              ))}
            </div>
            <Button variant="primary" disabled={!p.selected || p.busy} onClick={p.onEvaluate}>
              Evaluate selected
            </Button>
          </div>
        </div>

        {result && (
          <div className="grid gap-3 border-t border-white/[0.06] p-5 sm:grid-cols-2 sm:p-6 xl:grid-cols-4">
            <Stat label="Breach horizon" value={pct(result.breach_probability)} sub="Probability of breaching the window" accent={band.css} />
            <Stat label="Observed mean" value={pct(result.observed_mean)} sub={`vs ${pct(commitment?.target ?? 0)} target`} />
            <Stat label="Lower bound" value={pct(result.lower_bound)} sub="Adaptive conformal" />
            <Stat label="E-value" value={result.e_value.toFixed(2)} sub={result.e_value >= 20 ? 'Strong evidence of shortfall' : 'Anytime-valid shortfall test'} />
            <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-xs text-slate-400 sm:col-span-2 xl:col-span-4">
              <span>RVF receipt <Hash value={result.receipt} /></span>
              <span>Correlation <Hash value={result.correlation_id} /></span>
              <span>
                {result.analogues.length
                  ? `${result.analogues.length} RuVector analogues in orbit`
                  : 'No historical analogues recalled yet'}
              </span>
            </div>
          </div>
        )}
      </section>

      <Panel kicker="Microsoft Regorus" title="Governed exception">
        <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
          <p className="max-w-xl text-sm text-slate-400">
            Request a 30-minute maintenance exception. Policy is executed server-side; a denied request appends nothing to the
            archive.
          </p>
          <div className="flex shrink-0 flex-col gap-2 whitespace-nowrap sm:flex-row">
            <Button variant="danger" disabled={!p.selected} onClick={() => p.onException(true)}>Try as viewer (deny)</Button>
            <Button disabled={!p.selected} onClick={() => p.onException(false)}>Approve as operator</Button>
          </div>
        </div>
        {p.policy && (
          <div
            className={`mt-4 flex items-center gap-3 rounded-xl border px-4 py-3 font-mono text-xs ${
              p.policy.allowed ? 'border-emerald-400/20 bg-emerald-400/5 text-emerald-300' : 'border-rose-400/20 bg-rose-400/5 text-rose-300'
            }`}
          >
            <span>{p.policy.allowed ? 'ALLOW' : 'DENY'}</span>
            <span className="text-slate-400">{p.policy.message}</span>
          </div>
        )}
      </Panel>
    </div>
  );
}
