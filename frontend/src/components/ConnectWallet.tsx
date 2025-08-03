"use client";

import { Wallet } from "lucide-react";

export function ConnectWallet() {
  // This will be integrated with RainbowKit later
  return (
    <button className="flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
      <Wallet className="h-4 w-4" />
      <span>Connect Wallet</span>
    </button>
  );
}