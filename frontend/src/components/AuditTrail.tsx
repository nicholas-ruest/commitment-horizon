import { Hash, Panel } from './ui';

type AuditRecord = { record_type?: string; event?: string; receipt?: string; result?: { risk_band?: string; receipt?: string }; [k: string]: unknown };

const kinds: Record<string, { color: string; label: (r: AuditRecord) => string }> = {
  audit: { color: '#818cf8', label: (r) => r.event ?? 'audit event' },
  commitment: { color: '#5eead4', label: (r) => `Commitment ${String(r.name ?? r.id ?? '')}` },
  evidence: { color: '#38bdf8', label: (r) => `${Array.isArray(r.points) ? r.points.length : 0} evidence points ingested` },
  evaluation: { color: '#fb923c', label: (r) => `Evaluated · ${(r.result?.risk_band ?? 'unknown').replace('_', ' ')}` },
  exception: { color: '#34d399', label: (r) => `Exception · ${String(r.reason ?? '')}` },
};

export function AuditTrail({ records }: { records: unknown[] }) {
  const rows = records as AuditRecord[];
  return (
    <div className="grid grid-cols-1 gap-5 lg:grid-cols-[1fr_1.1fr]">
      <Panel kicker={`${rows.length} records · reloaded from RVF`} title="Restart-durable audit">
        <p className="-mt-2 mb-5 text-sm text-slate-400">Read back from the RVF archive — not browser state.</p>
        <ol className="relative ml-2 border-l border-white/10">
          {rows.map((r, i) => {
            const kind = kinds[r.record_type ?? ''] ?? { color: '#94a3b8', label: () => r.record_type ?? 'record' };
            const receipt = r.receipt ?? r.result?.receipt;
            return (
              <li key={i} className="animate-rise relative pb-5 pl-6 last:pb-0" style={{ animationDelay: `${i * 50}ms` }}>
                <span
                  className="absolute top-1.5 -left-[5px] size-2.5 rounded-full"
                  style={{ background: kind.color, boxShadow: `0 0 10px ${kind.color}` }}
                />
                <p className="font-mono text-[0.65rem] tracking-[0.16em] uppercase" style={{ color: kind.color }}>{r.record_type}</p>
                <p className="mt-0.5 text-sm text-slate-200">{kind.label(r)}</p>
                {receipt && <p className="mt-1"><Hash value={receipt} /></p>}
              </li>
            );
          })}
        </ol>
      </Panel>
      <Panel kicker="Canonical" title="Raw RVF records">
        <pre
          data-testid="audit-json"
          className="max-h-[60vh] overflow-auto rounded-xl border border-white/[0.06] bg-slate-950/70 p-4 font-mono text-[0.72rem] leading-relaxed break-all whitespace-pre-wrap text-slate-300"
        >
          {JSON.stringify(records, null, 2)}
        </pre>
      </Panel>
    </div>
  );
}
