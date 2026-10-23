import { useState, type FormEvent } from 'react';
import { pct } from '../theme';
import { Button, Panel } from './ui';

const field =
  'w-full rounded-xl border border-white/10 bg-slate-950/60 px-3.5 py-2.5 font-mono text-sm text-white placeholder:text-slate-600 transition focus:border-teal-300/60 focus:ring-2 focus:ring-teal-300/20 focus:outline-none';
const label = 'flex flex-col gap-2 text-xs font-medium tracking-wide text-slate-400';

export function RegisterForm({ onSubmit, busy }: { onSubmit: (e: FormEvent<HTMLFormElement>) => void; busy: boolean }) {
  const [target, setTarget] = useState(0.95);
  const [windowMin, setWindowMin] = useState(60);
  const budget = Math.max(0, (1 - target) * windowMin);

  return (
    <div className="grid grid-cols-1 gap-5 lg:grid-cols-[1.4fr_1fr]">
      <Panel kicker="Define a promise" title="Register a commitment">
        <form onSubmit={onSubmit} className="grid gap-4 sm:grid-cols-2">
          <label className={label}>ID<input className={field} name="id" required pattern="[A-Za-z0-9-]+" defaultValue="payments-slo" /></label>
          <label className={label}>Name<input className={field} name="name" required defaultValue="Payments success rate" /></label>
          <label className={label}>Owner<input className={field} name="owner" required defaultValue="payments-platform" /></label>
          <label className={label}>
            Target
            <input className={field} name="target" type="number" step="0.001" min="0" max="1" value={target} onChange={(e) => setTarget(Number(e.target.value))} />
          </label>
          <label className={`${label} sm:col-span-2`}>
            Window (minutes)
            <input className={field} name="window" type="number" min="5" max="10080" value={windowMin} onChange={(e) => setWindowMin(Number(e.target.value))} />
          </label>
          <Button variant="primary" disabled={busy} className="mt-2 sm:col-span-2">Create commitment</Button>
        </form>
      </Panel>

      <Panel kicker="Preview" title="Error budget">
        <div className="relative mt-2 h-3 overflow-hidden rounded-full bg-white/5">
          <div
            className="absolute inset-y-0 left-0 rounded-full bg-gradient-to-r from-teal-300 to-cyan-300 transition-all duration-500"
            style={{ width: `${Math.min(100, Math.max(0, target * 100))}%` }}
          />
        </div>
        <div className="mt-6 grid grid-cols-2 gap-3">
          <div>
            <p className="font-mono text-[0.68rem] tracking-[0.16em] text-slate-400 uppercase">Promise</p>
            <p className="mt-1 text-3xl font-semibold tabular-nums">{pct(target || 0)}</p>
          </div>
          <div>
            <p className="font-mono text-[0.68rem] tracking-[0.16em] text-slate-400 uppercase">Budget / window</p>
            <p className="mt-1 text-3xl font-semibold tabular-nums">{budget.toFixed(1)}<span className="text-base text-slate-400"> min</span></p>
          </div>
        </div>
        <p className="mt-6 text-sm leading-relaxed text-slate-400">
          Every evaluation runs adaptive conformal calibration and an anytime-valid e-process against this target, then
          archives the result to RVF with a verifiable receipt.
        </p>
      </Panel>
    </div>
  );
}
