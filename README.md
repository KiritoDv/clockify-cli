# Welcome to Clockify CLI!

Clockify CLI is a command line interface for Clockify. It allows you to manage your time entries and projects from the command line.

## Quick Start

You need a few things to get started:
* Clockify API key (see [here](https://clockify.me/user/settings) how to get one)

To use the cli, you need to set your <ins>API key</ins> on the CLI. You can do this by running the following command:

    clockify config login <API key>

With this, you can start using the CLI. You can get a list of all commands by running:

    clockify --help

## Usage

### Time Entries

You can get the list of commands for time entries by running:

    clockify task --help

To start a new time entry, run:

    clockify task add
