import type { ButtonHTMLAttributes, ReactNode } from 'react';

const variants = {
  primary:
    'bg-gradient-to-r from-teal-300 to-cyan-300 text-slate-950 shadow-[0_0_30px_-6px_rgb(94_234_212/0.7)] hover:shadow-[0_0_40px_-4px_rgb(94_234_212/0.9)] hover:brightness-110',
  ghost: 'border border-white/10 bg-white/[0.03] text-slate-200 hover:border-teal-300/50 hover:bg-white/[0.07]',
  danger: 'border border-rose-400/30 bg-rose-500/10 text-rose-200 hover:border-rose-300/60 hover:bg-rose-500/20',
};

export function Button({
  variant = 'ghost',
  className = '',
  ...rest
}: ButtonHTMLAttributes<HTMLButtonElement> & { variant?: keyof typeof variants }) {
  return (
    <button
      className={`inline-flex cursor-pointer items-center justify-center gap-2 rounded-xl px-4 py-2.5 text-sm font-semibold transition duration-200 disabled:cursor-not-allowed disabled:opacity-40 disabled:shadow-none ${variants[variant]} ${className}`}
      {...rest}
    />
  );
}

export function Panel({ title, kicker, children, className = '', actions }: {
  title?: string;
  kicker?: string;
  children: ReactNode;
  className?: string;
  actions?: ReactNode;
}) {
  return (
    <section className={`glass animate-rise p-5 sm:p-6 ${className}`}>
      {(title || actions) && (
        <header className="mb-4 flex items-start justify-between gap-4">
          <div>
            {kicker && <p className="font-mono text-[0.68rem] tracking-[0.2em] text-teal-300/80 uppercase">{kicker}</p>}
            {title && <h2 className="mt-1 text-lg font-semibold tracking-tight text-white">{title}</h2>}
          </div>
          {actions}
        </header>
      )}
      {children}
    </section>
  );
}

export function Stat({ label, value, sub, accent }: { label: string; value: string; sub?: string; accent?: string }) {
  return (
    <div className="rounded-2xl border border-white/[0.07] bg-white/[0.025] p-4">
      <p className="font-mono text-[0.68rem] tracking-[0.16em] text-slate-400 uppercase">{label}</p>
      <p className="mt-2 text-3xl font-semibold tracking-tight tabular-nums" style={accent ? { color: accent } : undefined}>
        {value}
      </p>
      {sub && <p className="mt-1 text-xs text-slate-400">{sub}</p>}
    </div>
  );
}

export function Hash({ value }: { value: string }) {
  return (
    <code title={value} className="rounded-md bg-white/5 px-1.5 py-0.5 font-mono text-[0.72rem] text-slate-400">
      {value.length > 14 ? `${value.slice(0, 14)}…` : value}
    </code>
  );
}
