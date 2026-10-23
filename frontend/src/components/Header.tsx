export type View = 'monitor' | 'register' | 'audit';

type Props = { view: View; onNavigate: (view: View) => void; onAudit: () => void; canAudit: boolean; online: boolean };

export function Header({ view, onNavigate, onAudit, canAudit, online }: Props) {
  const tab = (active: boolean) =>
    `relative cursor-pointer rounded-lg px-3.5 py-2 text-sm font-medium transition disabled:cursor-not-allowed disabled:opacity-40 ${
      active ? 'bg-white/10 text-white shadow-inner' : 'text-slate-400 hover:text-white'
    }`;
  return (
    <header className="flex flex-col gap-6 pt-6 pb-8 md:flex-row md:items-end md:justify-between">
      <div className="animate-rise">
        <div className="flex items-center gap-3">
          <svg viewBox="0 0 32 32" className="size-9" aria-hidden>
            <defs>
              <linearGradient id="mark" x1="0" x2="1" y1="0" y2="1">
                <stop offset="0" stopColor="#5eead4" />
                <stop offset="1" stopColor="#818cf8" />
              </linearGradient>
            </defs>
            <circle cx="16" cy="16" r="14" fill="none" stroke="url(#mark)" strokeWidth="1.5" opacity="0.5" />
            <path d="M3 19c5-4 9-5 13-5s8 1 13 5" fill="none" stroke="url(#mark)" strokeWidth="2.2" strokeLinecap="round" />
            <circle cx="16" cy="10" r="2.6" fill="#5eead4" />
          </svg>
          <p className="font-mono text-[0.7rem] tracking-[0.28em] text-teal-300 uppercase">Operational intelligence</p>
        </div>
        <h1 className="mt-3 bg-gradient-to-br from-white via-slate-100 to-slate-400 bg-clip-text text-4xl font-semibold tracking-[-0.04em] text-transparent sm:text-6xl">
          Commitment Horizon
        </h1>
        <p className="mt-2 max-w-md text-slate-400">See service promises fail before customers do.</p>
      </div>
      <div className="flex flex-col items-start gap-3 md:items-end">
        <span
          className={`inline-flex items-center gap-2 rounded-full border px-3 py-1 font-mono text-[0.68rem] tracking-[0.16em] uppercase ${
            online ? 'border-emerald-400/30 bg-emerald-400/10 text-emerald-300' : 'border-rose-400/30 bg-rose-400/10 text-rose-300'
          }`}
        >
          <span className={`size-1.5 rounded-full ${online ? 'animate-pulse bg-emerald-300' : 'bg-rose-300'}`} />
          {online ? 'Live · RVF' : 'API offline'}
        </span>
        <nav aria-label="Primary" className="glass flex gap-1 !rounded-xl p-1">
          <button className={tab(view === 'monitor')} onClick={() => onNavigate('monitor')}>Horizon</button>
          <button className={tab(view === 'register')} onClick={() => onNavigate('register')}>New commitment</button>
          <button className={tab(view === 'audit')} disabled={!canAudit} onClick={onAudit}>Audit</button>
        </nav>
      </div>
    </header>
  );
}
