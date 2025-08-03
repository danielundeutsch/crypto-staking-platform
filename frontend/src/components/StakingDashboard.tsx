"use client";

import { StakingCard } from "./StakingCard";
import { useEffect, useState } from "react";
import { stakingAPI, Service } from "@/lib/api";

const SUPPORTED_CHAINS = [
  { name: "ethereum", apr: 5.2, minStake: "0.1" },
  { name: "bitcoin", apr: 4.8, minStake: "0.001" },
  { name: "solana", apr: 6.5, minStake: "10" },
  { name: "polygon", apr: 5.8, minStake: "100" },
  { name: "avalanche", apr: 6.0, minStake: "1" },
  { name: "cardano", apr: 4.5, minStake: "100" },
];

export function StakingDashboard() {
  const [services, setServices] = useState<Service[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchServices();
  }, []);

  const fetchServices = async () => {
    try {
      const data = await stakingAPI.getServices();
      setServices(data);
    } catch (error) {
      console.error("Failed to fetch services:", error);
    } finally {
      setLoading(false);
    }
  };

  const isChainAvailable = (chainName: string) => {
    return services.some(s => s.chain === chainName && s.status === 'healthy');
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[...Array(6)].map((_, i) => (
            <div key={i} className="border rounded-lg p-6 animate-pulse">
              <div className="h-6 bg-gray-200 rounded w-1/2 mb-4"></div>
              <div className="space-y-3">
                <div className="h-4 bg-gray-200 rounded"></div>
                <div className="h-4 bg-gray-200 rounded"></div>
                <div className="h-10 bg-gray-200 rounded"></div>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h2 className="text-2xl font-bold mb-2">Available Staking Options</h2>
        <p className="text-gray-600">
          Choose a blockchain network to stake your tokens and earn rewards
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {SUPPORTED_CHAINS.map((chain) => (
          <StakingCard
            key={chain.name}
            chain={chain.name}
            available={isChainAvailable(chain.name)}
            apr={chain.apr}
            minStake={chain.minStake}
          />
        ))}
      </div>
    </div>
  );
}