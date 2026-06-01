# Termux Battery HTTP Server

A minimal HTTP server for Termux that exposes Android battery status as a JSON response using `termux-battery-status`.

## Overview

This project provides a very simple HTTP server that returns the current battery information from an Android device running Termux. It can be used for local monitoring, automation, dashboards, or integration with other tools.

## Features

- Exposes battery status over HTTP
- Uses the Termux `termux-battery-status` command
- Returns battery data as JSON
- Simple and lightweight
- Useful for local scripts and monitoring tools

## Requirements

- Android device
- Termux
- Termux:API app
- `termux-api` package installed in Termux

Install the required package:

```bash
pkg install termux-api
```

Make sure the Termux:API app is installed on your Android device.

## Usage

Start the server:

```bash
./phone-status-server
```

Then open the endpoint in your browser or use `curl`:

```bash
curl http://localhost:8080
```

Example response:

```json
{
  "health": "GOOD",
  "percentage": 87,
  "plugged": "PLUGGED_AC",
  "status": "CHARGING",
  "temperature": 31.2,
  "current": 120000
}
```

The exact response fields depend on the output returned by `termux-battery-status`.

## Example Use Cases

- Check battery level from another local script
- Integrate Android battery status with a dashboard
- Monitor a Termux device used as a small server
- Automate actions based on charging status or battery percentage

## Notes

This server is intended to be simple and lightweight. By default, it should be used in a trusted local network or only on localhost.

If you expose it outside your device, consider adding authentication or limiting network access.

## License

MIT
