export function Toasts({ notice, error, onDismiss }: { notice: string; error: string; onDismiss: () => void }) {
  if (!notice && !error) return null;
  return (
    <div className="fixed right-4 bottom-4 left-4 z-50 flex flex-col items-end gap-2 sm:left-auto sm:w-96">
      {notice && (
        <div role="status" className="glass animate-rise flex w-full items-start gap-3 !rounded-2xl border-emerald-400/30 p-4 text-sm text-emerald-200">
          <span className="mt-1 size-2 shrink-0 rounded-full bg-emerald-300 shadow-[0_0_10px_#6ee7b7]" />
          <span className="flex-1">{notice}</span>
          <button onClick={onDismiss} aria-label="Dismiss" className="cursor-pointer text-slate-500 hover:text-white">×</button>
        </div>
      )}
      {error && (
        <div role="alert" className="glass animate-rise flex w-full items-start gap-3 !rounded-2xl border-rose-400/40 p-4 text-sm text-rose-200">
          <span className="mt-1 size-2 shrink-0 rounded-full bg-rose-400 shadow-[0_0_10px_#fb7185]" />
          <span className="flex-1">{error}</span>
          <button onClick={onDismiss} aria-label="Dismiss" className="cursor-pointer text-slate-500 hover:text-white">×</button>
        </div>
      )}
    </div>
  );
}
