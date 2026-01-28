# BulkRunner.rs

A CLI tool to execute Blue Prism AutomateC processes in bulk, dispatching to multiple resources concurrently.

## Problem It Solves

During shift changes or maintenance windows, Blue Prism administrators need to start processes on multiple bots simultaneously.
Manually dispatching through the Control Room is time-consuming and error-prone. BulkRunner.rs automates this by:

- Querying available bots from SQL Server
- Filtering to only ready/idle resources
- Dispatching processes to multiple bots concurrently
- Limiting concurrent dispatches to prevent system overload

## Features

- **Concurrent Dispatch**: Run processes on up to 30 bots simultaneously (configurable)
- **SQL-Based Bot Selection**: Query bots directly from your Blue Prism database
- **Configurable Limits**: Control both total bots and concurrent dispatches
- **Status Filtering**: Only dispatches to ready bots (Idle, Logged Out)
- **SSO Authentication**: Uses Windows SSO for AutomateC calls

## Installation

### Prerequisites

- **Rust 1.82+** - Install from [rustup.rs](https://rustup.rs)
- **Blue Prism AutomateC** - Located at `C:\Program Files\Blue Prism Limited\Blue Prism Automate\AutomateC.exe`
- **SQL Server Access** - Windows Authentication to your Blue Prism database

### Build from Source

```bash
git clone https://github.com/your-org/bulk_runner.rs.git
cd bulk_runner.rs
cargo build --release
```

The binary will be at `target/release/bulk_runner_rs.exe`.

## Usage

```bash
bulk_runner_rs <PROCESS> [OPTIONS]
```

### CLI Flags

| Flag                     | Short | Default    | Description                                |
| ------------------------ | ----- | ---------- | ------------------------------------------ |
| `<PROCESS>`              | -     | (required) | The Blue Prism process name to run         |
| `--concurrency_limit`    | `-c`  | 30         | Max concurrent bot dispatches              |
| `--limit_total_runnable` | `-l`  | 30         | Total bots to dispatch                     |
| `--file`                 | `-f`  | bots.sql   | Path to SQL query file                     |
| `--verbosity`            | `-v`  | INFO       | Log level: ERROR, WARN, INFO, DEBUG, TRACE |
| `--span`                 | `-s`  | NONE       | Span logging: NONE, EXIT, ENTER, FULL      |

### Examples

**Basic usage** - Run "Morning Startup" on up to 30 bots:

```bash
bulk_runner_rs "Morning Startup"
```

**Limit to 10 concurrent dispatches** - Reduces load on AutomateC:

```bash
bulk_runner_rs "Data Extract" -c 10 -l 50
```

**Use custom SQL file with debug logging**:

```bash
bulk_runner_rs "EOD Process" -f custom_bots.sql -v DEBUG
```

**Full tracing for troubleshooting**:

```bash
bulk_runner_rs "Test Process" -v TRACE -s FULL
```

## SQL File Format

Create a SQL file (default: `bots.sql`) that returns bot names and statuses:

### Expected Columns

| Column   | Type    | Description                                                |
| -------- | ------- | ---------------------------------------------------------- |
| `name`   | VARCHAR | The Blue Prism resource name                               |
| `status` | VARCHAR | Resource status (IDLE, PENDING, LOGGED OUT, OFFLINE, etc.) |

### Example SQL Query

```sql
SELECT TOP (@P1)
    r.name,
    r.statustext AS status
FROM BPAResource r
WHERE r.statustext IN ('Idle', 'Logged Out')
  AND r.processesaliases IS NOT NULL
ORDER BY r.name
```

The `@P1` bind parameter is automatically set to `--limit_total_runnable` value.

### Valid Status Values

| Status      | Dispatchable |
| ----------- | ------------ |
| IDLE        | ✅ Yes       |
| LOGGED OUT  | ✅ Yes       |
| PENDING     | ❌ No        |
| OFFLINE     | ❌ No        |
| PRIVATE     | ❌ No        |
| UNAVAILABLE | ❌ No        |

## Environment Variables

| Variable                 | Description                                                             |
| ------------------------ | ----------------------------------------------------------------------- |
| `BYPASS_AUTOMATEC_CHECK` | Set to any value to skip AutomateC existence check (useful for testing) |

Database connection uses Windows Authentication via the `deadpool-tiberius` driver. Configure your SQL Server connection via standard Windows credential delegation.

## License

See [LICENSE](LICENSE) for details.
