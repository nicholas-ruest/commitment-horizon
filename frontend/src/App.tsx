import { type FormEvent, useEffect, useState } from 'react';
import { api, type Commitment, type Evaluation } from './client';
import { AuditTrail } from './components/AuditTrail';
import { Header, type View } from './components/Header';
import { Monitor } from './components/Monitor';
import { RegisterForm } from './components/RegisterForm';
import { Toasts } from './components/Toasts';
import { scenarios } from './theme';
import { HorizonBackdrop } from './three/HorizonBackdrop';

export function App() {
  const [items, setItems] = useState<Commitment[]>([]);
  const [selected, setSelected] = useState('');
  const [results, setResults] = useState<Record<string, Evaluation>>({});
  const [scenario, setScenario] = useState(scenarios[1]);
  const [audit, setAudit] = useState<unknown[]>([]);
  const [view, setView] = useState<View>('monitor');
  const [busy, setBusy] = useState(false);
  const [loaded, setLoaded] = useState(false);
  const [online, setOnline] = useState(true);
  const [policy, setPolicy] = useState<{ allowed: boolean; message: string } | null>(null);
  const [notice, setNotice] = useState('');
  const [error, setError] = useState('');

  const refresh = async () => {
    setBusy(true);
    try {
      const rows = await api.list();
      setItems(rows);
      setOnline(true);
      setSelected((current) => current || rows[0]?.id || '');
    } catch (e) {
      setOnline(false);
      setError(String(e));
    } finally {
      setBusy(false);
      setLoaded(true);
    }
  };
  useEffect(() => {
    void refresh();
  }, []);

  const submit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError('');
    setBusy(true);
    const data = new FormData(e.currentTarget);
    try {
      const row = await api.create({
        id: data.get('id'),
        name: data.get('name'),
        owner: data.get('owner'),
        target: Number(data.get('target')),
        window_minutes: Number(data.get('window')),
      });
      setNotice(`Registered ${row.name}`);
      setSelected(row.id);
      setView('monitor');
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const evaluate = async () => {
    setError('');
    setBusy(true);
    try {
      const result = await api.evaluate(selected, {
        commitment_id: selected,
        source: `synthetic-browser-${scenario.key}`,
        points: scenario.points.map((observed, i) => ({ sequence: i + 1, observed, timestamp_ms: 1700000000000 + i * 60000 })),
      });
      setResults((r) => ({ ...r, [selected]: result }));
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const loadAudit = async () => {
    setBusy(true);
    try {
      setAudit(await api.audit(selected));
      setView('audit');
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const requestException = async (viewer = false) => {
    setError('');
    try {
      await api.exception(selected, { reason: 'planned maintenance', duration_minutes: 30 }, viewer ? 'local-viewer' : 'local-operator');
      setPolicy({ allowed: true, message: 'operator · receipt appended to RVF' });
      setNotice('Exception approved and appended to the audit trail');
      await loadAudit();
    } catch (e) {
      setPolicy({ allowed: false, message: 'viewer · nothing appended' });
      setError(String(e));
    }
  };

  return (
    <>
      <HorizonBackdrop risk={results[selected]?.risk_band ?? 'idle'} />
      {busy && (
        <div className="fixed inset-x-0 top-0 z-50 h-0.5 overflow-hidden" aria-label="Loading" role="progressbar">
          <div className="animate-scan h-full w-1/3 bg-gradient-to-r from-transparent via-teal-300 to-transparent" />
        </div>
      )}
      <main className="mx-auto max-w-7xl px-4 pb-24 sm:px-6">
        <Header view={view} onNavigate={setView} onAudit={loadAudit} canAudit={!!selected} online={online} />
        {view === 'monitor' && (
          <Monitor
            items={items}
            selected={selected}
            onSelect={setSelected}
            results={results}
            scenario={scenario}
            onScenario={setScenario}
            onEvaluate={evaluate}
            onException={requestException}
            onRegister={() => setView('register')}
            busy={busy}
            loaded={loaded}
            policy={policy}
          />
        )}
        {view === 'register' && <RegisterForm onSubmit={submit} busy={busy} />}
        {view === 'audit' && <AuditTrail records={audit} />}
      </main>
      <Toasts
        notice={notice}
        error={error}
        onDismiss={() => {
          setNotice('');
          setError('');
        }}
      />
    </>
  );
}
