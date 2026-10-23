import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
// Dev proxies /api to the local API so the app works through one forwarded port (e.g. GitHub Codespaces).
export default defineConfig({plugins:[react(),tailwindcss()],server:{port:5173,allowedHosts:['.app.github.dev'],proxy:{'/api':'http://127.0.0.1:8080'}},preview:{host:'127.0.0.1',port:4173}});
