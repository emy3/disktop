# disktop
cli disk viewer

```bash
main.rs → entry point, sets up terminal, event loop

crossterm.rs / termion.rs / termwiz.rs → backend adapters

ui.rs → handles layout, widgets, tables, gauges

app.rs → holds app state (like a model: disk usage, CPU stats, etc.)
```
