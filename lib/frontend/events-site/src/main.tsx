import { StrictMode, useEffect } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import "@flaticon/flaticon-uicons/css/all/all.css"
import App from './App.tsx'
import Event from './event/Event.tsx'
import { BrowserRouter, createRoutesFromChildren, matchRoutes, Route, Routes, useLocation, useNavigationType } from 'react-router';
import "./i18next.tsx";
import * as Sentry from "@sentry/react";

Sentry.init({
  dsn: import.meta.env.VITE_SENTRY_DSN,
  integrations: [
    Sentry.httpContextIntegration(),
    Sentry.reactRouterV7BrowserTracingIntegration({
    useEffect,
    useLocation,
    useNavigationType,
    createRoutesFromChildren,
    matchRoutes
  }), Sentry.httpClientIntegration()]
})

const root = createRoot(document.getElementById('root')!, {
  onUncaughtError: Sentry.reactErrorHandler((error, errorInfo) => {
    console.warn("Uncaught error", error, errorInfo.componentStack)
  }),
  onCaughtError: Sentry.reactErrorHandler(),
  onRecoverableError: Sentry.reactErrorHandler(),
});

const SentryRoutes = Sentry.withSentryReactRouterV7Routing(Routes);
root.render(
  <StrictMode>
      <BrowserRouter>
        <SentryRoutes>
          <Route index element={<App />} />
          <Route path="event/:id" element={<Event />} />
        </SentryRoutes>
      </BrowserRouter>
  </StrictMode>,
)
