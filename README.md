A template project to try to integrate Tauri V2 with dotnet backend service using sidecar

Main architecture:

- Tauri V2 with Vite SPA for frontend service
- ASPNETCORE dotnet 9.0 service for backend service

Goals:

- Single desktop application with the backend service included 
- Work out of the box without the need for user to configure the lifecycle of frontend and backend
- Self-update feature which should reflect updates from both frontend code and backend code
- Update binaries should be small in size and automatically versioned with new releases on github
- Supports cross-platform on both linux and windows machines
- Data persistence which survives uninstall / install
- No external dependencies needed (dotnet runtime / webview2 / webkitgtk etc.)

Good to haves:

- Small distribution size (dotnet service needs to be AOT compiled / trimmings)
- Web UI serving the same exact frontend as the tauri application (with limitations such as windows controls etc.)