"use client";

import { Coins } from "lucide-react";
import { ConnectWallet } from "./ConnectWallet";

export function Header() {
  return (
    <header className="border-b">
      <div className="container mx-auto px-4 h-16 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <Coins className="h-6 w-6 text-blue-600" />
          <h1 className="text-xl font-bold">Crypto Staking Platform</h1>
        </div>
        <ConnectWallet />
      </div>
    </header>
  );
}