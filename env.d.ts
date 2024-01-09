/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_API_URL: string
  readonly VITE_APP_WEBSOCKET_URL: string
  readonly VITE_APP_FILES_DOMAIN: string
  readonly VITE_EMPIRE_SESSION_ID: string
  readonly VITE_APP_GIT_SHOW: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}