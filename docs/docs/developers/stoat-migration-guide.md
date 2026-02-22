# Voxly Migration Guide

:::warning

This is not yet finished.

:::

## Endpoint changes

| Service    | Old URL                             | New URL                                 |
| ---------- | ----------------------------------- | --------------------------------------- |
| **API**    | `https://api.voxly.gg`           | `https://voxly.gg/api`                |
|            | `https://app.voxly.gg/api`       | `https://voxly.gg/api`                |
|            | `https://voxly.gg/api`           | No equivalent                           |
| **Events** | `wss://ws.voxly.gg`              | `wss://voxly.gg/events`               |
|            | `wss://app.voxly.gg/events`      | `wss://voxly.gg/events`               |
|            | `wss://voxly.gg/events`          | No equivalent                           |
| **Files**  | `https://autumn.voxly.gg`        | `https://cdn.stoatusercontent.com`      |
|            | `https://cdn.revoltusercontent.com` | `https://cdn.stoatusercontent.com`      |
| **Proxy**  | `https://jan.voxly.gg`           | `https://external.stoatusercontent.com` |
| **Voice**  | `https://vortex.voxly.gg`        | Superseded by Voice Chats v2            |
