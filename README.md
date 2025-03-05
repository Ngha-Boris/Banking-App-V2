# Bank App

A simple banking application built with Rust, using Actix-Web for the API and Supabase as the database with a built-in UI. This app allows users to top up accounts, withdraw money, transfer funds between accounts, and integrate with MTN/Orange mobile money services.

## Features
- **Top-Up Account**: Add funds to your bank account.
- **Withdraw Money**: Remove funds from your bank account.
- **Internal Transfer**: Send money between accounts within the system.
- **Mobile Money Withdrawal**: Transfer funds to MTN or Orange mobile money accounts.
- **Mobile Money Top-Up**: Add funds from MTN or Orange mobile money to your bank account.

## Tech Stack
- **Language**: Rust
- **Web Framework**: Actix-Web
- **Database**: Supabase (PostgreSQL with UI)
- **CI/CD**: GitHub Actions

## Project Status
As of March 05, 2025, the project is in early development with basic setup and a working top-up endpoint.

## Prerequisites
- **Rust**: Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Supabase Account**: Sign up at [supabase.com](https://supabase.com)
- **Git**: For version control

## Setup Instructions

### 1. Clone the Repository
```bash
git git@github.com:Ngha-Boris/Banking-App-V2.git

cd Banking-App-V2
```
### Run App
```bash
cargo run
