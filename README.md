# Night.fall 

A private, offline-first wallet for Zcash that keeps your data local and your transactions shielded.

## What's Nightfall?

Nightfall is a secure wallet that stores everything on your device - no cloud, no tracking . It supports both unified and transparent addresses, automatically shielding your funds when you make payments. As an added feature, you can swap on NEAR right from the wallet.

Multiple users? No problem. Everyone can have their own wallet on the same device without seeing each other's stuff.

> **NOTE:** As at time of writing Nightfall is currently on testnet only.

## Getting Started

### What You'll Need

- **Rust** (v1.90.0)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Node.js** (v22.0.0)
  Download from [nodejs.org](https://nodejs.org/en/download)

### Running Locally

**Backend:**
```bash
git clone https://github.com/Mofe-Bankole/nightfall
cd core
cargo run
```

**Frontend:**
```bash
git clone https://github.com/Mofe-Bankole/nightfall
cd web
npm i && npm run dev
```

Open up `localhost:3000` in your browser and you're good to go.

*Note: You'll need internet to broadcast transactions, but everything else works offline.*

## Security

Nightfall keeps things tight with a two-part setup:
- **Backend:** Rust handles all the sensitive stuff
- **Frontend:** Next.js for the UI

Your private keys and unified viewing keys never touch the frontend. Everything's stored in an encrypted database on your disk.

Stay safe out there. 🔐
