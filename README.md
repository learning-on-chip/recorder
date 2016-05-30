# Recorder [![Status][status-img]][status-url]

Recorder is a tool for recording workload patterns.

## [Documentation][doc]

## Usage

```
$ recorder dynamic --help
Usage: recorder dynamic [options]

Options:
    --queue <name>           Queue for distributing jobs (required).
    --caching                Enable caching of McPAT optimization results.
    --server <host>:<port>   Redis server [default: 127.0.0.0:6379].

    --database <path>        SQLite database (required).
    --table <name>           Table for storing results (required).

    --help                   Display this message.
```

```
$ recorder static --help
Usage: recorder static [options]

Options:
    --config <path>          McPAT configuration file (required).

    --database <path>        SQLite database (required).
    --table <name>           Table for storing results (required).

    --caching                Enable caching of McPAT optimization results.
    --server <host>:<port>   Redis server [default: 127.0.0.0:6379].

    --help                   Display this message.
```

## Contribution

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[doc]: https://learning-on-chip.github.io/recorder
[status-img]: https://travis-ci.org/learning-on-chip/recorder.svg?branch=master
[status-url]: https://travis-ci.org/learning-on-chip/recorder
