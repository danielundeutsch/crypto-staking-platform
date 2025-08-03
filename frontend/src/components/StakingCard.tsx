"use client";

import { useState } from "react";
import { Coins, TrendingUp, Lock } from "lucide-react";
import { cn } from "@/lib/utils";

interface StakingCardProps {
  chain: string;
  available?: boolean;
  apr?: number;
  minStake?: string;
}

export function StakingCard({ chain, available = true, apr = 5.0, minStake = "10" }: StakingCardProps) {
  const [amount, setAmount] = useState("");
  const [duration, setDuration] = useState(30);

  const handleStake = () => {
    console.log(`Staking ${amount} on ${chain} for ${duration} days`);
    // Will integrate with API later
  };

  return (
    <div className={cn(
      "border rounded-lg p-6 space-y-4",
      available ? "border-gray-200 hover:border-blue-400" : "border-gray-100 opacity-60"
    )}>
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold capitalize">{chain}</h3>
        <span className={cn(
          "text-xs px-2 py-1 rounded",
          available ? "bg-green-100 text-green-700" : "bg-gray-100 text-gray-500"
        )}>
          {available ? "Available" : "Coming Soon"}
        </span>
      </div>

      {available && (
        <>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div className="flex items-center space-x-2">
              <TrendingUp className="h-4 w-4 text-green-600" />
              <span>APR: {apr}%</span>
            </div>
            <div className="flex items-center space-x-2">
              <Coins className="h-4 w-4 text-blue-600" />
              <span>Min: {minStake}</span>
            </div>
          </div>

          <div className="space-y-3">
            <div>
              <label className="text-sm text-gray-600">Amount to stake</label>
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0.0"
                className="w-full mt-1 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label className="text-sm text-gray-600">Lock period</label>
              <select
                value={duration}
                onChange={(e) => setDuration(Number(e.target.value))}
                className="w-full mt-1 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value={7}>7 days</option>
                <option value={14}>14 days</option>
                <option value={30}>30 days</option>
                <option value={90}>90 days</option>
                <option value={180}>180 days</option>
                <option value={365}>365 days</option>
              </select>
            </div>

            <button
              onClick={handleStake}
              disabled={!amount || parseFloat(amount) < parseFloat(minStake)}
              className="w-full py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
            >
              <div className="flex items-center justify-center space-x-2">
                <Lock className="h-4 w-4" />
                <span>Stake {chain}</span>
              </div>
            </button>
          </div>
        </>
      )}
    </div>
  );
}