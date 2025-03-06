import { StrictMode, useEffect } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import Event from './event/Event.tsx'
import { BrowserRouter, createRoutesFromChildren, matchRoutes, Route, Routes, useLocation, useNavigationType } from 'react-router';
import "./i18next.tsx";
import * as Sentry from "@sentry/react";
import { AuthProvider } from "react-oidc-context";
import { createTheme, ThemeProvider, CssBaseline } from '@mui/material';

const cognitoAuthConfig = {
  authority: "https://cognito-idp.eu-north-1.amazonaws.com/eu-north-1_1i9kY1Vzh",
  client_id: "6o33hc2kve1cli1nffqpf1f43e",
  redirect_uri: "http://localhost:5173",
  response_type: "code",
  scope: "email openid profile",
}

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

const theme = createTheme({
  colorSchemes: {
    dark: true
  }
})

const SentryRoutes = Sentry.withSentryReactRouterV7Routing(Routes);
root.render(
  <StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <AuthProvider {...cognitoAuthConfig}>
        <BrowserRouter>
          <SentryRoutes>
            <Route index element={<App />} />
            <Route path="event/:id" element={<Event />} />
          </SentryRoutes>
        </BrowserRouter>
      </AuthProvider>
    </ThemeProvider>
  </StrictMode>,
)
