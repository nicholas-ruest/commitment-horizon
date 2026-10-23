import type { Evaluation } from './client';

export type RiskBand = Evaluation['risk_band'];

export const riskPalette: Record<RiskBand | 'idle', { hex: number; css: string; label: string }> = {
  idle: { hex: 0x5eead4, css: '#5eead4', label: 'Awaiting evidence' },
  healthy: { hex: 0x34d399, css: '#34d399', label: 'Healthy' },
  watch: { hex: 0xfbbf24, css: '#fbbf24', label: 'Watch' },
  at_risk: { hex: 0xfb923c, css: '#fb923c', label: 'At risk' },
  breached: { hex: 0xf43f5e, css: '#f43f5e', label: 'Breached' },
};

export type Scenario = { key: string; label: string; hint: string; points: number[] };

// Deterministic synthetic evidence; "drift" is the original browser journey.
export const scenarios: Scenario[] = [
  { key: 'stable', label: 'Stable', hint: 'Holding above target', points: [0.995, 0.991, 0.993, 0.989, 0.992, 0.99] },
  { key: 'drift', label: 'Drifting', hint: 'Gradual erosion', points: [0.99, 0.96, 0.88, 0.81] },
  { key: 'collapse', label: 'Collapse', hint: 'Sharp incident', points: [0.98, 0.97, 0.72, 0.55, 0.41] },
];

export const pct = (value: number, digits = 1) => `${(value * 100).toFixed(digits)}%`;
export const shortHash = (hash: string, n = 10) => (hash.length > n ? `${hash.slice(0, n)}…` : hash);
